use harsh::Harsh;
use once_cell::sync::Lazy;
use serde::Deserialize;
use sqlx::{mysql::MySqlRow, FromRow, MySqlPool, Row};
use std::{collections::HashMap, env, net::IpAddr};
use time::OffsetDateTime;

use crate::{csrf, url};

static HASHIDS: Lazy<Harsh> = Lazy::new(|| {
    Harsh::builder()
        .salt(env::var("HASHID_SALT").unwrap())
        .length(5)
        .build()
        .unwrap()
});

/// Maximum length allowed for a comment.
pub const MAX_TEXT_LENGTH: usize = 8192;

#[derive(Clone, Debug)]
pub struct Comment {
    /// Unique ID of this comment.
    pub id: String,

    /// If this comment is a child of another comment, the ID of the parent
    /// comment.
    pub parent_id: Option<String>,

    /// Slug of the article this comment is on.
    pub article_slug: String,

    /// Date and time when the comment was published.
    pub published: OffsetDateTime,

    /// The author of the comment.
    pub author: Author,

    /// Current comment score, sum of all positive and negative votes.
    pub score: i16,

    /// The text of the comment in Markdown format.
    pub text: String,

    /// Replies to this comment.
    pub children: Vec<Comment>,
}

impl Comment {
    pub fn len(&self) -> usize {
        self.children.iter().map(Self::len).sum::<usize>() + 1
    }

    pub fn canonical_url(&self) -> String {
        format!(
            "https://stephencoakley.com/{}#comment-{}",
            self.article_slug, self.id
        )
    }
}

