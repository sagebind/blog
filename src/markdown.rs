//! Markdown parsing and rendering based on pulldown-cmark with some added
//! features.

use std::iter;

use maud::{html, PreEscaped};
use pulldown_cmark::{html, CodeBlockKind, Event, LinkType, Options, Parser, Tag};

use crate::{
    highlight::{find_syntax, highlight},
    url,
};

/// Render a block of Markdown into HTML.
pub fn render_html(markdown: impl AsRef<str>, trusted: bool) -> String {
    let options = if trusted {
        Options::all()
    } else {
        Options::ENABLE_SMART_PUNCTUATION | Options::ENABLE_STRIKETHROUGH
    };

    let parser = highlight_code(autolink(Parser::new_ext(markdown.as_ref(), options)));

    fn render<'a>(parser: impl Iterator<Item = Event<'a>>) -> String {
        let mut html_buf = String::new();
        html::push_html(&mut html_buf, parser);

        html_buf
    }

    if trusted {
        render(parser)
    } else {
        render(nofollow_links(parser))
    }
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
                if let Some((prefix, url, suffix)) = url::find(&text) {
                    let link = Tag::Link(LinkType::Autolink, url.clone().into(), "".into());

                    stack.push(Event::Text(suffix.into()));
                    stack.push(Event::End(link.clone()));
                    stack.push(Event::Text(url.into()));
                    stack.push(Event::Start(link.clone()));

                    preparsed_count += 3;

                    return Some(Event::Text(prefix.into()));
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

fn nofollow_links<'a>(
    mut events: impl Iterator<Item = Event<'a>>,
) -> impl Iterator<Item = Event<'a>> {
    iter::from_fn(move || {
        let mut current_link = None;
        let mut link_children = Vec::new();

        loop {
            match events.next()? {
                Event::Start(link @ Tag::Link(..)) => {
                    current_link = Some(link);
                    link_children.clear();
                }
                Event::End(Tag::Link(_, url, title)) if current_link.is_some() => {
                    let mut html = String::new();
                    html::push_html(&mut html, link_children.drain(..));

                    return Some(Event::Html(html! {
                        a href=(url) rel="nofollow" title=[Some(title).filter(|s| !s.is_empty())] {
                            (PreEscaped(html))
                        }
                    }.into_string().into()));
                }
                event if current_link.is_some() => link_children.push(event),
                event => return Some(event),
            }
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
