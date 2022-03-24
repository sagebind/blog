use comrak::{
    markdown_to_html_with_plugins, plugins::syntect::SyntectAdapter, ComrakExtensionOptions,
    ComrakOptions, ComrakParseOptions, ComrakPlugins, ComrakRenderOptions, ComrakRenderPlugins,
};

pub fn render(markdown: &str) -> String {
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
        },
    };

    markdown_to_html_with_plugins(markdown, &comrak_options, &plugins)
}
