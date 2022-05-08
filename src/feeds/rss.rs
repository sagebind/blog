use maud::{html, PreEscaped};
use time::format_description::well_known::Rfc2822;

use super::Feed;

pub fn render(feed: &Feed) -> String {
    html! {
        (PreEscaped("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"))
        rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom" {
            channel {
                title {
                    (feed.title)
                }
                link {
                    (feed.feed_url)
                }
                (PreEscaped(format!("<atom:link href=\"{}\" rel=\"self\" type=\"application/rss+xml\"/>", feed.feed_url)))
                description {
                    (feed.description)
                }
                @if let Some(date) = feed.last_updated() {
                    pubDate {
                        (date.format(&Rfc2822).unwrap())
                    }
                }

                @for item in &feed.items {
                    item {
                        title {
                            (item.title)
                        }
                        link {
                            (item.url)
                        }
                        @if let Some(author) = item.authors.get(0) {
                            author {
                                (author.name)
                            }
                        }
                        guid {
                            (item.id)
                        }
                        pubDate {
                            (item.date_published.format(&Rfc2822).unwrap())
                        }
                        @if let Some(tags) = item.tags.as_ref() {
                            @for tag in tags {
                                category {
                                    (tag)
                                }
                            }
                        }
                        description {
                            (item.content_html)
                        }
                    }
                }
            }
        }
    }.into_string()
}
