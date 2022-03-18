use include_dir::{include_dir, Dir};
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use time::{Date, Month};

static WORD_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\b[\w']+\b").unwrap());

static ARTICLES_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/articles");

pub struct Article {
    pub slug: String,
    pub title: String,
    pub author: String,
    pub date: Date,
    pub tags: Vec<String>,
    pub source: String,
    pub word_count: usize,
}

impl Article {
    fn parse(filename: &str, mut source: &str) -> Self {
        let mut split = filename.splitn(4, "-");
        let year = split.next().unwrap().parse::<i32>().unwrap();
        let month = split.next().unwrap().parse::<u8>().unwrap();
        let day = split.next().unwrap().parse::<u8>().unwrap();
        let slug = split.next().unwrap().strip_suffix(".md").unwrap();

        // If a TOML front matter block is given, parse the contained metadata.
        let suffix = source.strip_prefix("+++").unwrap();
        let (frontmatter, markdown) = suffix.split_once("+++").unwrap();

        let frontmatter = toml::from_str::<Frontmatter>(frontmatter).unwrap();

        source = markdown.trim();

        let word_count = WORD_REGEX.find_iter(source).count();

        let mut date_month = Month::January;

        for _ in 1..month {
            date_month = date_month.next();
        }

        Self {
            slug: format!("{}/{}/{}/{}", year, month, day, slug),
            title: frontmatter.title,
            author: frontmatter.author,
            date: Date::from_calendar_date(year, date_month, day).unwrap(),
            tags: frontmatter.tags,
            source: source.to_owned(),
            word_count,
        }
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }

    pub fn estimated_reading_time(&self) -> usize {
        (self.word_count / 200).max(1)
    }

    pub fn summary(&self, len: usize) -> String {
        if self.source.len() > len {
            if let Some(i) = &self.source[..len].rfind(" ") {
                format!("{}...", &self.source[..*i])
            } else {
                self.source.clone()
            }
        } else {
            self.source.clone()
        }
    }
}

pub fn get_all(_include_unpublished: bool) -> Vec<Article> {
    ARTICLES_DIR
        .files()
        .sorted_by_key(|file| file.path())
        .rev()
        .map(|file| {
            let filename = file
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap();

            Article::parse(filename, file.contents_utf8().unwrap())
        })
        .collect()
}

pub fn get_tagged(tag: &str) -> Vec<Article> {
    get_all(false).into_iter()
        .filter(|article| article.has_tag(tag))
        .collect()
}

pub fn get_by_slug(slug: &str) -> Option<Article> {
    get_all(false).into_iter()
        .filter(|article| article.slug == slug)
        .next()
}

#[derive(Deserialize)]
struct Frontmatter {
    title: String,
    author: String,
    tags: Vec<String>,
}
