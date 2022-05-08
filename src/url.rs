//! Helpers for scanning text for URLs.

use once_cell::sync::Lazy;
use regex::Regex;

static REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"https?://[\w\-]+(\.[\w\-]+)+([\w/\-@:%=+~]+)?(\.[\w/\-@:%=+~]+)*(\?[\w/\-@:%=+~]+)?#?[\w/\-@:%=+~]*"#).unwrap()
});

pub fn count(text: &str) -> usize {
    REGEX.find_iter(text).count()
}

pub fn find<'a>(text: impl AsRef<str> + 'a) -> Option<(String, String, String)> {
    let text = text.as_ref();
    let m = REGEX.find(text)?;

    Some((
        text[..m.start()].into(),
        m.as_str().into(),
        text[m.end()..].into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_links() {
        assert_eq!(
            count(
                "hello world http://example.org/foo <a href=\"https://www.example.com\">link</a>"
            ),
            2
        );
    }

    #[test]
    fn find_nothing() {
        assert_eq!(find("hello world"), None);
    }

    #[test]
    fn find_one_link() {
        let result = find("hello https://world.com all!").unwrap();
        assert_eq!(result.0, "hello ");
        assert_eq!(result.1, "https://world.com");
        assert_eq!(result.2, " all!");
    }
}
