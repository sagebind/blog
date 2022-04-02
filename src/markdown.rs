use maud::{html, PreEscaped};
use pulldown_cmark::{html, CodeBlockKind, Event, Parser, Tag, Options};

use crate::highlight::{find_syntax, highlight};

const OPTIONS: Options = Options::all();

pub fn render(markdown: &str) -> String {
    let parser = highlight_code(Parser::new_ext(markdown, OPTIONS));

    let mut html_buf = String::new();
    html::push_html(&mut html_buf, parser);

    html_buf
}

fn highlight_code<'a>(
    mut events: impl Iterator<Item = Event<'a>>,
) -> impl Iterator<Item = Event<'a>> {
    std::iter::from_fn(move || {
        let event = events.next()?;

        if let Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) = &event {
            if let Some(syntax_ref) = find_syntax(&lang) {
                let mut code = None;

                loop {
                    match events.next()? {
                        Event::Text(text) => code = Some(text),
                        Event::End(Tag::CodeBlock(_)) => break,
                        e => panic!("unexpected event: {:?}", e),
                    }
                }

                let highlighted = highlight(&code.unwrap(), &syntax_ref);

                return Some(Event::Html(
                    html! {
                        pre class={ "language-" (lang) } {
                            code class={ "language-" (lang) } {
                                (PreEscaped(highlighted))
                            }
                        }
                    }
                    .into_string()
                    .into(),
                ));
            }
        }

        Some(event)
    })
}
