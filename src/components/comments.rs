use maud::{html, Markup};

use crate::{
    comments::Comment,
    components::{date, gravatar::gravatar, markdown},
};

pub fn comments_section(comments: &[Comment]) -> Markup {
    html! {
        div id="comments" {
            h2 {
                (comments.len())
                " comment"
                @if comments.len() != 1 {
                    "s"
                }
            }

            p {
                "Let me know what you think in the comments below. Remember to keep it civil!"
            }

            p {
                a href="/${this.articleSlug}/comments.atom" { "Subscribe to this thread" }
            }

            (comment_form(None))

            @for comment in comments {
                (self::comment(comment))
            }
        }
    }
}

pub fn comment(comment: &Comment) -> Markup {
    let show_reply = false;

    html! {
        article class="comment" id={ "comment-" (comment.id) } {
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

                (markdown(&comment.text))

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

                    @if show_reply {
                        a onclick="${() => this.showReply = false}" tabindex="0" { "close" }
                    } @else {
                        a onclick="${() => this.showReply = true}" tabindex="0" { "reply" }
                    }
                }

                @if show_reply {
                    (comment_form(Some(&comment.id)))
                }

                @for child in &comment.children {
                    (self::comment(child))
                }
            }
        }
    }
}

fn comment_form(parent_comment_id: Option<&str>) -> Markup {
    html! {
        form class="comment-form" hx-post="/comments" hx-target="#comments" {
            @if let Some(id) = parent_comment_id {
                input type="hidden" name="parent_comment_id" value=(id);
            }

            div {
                textarea
                    name="text"
                    placeholder="Comment text (supports Markdown)"
                    required {}
            }
            div class="author-details" {
                input
                    type="text"
                    name="name"
                    placeholder="Name"
                    required
                    maxlength="255";
                input
                    type="email"
                    name="email"
                    placeholder="Email"
                    required
                    maxlength="255";
                input
                    type="text"
                    name="website"
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
