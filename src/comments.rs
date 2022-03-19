use std::env;

use harsh::Harsh;
use mysql::{prelude::Queryable, Opts, Pool};
use serde::Deserialize;
use time::OffsetDateTime;

#[derive(Debug)]
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
    /// Human-readable description of the publish date.
    pub fn published_label(&self) -> String {
        todo!()
    }

    pub fn canonical_url(&self) -> String {
        format!(
            "https://stephencoakley.com/{}#comment-{}",
            self.article_slug, self.id
        )
    }
}

#[derive(Debug, Deserialize)]
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
    pub fn new() -> Self {
        let opts = Opts::from_url(&env::var("DATABASE_URL").unwrap()).unwrap();
        let pool = Pool::new(opts).unwrap();

        Self {
            pool,
            hashids: Harsh::builder()
                .salt(env::var("HASHID_SALT").unwrap())
                .length(5).build()
                .unwrap(),
        }
    }

    /// Fetch the entire comment tree for the given article slug.
    pub fn tree_for_slug(&self, article_slug: &str) -> Vec<Comment> {
        let mut conn = self.pool.get_conn().unwrap();

        conn.exec_map(
            "SELECT
                    id,
                    parentId,
                    slug,
                    authorName,
                    authorEmail,
                    authorWebsite,
                    score,
                    text
                FROM CommentWithScore
                WHERE slug = ?
                    AND dateDeleted IS NULL",
            (article_slug,),
            |(id, parent_id, article_slug, author_name, author_email, author_website, score, text)| {
                let parent_id: Option<u64> = parent_id;

                Comment {
                    id: self.hashids.encode(&[id]),
                    parent_id: parent_id.map(|id| self.hashids.encode(&[id])),
                    article_slug,
                    published: OffsetDateTime::now_utc(), // todo,
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
        .unwrap()
    }
}
