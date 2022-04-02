use harsh::Harsh;
use once_cell::sync::Lazy;
use serde::Deserialize;
use sqlx::{mysql::MySqlRow, FromRow, MySqlPool, Row};
use std::{collections::HashMap, env};
use time::OffsetDateTime;

use crate::csrf;

static HASHIDS: Lazy<Harsh> = Lazy::new(|| {
    Harsh::builder()
        .salt(env::var("HASHID_SALT").unwrap())
        .length(5)
        .build()
        .unwrap()
});

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

    pub score: i16,

    pub children: Vec<Comment>,

    /// The text of the comment in Markdown format.
    pub text: String,
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

        if !csrf::verify_token(&post.token) {
            return;
        }

        // SPAM PREVENTION:
        // - limit number of links
        // - don't allow HTML
        // - validate CSRF token
        // - validate token time
        // - filter known bad IPs
        // - limit length

        // Sanitize any HTML that may be present in the comment text.
        post.text = ammonia::clean(&post.text);

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

fn decode_id(id: String) -> i64 {
    HASHIDS.decode(&id).unwrap()[0] as i64
}
