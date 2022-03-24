use maud::{html, Markup, DOCTYPE, PreEscaped};
use time::OffsetDateTime;

use crate::{
    articles::{self, Article},
    components::{date, markdown::markdown, comments::comments_section}, comments::Comment,
};

pub fn home() -> Markup {
    layout(
        "Stephen Coakley",
        html! {
            p {
                big { "Hello there!" } " I'm a full-time software engineer that is passionate about web and system "
                "software, efficiency, and open-source. Occasionally I post articles here that range from short thoughts "
                "on life to results of some technical research or discovery I've done."
            }

            h1 { "Latest Articles" }

            @for article in articles::get_all(false).into_iter().take(4) {
                (article_summary(&article))
            }

            a class="button" href="/articles" { "See more articles" }
        },
    )
}

pub fn articles() -> Markup {
    layout(
        "Articles - Stephen Coakley",
        html! {
            h1 { "Articles" }

            @for article in articles::get_all(false) {
                (article_summary(&article))
            }
        },
    )
}

pub fn article(article: &Article, comments: &[Comment]) -> Markup {
    layout(
        "Articles - Stephen Coakley",
        html! {
            article {
                h1 { (&article.title) }

                p class="postmeta" {
                    (date(article.date))

                    span class="author-by" { " by " }
                    span class="author" {
                        a href="/about" { (&article.author) }
                    }

                    @for tag in &article.tags {
                        a class="tag" href={"/tag/" (tag)} { (tag) }
                    }

                    br;

                    span { (article.estimated_reading_time()) " min read" }
                }

                (PreEscaped(&article.content_html))
            }

            hr;

            aside id="bio" {
                div class="bio-image" {
                    a href="/about" {
                        img src="https://s.gravatar.com/avatar/4c0d6bf3fb628cc4ccd1d1613f421290?s=48";
                    }
                }

                div {
                    p {
                        "Hi! I'm "
                        a href="/about" {
                            strong { "Stephen Coakley" }
                        }
                        ", a software engineer and committed Christian. I am passionate about faith, life, systems and "
                        "web software, gaming, and music."
                    }
                    p {
                        "Occasionally I write articles here about those things, mostly focused on web development or "
                        "low-level programming."
                    }
                }
            }

            (comments_section(&article.slug, comments))
        },
    )
}

pub fn tag(tag: &str) -> Markup {
    let title = format!("Articles  tagged \"{}\"", tag);

    layout(
        &title,
        html! {
            h1 { (title) }

            @for article in articles::get_tagged(tag) {
                (article_summary(&article))
            }
        },
    )
}

pub fn about() -> Markup {
    layout(
        "Stephen Coakley",
        html! {
            (markdown(include_str!("../wwwroot/about.md")))
        },
    )
}

pub fn feeds() -> Markup {
    layout(
        "Stephen Coakley",
        html! {
            (markdown(include_str!("../wwwroot/feeds.md")))
        },
    )
}

pub fn stuff() -> Markup {
    layout(
        "Stephen Coakley",
        html! {
            (markdown(include_str!("../wwwroot/stuff.md")))
        },
    )
}

fn layout(title: &str, body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        head lang="en" {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width";

            title { (title) }

            meta name="description" content="I'm a software developer based in Wisconsin";
            meta name="keywords" content="Stephen Coakley, programming, web development, apps, Rust";
            meta name="author" content="Stephen Coakley";

            link rel="alternate" type="application/feed+json" href="/feed.json" title="Blog Feed";
            link rel="alternate" type="application/atom+xml" href="/feed.atom" title="Blog Feed";
            link rel="alternate" type="application/rss+xml" href="/feed.rss" title="Blog Feed";

            link rel="icon" type="image/png" href="/assets/images/favicon.128.png" sizes="128x128";
            link rel="icon" type="image/png" href="/assets/images/favicon.64.png" sizes="64x64";
            link rel="icon" type="image/png" href="/assets/images/favicon.48.png" sizes="48x48";
            link rel="icon" type="image/png" href="/assets/images/favicon.32.png" sizes="32x32";
            link rel="shortcut icon" href="/assets/images/favicon.ico";
            link rel="icon" type="image/png" href="/assets/images/favicon.16.png" sizes="16x16";

            meta name="viewport" content="initial-scale=1";

            link rel="stylesheet" href="/css/style.css";
            script defer src="https://unpkg.com/alpinejs@3.x.x/dist/cdn.min.js" {}
            script defer src="https://unpkg.com/htmx.org@1.7.0" {}
        }
        body {
            header hx-boost="true" {
                nav {
                    a id="logo" class="title" href="/" { "Stephen·Coakley" }
                    div style="flex-grow: 1;" {}
                    div class="links" {
                        a class="button" href="/articles" { "Articles" }
                        a class="button" href="/stuff" { "Stuff" }
                        a class="button" href="/feeds" { "Feeds" }
                        a class="button" href="/about" { "About" }
                    }
                }
            }

            main role="main" {
                (body)
            }

            footer {
                hr;
                p class="center" {
                    a title="Email" href="mailto:me@stephencoakley.com" { "Email" }
                    " | "
                    a title="GitHub" href="https://github.com/sagebind" { "GitHub" }
                    " | "
                    a href="/feeds" { "Feeds" }
                }

                p class="center copyright" {
                    "© " (OffsetDateTime::now_utc().year()) " Stephen Coakley"
                }
            }
        }
    }
}

fn article_summary(article: &Article) -> Markup {
    html! {
        article {
            h2 {
                a href={ "/" (&article.slug) } { (&article.title) }
            }

            small {
                (date(article.date))

                span class="tags" {
                    @for tag in &article.tags {
                        a class="tag" href={"/tag/" (tag)} { (tag) }
                    }
                }

                br;

                span { (article.estimated_reading_time()) " min read" }
            }

            p { (article.summary(250)) }
        }
    }
}