impl<'r> FromRow<'r, MySqlRow> for Comment {
    fn from_row(row: &'r MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: encode_id(row.get("id")),
            parent_id: row.get::<Option<i64>, _>("parentId").map(encode_id),
            article_slug: row.get("slug"),
            published: OffsetDateTime::from_unix_timestamp(
                row.get::<f64, _>("datePublished") as i64
            )
            .unwrap(),
            author: Author {
                name: row.get("authorName"),
                email: row.get("authorEmail"),
                website: row.get("authorWebsite"),
            },
            children: Vec::new(),
            score: row.get("score"),
            text: row.get("text"),
        })
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Author {
    /// The name the comment author supplied.
    pub name: String,

    /// The email address the comment author supplied.
    pub email: Option<String>,

    /// The website the comment author supplied.
    pub website: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PostComment {
    pub parent_id: Option<String>,
    pub author_name: String,
    pub author_email: Option<String>,
    pub author_website: Option<String>,
    pub text: String,
    pub token: String,
}

#[derive(Clone, Debug)]
pub struct CommentStore {
    pool: MySqlPool,
}

impl CommentStore {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn get_newest(&self) -> Vec<Comment> {
        sqlx::query_as(
            "SELECT
                id,
                parentId,
                slug,
                datePublished,
                authorName,
                authorEmail,
                authorWebsite,
                score DIV 1 AS `score`,
                text
            FROM CommentWithScore
            WHERE dateDeleted IS NULL
            ORDER BY datePublished DESC
            LIMIT 50",
        )
        .fetch_all(&self.pool)
        .await
        .unwrap()
    }

    pub async fn get_by_id(&self, id: &str) -> Option<Comment> {
        sqlx::query_as(
            "SELECT
                id,
                parentId,
                slug,
                datePublished,
                authorName,
                authorEmail,
                authorWebsite,
                score DIV 1 AS `score`,
                text
            FROM CommentWithScore
            WHERE dateDeleted IS NULL
            AND id = ?",
        )
        .bind(decode_id(id)?)
        .fetch_optional(&self.pool)
        .await
        .unwrap()
    }

    /// Fetch the entire comment tree for the given article slug.
    pub async fn tree_for_slug(&self, article_slug: &str) -> Vec<Comment> {
        rebuild_comment_tree(self.fetch_all_comments_for_slug(article_slug).await)
    }

    pub async fn fetch_all_comments_for_slug(&self, article_slug: &str) -> Vec<Comment> {
        sqlx::query_as(
            "SELECT
                id,
                parentId,
                slug,
                datePublished,
                authorName,
                authorEmail,
                authorWebsite,
                score DIV 1 AS `score`,
                text
            FROM CommentWithScore
            WHERE slug = ?
                AND dateDeleted IS NULL
            ORDER BY datePublished ASC",
        )
        .bind(article_slug)
        .fetch_all(&self.pool)
        .await
        .unwrap()
    }

    pub async fn post(&self, article_slug: &str, mut post: PostComment) {
        log::info!("received comment: {:?}", post);

        // SPAM PREVENTION:
        // - validate form token
        // - limit length
        // - limit number of links
        // - sanitize HTML

        if !csrf::verify_token(&post.token) {
            return;
        }

        // The max length is enforced by the frontend, so if we exceed that then
        // this is probably coming from a spambot.
        if post.text.chars().count() > MAX_TEXT_LENGTH {
            return;
        }

        if url::count(&post.text) > 10 {
            return;
        }

        // Sanitize any HTML that may be present in the comment text.
        post.text = ammonia::clean(&post.text);

        // If the comment is blank then it should also be ignored.
        if post.text.trim().is_empty() {
            return;
        }

        let now = OffsetDateTime::now_utc();

        sqlx::query(
            "
        INSERT INTO Comment (
            parentId,
            slug,
            datePublished,
            authorName,
            authorEmail,
            authorWebsite,
            text
        ) VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(post.parent_id.map(decode_id))
        .bind(article_slug)
        .bind((now.unix_timestamp_nanos() as f64) / 1_000_000_000.0)
        .bind(post.author_name)
        .bind(post.author_email)
        .bind(post.author_website)
        .bind(post.text)
        .execute(&self.pool)
        .await
        .unwrap();
    }

    pub async fn upvote(&self, comment_id: &str, ip_addr: IpAddr) {
        self.vote(comment_id, ip_addr, 1).await
    }

    pub async fn downvote(&self, comment_id: &str, ip_addr: IpAddr) {
        self.vote(comment_id, ip_addr, -1).await
    }

    async fn vote(&self, comment_id: &str, ip_addr: IpAddr, value: i8) {
        // To prevent excessive spam we limit the total number of votes any one
        // comment can receive.
        const MAX_VOTES: i16 = 500;

        // Ignore multi-votes.
        if value.abs() > 1 || value == 0 {
            return;
        }

        let comment = match self.get_by_id(comment_id).await {
            Some(comment) => comment,

            // Invalid comment ID, ignore vote
            None => return,
        };

        if comment.score.abs() >= MAX_VOTES {
            log::debug!(
                "ignoring vote for comment `{comment_id}`, already exceeded {MAX_VOTES} votes"
            );
            return;
        }

        sqlx::query("REPLACE INTO Vote (commentId, voterIp, vote) VALUES (?, ?, ?)")
            .bind(decode_id(comment_id).unwrap())
            .bind(ip_addr.to_string())
            .bind(value)
            .execute(&self.pool)
            .await
            .unwrap();
    }
}

fn rebuild_comment_tree(comments: Vec<Comment>) -> Vec<Comment> {
    let mut comments_by_parent_id = HashMap::<String, Vec<Comment>>::new();
    let mut top_level_comments = Vec::new();

    // Remove comments from the list that aren't top-level and organize them by
    // the parent comment ID.
    for comment in comments {
        if let Some(id) = comment.parent_id.clone() {
            comments_by_parent_id.entry(id).or_default().push(comment);
        } else {
            top_level_comments.push(comment);
        }
    }

    fn build_subtrees(
        comments: &mut Vec<Comment>,
        comments_by_parent_id: &mut HashMap<String, Vec<Comment>>,
    ) {
        for comment in comments.iter_mut() {
            if let Some(mut children) = comments_by_parent_id.remove(&comment.id) {
                build_subtrees(&mut children, comments_by_parent_id);
                comment.children = children;
            }
        }
    }

    build_subtrees(&mut top_level_comments, &mut comments_by_parent_id);

    top_level_comments
}

fn encode_id(id: i64) -> String {
    HASHIDS.encode(&[id as u64])
}

fn decode_id(id: impl AsRef<str>) -> Option<i64> {
    HASHIDS.decode(id).ok().map(|v| v[0] as i64)
}
