use maud::{html, PreEscaped};
use time::format_description::well_known::Rfc3339;

use super::Feed;

pub fn render(feed: &Feed) -> String {
    html! {
        (PreEscaped("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"))
        feed xmlns="http://www.w3.org/2005/Atom" {
            title {
                (feed.title)
            }
            link {
                (feed.feed_url)
            }
            (PreEscaped(format!("<link href=\"{}\" rel=\"self\"/>", feed.feed_url)))
            subtitle {
                (feed.description)
            }
            @if let Some(date) = feed.last_updated() {
                updated {
                    (date.format(&Rfc3339).unwrap())
                }
            }

            @for item in &feed.items {
                entry {
                    id {
                        (item.id)
                    }
                    title {
                        (item.title)
                    }
                    (PreEscaped(format!("<link href=\"{}\"/>", item.url)))
                    updated {
                        (item.date_published.format(&Rfc3339).unwrap())
                    }
                    @for author in &item.authors {
                        author {
                            name {
                                (author.name)
                            }
                            @if let Some(url) = author.url.as_ref() {
                                url {
                                    (url)
                                }
                            }
                        }
                    }
                    @if let Some(tags) = item.tags.as_ref() {
                        @for tag in tags {
                            (PreEscaped(format!("<category term=\"{tag}\"/>")))
                        }
                    }
                    content type="html" {
                        (item.content_html)
                    }
                }
            }
        }
    }.into_string()
}
