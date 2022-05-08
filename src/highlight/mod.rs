use once_cell::sync::Lazy;
use syntect::{
    easy::HighlightLines,
    highlighting::{Theme, ThemeSet},
    html::{append_highlighted_html_for_styled_line, IncludeBackground},
    parsing::{SyntaxDefinition, SyntaxReference, SyntaxSet},
    util::LinesWithEndings,
};

static CONFIG: Lazy<Config> = Lazy::new(Config::default);

struct Config {
    syntax_set: SyntaxSet,
    theme: Theme,
}

impl Default for Config {
    fn default() -> Self {
        let mut syntax_builder = SyntaxSet::load_defaults_newlines().into_builder();
        syntax_builder.add(
            SyntaxDefinition::load_from_str(include_str!("toml.sublime-syntax"), true, None)
                .unwrap(),
        );

        Self {
            syntax_set: syntax_builder.build(),
            theme: ThemeSet::load_defaults().themes["base16-ocean.dark"].clone(),
        }
    }
}

pub fn find_syntax(name: &str) -> Option<&'static SyntaxReference> {
    CONFIG.syntax_set.syntaxes().iter().find(|syntax| {
        syntax.name.eq_ignore_ascii_case(name)
            || syntax
                .file_extensions
                .iter()
                .any(|extension| extension.eq_ignore_ascii_case(name))
            || (syntax.name == "C#" && name == "csharp")
    })
}

pub fn highlight(code: &str, syntax: &SyntaxReference) -> String {
    let mut highlighter = HighlightLines::new(syntax, &CONFIG.theme);
    let mut output = String::new();

    // For PHP add an implicit `<?php` header if not present in the source code.
    // This is not actually rendered in the output and only used to set the
    // appropriate scope for the syntax parser.
    if syntax.name == "PHP" && !code.contains("<?php") {
        highlighter.highlight("<?php\n", &CONFIG.syntax_set);
    }

    for line in LinesWithEndings::from(code) {
        let regions = highlighter.highlight(line, &CONFIG.syntax_set);
        append_highlighted_html_for_styled_line(&regions[..], IncludeBackground::No, &mut output);
    }

    output
}
