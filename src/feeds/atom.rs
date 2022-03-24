use maud::{html, PreEscaped};
use time::format_description::well_known::Rfc3339;

use super::Feed;

pub fn to_atom(feed: Feed) -> String {
    html! {
        (PreEscaped("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"))
        feed xmlns="http://www.w3.org/2005/Atom" {
            title {
                (feed.title)
            }
            link {
                (feed.feed_url)
            }
            (PreEscaped("<atom:link href=\"@Model.SelfLink\" rel=\"self\" type=\"application/rss+xml\"/>"))
            subtitle {
                (feed.description)
            }
            @if let Some(date) = feed.last_updated() {
                updated {
                    (date.format(&Rfc3339).unwrap())
                }
            }

            @for item in feed.items {
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
                    @for author in item.authors {
                        author {
                            name {
                                (author.name)
                            }
                            @if let Some(url) = author.url {
                                url {
                                    (url)
                                }
                            }
                        }
                    }
                    @if let Some(tags) = item.tags {
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
