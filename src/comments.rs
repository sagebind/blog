use std::{collections::HashMap, env};

use harsh::Harsh;
use mysql::{prelude::Queryable, Pool};
use serde::Deserialize;
use time::OffsetDateTime;

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

#[derive(Clone, Debug, Deserialize)]
pub struct Author {
    /// The name the comment author supplied.
    pub name: String,

    /// The email address the comment author supplied.
    pub email: Option<String>,

    /// The website the comment author supplied.
    pub website: Option<String>,
}

pub struct CommentStore {
    pool: Pool,
    hashids: Harsh,
}

impl CommentStore {
    pub fn new(pool: Pool) -> Self {
        Self {
            pool,
            hashids: Harsh::builder()
                .salt(env::var("HASHID_SALT").unwrap())
                .length(5)
                .build()
                .unwrap(),
        }
    }

    /// Fetch the entire comment tree for the given article slug.
    pub fn tree_for_slug(&self, article_slug: &str) -> Vec<Comment> {
        let mut conn = self.pool.get_conn().unwrap();

        let comments = conn
            .exec_map(
                "SELECT
                    id,
                    parentId,
                    slug,
                    datePublished,
                    authorName,
                    authorEmail,
                    authorWebsite,
                    score,
                    text
                FROM CommentWithScore
                WHERE slug = ?
                    AND dateDeleted IS NULL",
                (article_slug,),
                |(
                    id,
                    parent_id,
                    article_slug,
                    date_published,
                    author_name,
                    author_email,
                    author_website,
                    score,
                    text,
                )| {
                    let parent_id: Option<u64> = parent_id;
                    let date_published: f64 = date_published;

                    Comment {
                        id: self.hashids.encode(&[id]),
                        parent_id: parent_id.map(|id| self.hashids.encode(&[id])),
                        article_slug,
                        published: OffsetDateTime::from_unix_timestamp(date_published as i64)
                            .unwrap(),
                        author: Author {
                            name: author_name,
                            email: author_email,
                            website: author_website,
                        },
                        children: Vec::new(),
                        score,
                        text,
                    }
                },
            )
            .unwrap();

        rebuild_comment_tree(comments)
    }

    pub fn submit(&self, text: String) {
        // SPAM PREVENTION:
        // - limit number of links
        // - don't allow HTML
        // - validate CSRF token
        // - validate token time
        // - filter known bad IPs
        // - limit length
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
