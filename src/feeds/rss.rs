use maud::{html, PreEscaped};
use time::{format_description, OffsetDateTime};

use super::Feed;

pub fn to_rss(feed: Feed) -> String {
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
                (PreEscaped("<atom:link href=\"@Model.SelfLink\" rel=\"self\" type=\"application/rss+xml\"/>"))
                description {
                    (feed.description)
                }
                @if let Some(date) = feed.last_updated() {
                    pubDate {
                        (format_rfc822(date))
                    }
                }

                @for item in feed.items {
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
                            (format_rfc822(item.date_published))
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

fn format_rfc822(datetime: OffsetDateTime) -> String {
    datetime.format(&format_description::parse("[weekday repr:short], [day padding:zero] [month repr:short] [year] [hour]:[minute]:[second] +0000").unwrap()).unwrap()
}
