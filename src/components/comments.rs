use maud::{html, Markup, PreEscaped};

use crate::{
    comments::Comment,
    components::{date, gravatar::gravatar},
    csrf::generate_token,
    markdown,
};

pub fn comments_section(article_slug: &str, comments: &[Comment]) -> Markup {
    let comments_count = comments.iter().map(|c| c.len()).sum::<usize>();

    html! {
        div id="comments" {
            h2 {
                (comments_count)
                " comment"
                @if comments_count != 1 {
                    "s"
                }
            }

            p {
                "Let me know what you think in the comments below. Remember to keep it civil!"
            }

            p {
                a href={ "/" (article_slug) "/comments.atom" } { "Subscribe to this thread" }
            }

            (comment_form(article_slug, None))

            @for comment in comments {
                (self::comment(comment))
            }
        }
    }
}

pub fn comment(comment: &Comment) -> Markup {
    html! {
        article class="comment" id={ "comment-" (comment.id) } x-data="{ reply: false }" {
            div class="avatar" {
                (gravatar(comment.author.email.as_deref()))
            }

            div class="text-wrapper" {
                div class="comment-toolbar" {
                    span class="author" {
                        @if let Some(website) = comment.author.website.as_ref() {
                            a href=(website) rel="nofollow" {
                                (comment.author.name)
                            }
                        } @else {
                            (comment.author.name)
                        }
                    }

                    (score_label(comment.score))

                    (date(comment.published.date()))
                }

                (PreEscaped(markdown::render_html(&comment.text, false)))

                div class="comment-toolbar" {
                    a title="Upvote"
                        hx-post="/comments/upvote/{id}"
                        hx-target="#comments"
                        tabindex="0" {
                        "▲ upvote"
                    }

                    a title="Downvote"
                        hx-post="/comments/downvote/{id}"
                        hx-target="#comments"
                        tabindex="0" {
                        "▼ downvote"
                    }

                    a href={ "#comment-" (comment.id) } { "permalink" }

                    a x-on:click="reply = !reply" x-text="reply ? 'close' : 'reply'" {
                        "reply"
                    }
                }

                div id={ "comment-" (comment.id) "-reply" } x-show="reply" {
                    (comment_form(&comment.article_slug, Some(&comment.id)))
                }

                @for child in &comment.children {
                    (self::comment(child))
                }
            }
        }
    }
}

pub fn comment_form(article_slug: &str, parent_comment_id: Option<&str>) -> Markup {
    html! {
        form class="comment-form" hx-post={ "/" (article_slug) "/comments" } hx-target="#comments" {
            input type="hidden" name="token" value=(generate_token());

            @if let Some(id) = parent_comment_id {
                input type="hidden" name="parent_id" value=(id);
            }

            div {
                textarea
                    name="text"
                    placeholder="Comment text (supports Markdown)"
                    required
                    maxlength="8192" {}
            }
            div class="author-details" {
                input
                    type="text"
                    name="author_name"
                    placeholder="Name"
                    required
                    maxlength="255";
                input
                    type="email"
                    name="author_email"
                    placeholder="Email"
                    required
                    maxlength="255";
                input
                    type="text"
                    name="author_website"
                    placeholder="Website (optional)"
                    maxlength="255";
            }
            div {
                input type="submit" value="Submit";
            }
        }
    }
}

fn score_label(score: i16) -> Markup {
    html! {
        @if score > 1 {
            span {
                (score) " points"
            }
        } @else if score == 1 {
            span {
                "1 point"
            }
        }
    }
}
