use poem::{web::Json, IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::fmt;
use time::OffsetDateTime;

use crate::{articles::Article, comments::Comment};

mod atom;
mod rss;

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FeedFormat {
    Rss,
    Atom,
    Json,
}

impl Default for FeedFormat {
    fn default() -> Self {
        Self::Rss
    }
}

impl fmt::Display for FeedFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rss => f.write_str("rss"),
            Self::Atom => f.write_str("atom"),
            Self::Json => f.write_str("json"),
        }
    }
}

/// A syndicated news feed. Follows the structure as defined by the JSON Feed
/// specification.
#[derive(Clone, Debug, Serialize)]
pub struct Feed {
    version: String,
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

    /// Generate an HTTP response for this feed in the given format.
    pub fn into_response(self, format: FeedFormat) -> Response {
        match format {
            FeedFormat::Rss => rss::render(&self)
                .with_content_type("application/rss+xml; charset=utf-8")
                .into_response(),
            FeedFormat::Atom => atom::render(&self)
                .with_content_type("application/atom+xml; charset=utf-8")
                .into_response(),
            FeedFormat::Json => Json(self)
                .with_content_type("application/feed+json; charset=utf-8")
                .into_response(),
        }
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

pub fn articles<'a>(
    title: impl Into<String>,
    description: impl Into<String>,
    self_link: impl Into<String>,
    articles: impl IntoIterator<Item = &'a Article>,
) -> Feed {
    Feed {
        version: "https://jsonfeed.org/version/1.1".into(),
        title: title.into(),
        description: description.into(),
        home_page_url: "https://stephencoakley.com".into(),
        feed_url: self_link.into(),
        icon: "https://stephencoakley.com/assets/images/favicon.128.png".into(),
        favicon: "https://stephencoakley.com/assets/images/favicon.ico".into(),
        language: Some("en-US".into()),
        authors: vec![Author {
            name: "Stephen Coakley".into(),
            url: Some("https://stephencoakley.com".into()),
            avatar: Some(
                "https://s.gravatar.com/avatar/4c0d6bf3fb628cc4ccd1d1613f421290?s=512".into(),
            ),
        }],
        items: articles.into_iter().map(|article| article.into()).collect(),
    }
}

pub fn comments(
    title: impl Into<String>,
    description: impl Into<String>,
    self_link: impl Into<String>,
    comments: &[Comment],
) -> Feed {
    Feed {
        version: "https://jsonfeed.org/version/1.1".into(),
        title: title.into(),
        description: description.into(),
        home_page_url: "https://stephencoakley.com".into(),
        feed_url: self_link.into(),
        icon: "https://stephencoakley.com/assets/images/favicon.128.png".into(),
        favicon: "https://stephencoakley.com/assets/images/favicon.ico".into(),
        language: None,
        authors: vec![],
        items: comments.into_iter().map(|comment| comment.into()).collect(),
    }
}

impl<'a> From<&'a Article> for Item {
    fn from(article: &'a Article) -> Self {
        Self {
            id: article.canonical_url(),
            url: article.canonical_url(),
            title: article.title.clone(),
            date_published: article.date.midnight().assume_utc(),
            authors: vec![Author {
                name: "Stephen Coakley".into(),
                url: Some("https://stephencoakley.com".into()),
                avatar: Some(
                    "https://s.gravatar.com/avatar/4c0d6bf3fb628cc4ccd1d1613f421290?s=512".into(),
                ),
            }],
            tags: Some(article.tags.clone()),
            content_html: article.content_html.clone(),
        }
    }
}

impl<'a> From<&'a Comment> for Item {
    fn from(comment: &'a Comment) -> Self {
        Self {
            id: comment.canonical_url(),
            url: comment.canonical_url(),
            title: format!(
                "Comment on {} by {}",
                &comment.article_slug, &comment.author.name
            ),
            date_published: comment.published,
            authors: vec![Author {
                name: comment.author.name.clone(),
                url: comment.author.website.clone(),
                avatar: None,
            }],
            tags: None,
            content_html: comment.text.clone(),
        }
    }
}
