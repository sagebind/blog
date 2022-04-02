use std::fs;

use itertools::Itertools;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use regex::Regex;
use serde::Deserialize;
use time::{Date, Month};

use crate::markdown;

static WORD_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\b[\w']+\b").unwrap());

#[derive(Clone, Debug)]
pub struct Article {
    pub slug: String,
    pub title: String,
    pub author: String,
    pub date: Date,
    pub tags: Vec<String>,
    pub content_html: String,
    pub content_text: String,
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
            slug: format!("{:04}/{:02}/{:02}/{}", year, month, day, slug),
            title: frontmatter.title,
            author: frontmatter.author,
            date: Date::from_calendar_date(year, date_month, day).unwrap(),
            tags: frontmatter.tags,
            content_html: markdown::render_html(source),
            content_text: markdown::render_plaintext(source),
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
        if self.content_text.len() > len {
            if let Some(i) = &self.content_text[..len].rfind(" ") {
                format!("{}...", &self.content_text[..*i])
            } else {
                self.content_text.clone()
            }
        } else {
            self.content_text.clone()
        }
    }

    pub fn canonical_url(&self) -> String {
        format!(
            "https://stephencoakley.com/{}",
            self.slug
        )
    }
}

pub fn get_all(_include_unpublished: bool) -> &'static [Article] {
    static ARTICLES: Lazy<Vec<Article>> = Lazy::new(load);

    ARTICLES.as_slice()
}

pub fn get_tagged(tag: impl AsRef<str>) -> Vec<Article> {
    let tag = tag.as_ref();

    get_all(false).into_iter()
        .cloned()
        .filter(|article| article.has_tag(tag))
        .collect()
}

pub fn get_by_slug(slug: &str) -> Option<Article> {
    get_all(false).into_iter()
        .filter(|article| article.slug == slug)
        .next()
        .cloned()
}

#[derive(Deserialize)]
struct Frontmatter {
    title: String,
    author: String,
    tags: Vec<String>,
}

fn load() -> Vec<Article> {
    log::info!("parsing articles...");

    let mut articles = fs::read_dir("articles").unwrap()
        .map(|e| e.unwrap())
        .collect_vec()
        .into_par_iter()
        .map(|file| {
            let filename = file
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .map(|s| s.to_owned())
                .unwrap();

            Article::parse(&filename, &fs::read_to_string(file.path()).unwrap())
        })
        .collect::<Vec<_>>();

    articles.sort_by(|a, b| b.date.cmp(&a.date));

    articles
}
