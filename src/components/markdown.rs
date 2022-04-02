use maud::{html, PreEscaped, Markup};

pub fn markdown(markdown: &str) -> Markup {
    html! {
        (PreEscaped(crate::markdown::render_html(markdown)))
    }
}
