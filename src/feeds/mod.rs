use serde::Serialize;
use time::OffsetDateTime;

use crate::{comments::Comment, articles::Article};

pub mod atom;
pub mod rss;

/// A syndicated news feed. Follows the structure as defined by the JSON Feed
/// specification.
#[derive(Clone, Debug, Serialize)]
pub struct Feed {
    title: String,
    description: String,
    home_page_url: String,
    feed_url: String,
    icon: String,
    favicon: String,
    language: Option<String>,
    authors: Vec<Author>,
    items: Vec<Item>,
}

impl Feed {
    pub fn last_updated(&self) -> Option<OffsetDateTime> {
        self.items.iter().map(|item| item.date_published).max()
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Item {
    id: String,
    url: String,
    title: String,
    #[serde(with = "time::serde::rfc3339")]
    date_published: OffsetDateTime,
    authors: Vec<Author>,
    tags: Option<Vec<String>>,
    content_html: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct Author {
    name: String,
    url: Option<String>,
    avatar: Option<String>,
}

pub fn articles(articles: &[Article]) -> Feed {
    Feed {
        title: "Stephen Coakley".into(),
        description: "Latest articles from a Disciple of Christ and software engineer. I post infrequently and usually on technical topics.".into(),
        home_page_url: "https://stephencoakley.com".into(),
        feed_url: "https://stephencoakley.com/feed.json".into(),
        icon: "https://stephencoakley.com/assets/images/favicon.128.png".into(),
        favicon: "https://stephencoakley.com/assets/images/favicon.ico".into(),
        language: Some("en-US".into()),
        authors: vec![
            Author {
                name: "Stephen Coakley".into(),
                url: Some("https://stephencoakley.com".into()),
                avatar: Some("https://s.gravatar.com/avatar/4c0d6bf3fb628cc4ccd1d1613f421290?s=512".into()),
            }
        ],
        items: articles
            .into_iter()
            .map(|article| Item {
                id: article.canonical_url(),
                url: article.canonical_url(),
                title: article.title.clone(),
                date_published: article.date.midnight().assume_utc(),
                authors: vec![
                    Author {
                        name: "Stephen Coakley".into(),
                        url: Some("https://stephencoakley.com".into()),
                        avatar: Some("https://s.gravatar.com/avatar/4c0d6bf3fb628cc4ccd1d1613f421290?s=512".into()),
                    }
                ],
                tags: Some(article.tags.clone()),
                content_html: article.content_html.clone(),
            })
            .collect()
    }
}

pub fn comments(comments: &[Comment]) -> Feed {
    Feed {
        title: "".into(),
        description: "".into(),
        home_page_url: "https://stephencoakley.com".into(),
        feed_url: "https://stephencoakley.com/feed.json".into(),
        icon: "https://stephencoakley.com/assets/images/favicon.128.png".into(),
        favicon: "https://stephencoakley.com/assets/images/favicon.ico".into(),
        language: None,
        authors: vec![],
        items: comments
            .into_iter()
            .map(|comment| Item {
                id: comment.canonical_url(),
                url: comment.canonical_url(),
                title: format!("Comment on {} by {}", &comment.article_slug, &comment.author.name),
                date_published: comment.published,
                authors: vec![
                    Author {
                        name: comment.author.name.clone(),
                        url: comment.author.website.clone(),
                        avatar: None,
                    }
                ],
                tags: None,
                content_html: comment.text.clone(),
            })
            .collect(),
    }
}
