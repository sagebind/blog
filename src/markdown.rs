//! Markdown parsing and rendering based on pulldown-cmark with some added
//! features.

use std::iter;

use maud::{html, PreEscaped};
use once_cell::sync::Lazy;
use pulldown_cmark::{html, CodeBlockKind, Event, LinkType, Options, Parser, Tag};
use regex::Regex;

use crate::highlight::{find_syntax, highlight};

const OPTIONS: Options = Options::all();

/// Render a block of Markdown into HTML.
pub fn render_html(markdown: &str) -> String {
    let parser = highlight_code(autolink(Parser::new_ext(markdown, OPTIONS)));

    let mut html_buf = String::new();
    html::push_html(&mut html_buf, parser);

    html_buf
}

/// Render a block of Markdown into plain text.
pub fn render_plaintext(markdown: &str) -> String {
    Parser::new_ext(markdown, Options::ENABLE_SMART_PUNCTUATION).fold(
        String::new(),
        |mut s, event| {
            match event {
                Event::Text(text) | Event::Code(text) => s.push_str(&text),
                Event::SoftBreak | Event::Start(Tag::Item) => s.push(' '),
                Event::HardBreak | Event::End(Tag::Paragraph) => s.push_str("\n\n"),
                _ => {}
            };

            s
        },
    )
}

/// Detect URLs in plain text and transform them into links automatically.
fn autolink<'a>(mut events: impl Iterator<Item = Event<'a>>) -> impl Iterator<Item = Event<'a>> {
    static LINK_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r#"https?://[\w\-]+(\.[\w\-]+)+([\w/\-@:%=+~]+)?(\.[\w/\-@:%=+~]+)*(\?[\w/\-@:%=+~]+)?#?[\w/\-@:%=+~]*"#).unwrap());

    let mut stack = Vec::new();
    let mut inside_code_block = false;
    let mut preparsed_count = 0;

    iter::from_fn(move || {
        let preparsed = if preparsed_count > 0 {
            preparsed_count -= 1;
            true
        } else {
            false
        };

        match stack.pop().or_else(|| events.next())? {
            Event::Text(text) if !preparsed && !inside_code_block => {
                if let Some(m) = LINK_REGEX.find(&text) {
                    let url = m.as_str().to_string();
                    let link = Tag::Link(LinkType::Autolink, url.clone().into(), "".into());

                    stack.push(Event::Text(text[m.end()..].to_string().into()));
                    stack.push(Event::End(link.clone()));
                    stack.push(Event::Text(url.into()));
                    stack.push(Event::Start(link.clone()));

                    preparsed_count += 3;

                    Some(Event::Text(text[..m.start()].to_string().into()))
                } else {
                    Some(Event::Text(text))
                }
            }
            event @ Event::Start(Tag::CodeBlock(_)) => {
                inside_code_block = true;
                Some(event)
            }
            event @ Event::End(Tag::CodeBlock(_)) => {
                inside_code_block = false;
                Some(event)
            }
            event => Some(event),
        }
    })
}

/// Transform fenced code blocks into pre-highlighted HTML.
fn highlight_code<'a>(
    mut events: impl Iterator<Item = Event<'a>>,
) -> impl Iterator<Item = Event<'a>> {
    iter::from_fn(move || {
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
