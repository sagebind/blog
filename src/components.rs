use maud::{html, Markup, PreEscaped};
use time::{Date, OffsetDateTime};

use crate::markdown::render_html;

pub mod comments;
pub mod gravatar;
pub mod markdown;

pub fn date(date: Date) -> Markup {
    let today = OffsetDateTime::now_utc().date();
    let days = (today - date).whole_days();

    html! {
        time datetime=(date) title=(date) {
            @if date == today {
                "today"
            } @else if date < today {
                @if days > (365 * 2) {
                    ({ days / 365 }) " years ago"
                } @else if days > 365 {
                    "1 year ago"
                } @else if days > 60 {
                    ({ days / 30 }) " months ago"
                } @else if days > 30 {
                    "1 month ago"
                } @else if days > 1 {
                    (days) " days ago"
                } @else {
                    "yesterday"
                }
            } @else {
                (date)
            }
        }
    }
}

pub fn markdown(markdown: &str) -> Markup {
    html! {
        (PreEscaped(render_html(markdown)))
    }
}

