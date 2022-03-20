use comrak::{markdown_to_html_with_plugins, ComrakOptions, ComrakExtensionOptions, ComrakRenderOptions, ComrakParseOptions, ComrakPlugins, ComrakRenderPlugins, plugins::syntect::SyntectAdapter};
use maud::{html, PreEscaped, Markup};

pub fn markdown(markdown: &str) -> Markup {
    let comrak_options = ComrakOptions {
        extension: ComrakExtensionOptions {
            strikethrough: true,
            tagfilter: false,
            table: true,
            autolink: true,
            tasklist: true,
            superscript: false,
            header_ids: Some(String::new()),
            footnotes: true,
            front_matter_delimiter: Some("+++".into()),
            description_lists: true,
        },
        parse: ComrakParseOptions {
            smart: true,
            default_info_string: None,
        },
        render: ComrakRenderOptions {
            hardbreaks: true,
            github_pre_lang: true,
            width: 0,
            unsafe_: true,
            escape: false,
        },
    };

    let syntect = SyntectAdapter::new("base16-eighties.dark");

    let plugins = ComrakPlugins {
        render: ComrakRenderPlugins {
            codefence_syntax_highlighter: Some(&syntect),
        }
    };

    html! {
        (PreEscaped(markdown_to_html_with_plugins(
            markdown,
            &comrak_options,
            &plugins
        )))
    }
}
