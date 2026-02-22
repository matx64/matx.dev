use comrak::plugins::syntect::SyntectAdapterBuilder;
use comrak::{Options, Plugins, markdown_to_html_with_plugins};
use serde::Serialize;
use std::fs::{read_dir, read_to_string};
use yaml_rust2::YamlLoader;

#[derive(Serialize, Debug)]
pub struct Article {
    pub title: String,
    pub date: String,
    pub body: String,
    pub slug: String,
}

impl Article {
    pub fn new(header: String, body: String) -> Self {
        let doc = YamlLoader::load_from_str(&header).expect("Invalid Article header format");
        let doc = &doc[0];

        let title = doc["title"]
            .as_str()
            .expect("Article missing title header property")
            .to_owned();
        let date = doc["date"]
            .as_str()
            .expect("Article missing date header property")
            .to_owned();

        let adapter = SyntectAdapterBuilder::new().css().build();
        let mut plugins = Plugins::default();
        plugins.render.codefence_syntax_highlighter = Some(&adapter);

        let body = markdown_to_html_with_plugins(&body, &Options::default(), &plugins);
        let slug = title
            .to_lowercase()
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || c == '-' {
                    c
                } else {
                    ' '
                }
            })
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join("-");

        Self {
            title,
            date,
            body,
            slug,
        }
    }
}

pub fn get_articles() -> Vec<Article> {
    let mut entries: Vec<_> = read_dir("articles")
        .expect("Failed to read articles directory")
        .flatten()
        .collect();
    entries.sort_by_key(|b| std::cmp::Reverse(b.file_name()));

    let mut articles = Vec::new();

    for file in entries {
        let contents = read_to_string(file.path()).expect("Failed to read article file");
        let (header, body) = split_header_and_body(&contents);

        let article = Article::new(header, body);
        articles.push(article);
    }

    articles
}

fn split_header_and_body(contents: &str) -> (String, String) {
    let inner = contents
        .strip_prefix("---\n")
        .expect("Article missing opening --- delimiter");
    let (header, body) = inner
        .split_once("\n---")
        .expect("Article missing closing --- delimiter");
    let body = body.trim_start_matches(['\n', '\r']);
    (header.to_owned(), body.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_header_and_body_valid() {
        let input = "---\ntitle: Test\ndate: 2025-01-01\n---\nThis is the body\n";
        let (header, body) = split_header_and_body(input);
        assert_eq!(header, "title: Test\ndate: 2025-01-01");
        assert_eq!(body, "This is the body\n");
    }

    #[test]
    #[should_panic(expected = "Article missing opening --- delimiter")]
    fn test_split_header_and_body_missing_delimiters() {
        let input = "title: Test\nNo delimiters here";
        split_header_and_body(input);
    }

    #[test]
    fn test_split_header_and_body_empty_body() {
        let input = "---\ntitle: Test\n---\n";
        let (header, body) = split_header_and_body(input);
        assert_eq!(header, "title: Test");
        assert_eq!(body, "");
    }

    #[test]
    fn test_split_header_body_body_with_triple_dash() {
        let input = "---\ntitle: Test\n---\nBody content\n---\nMore content\n";
        let (header, body) = split_header_and_body(input);
        assert_eq!(header, "title: Test");
        assert_eq!(body, "Body content\n---\nMore content\n");
    }

    #[test]
    #[should_panic(expected = "Article missing opening --- delimiter")]
    fn test_split_header_body_no_opening_delimiter() {
        let input = "title: Test\ndate: 2025-01-01\n---\nBody here\n";
        split_header_and_body(input);
    }

    #[test]
    #[should_panic(expected = "Article missing closing --- delimiter")]
    fn test_split_header_body_no_closing_delimiter() {
        let input = "---\ntitle: Test\ndate: 2025-01-01\n";
        split_header_and_body(input);
    }

    #[test]
    fn test_article_new_valid() {
        let header = String::from("title: \"Test Article\"\ndate: \"Jan 1, 2025\"\n");
        let body = String::from("Test content");
        let article = Article::new(header, body);

        assert_eq!(article.title, "Test Article");
        assert_eq!(article.date, "Jan 1, 2025");
        assert!(article.body.contains("<p>Test content</p>"));
    }

    #[test]
    #[should_panic(expected = "Article missing title header property")]
    fn test_article_new_missing_title() {
        let header = String::from("date: \"Jan 1, 2025\"\n");
        let body = String::from("Test content");
        Article::new(header, body);
    }

    #[test]
    #[should_panic(expected = "Article missing date header property")]
    fn test_article_new_missing_date() {
        let header = String::from("title: \"Test Article\"\n");
        let body = String::from("Test content");
        Article::new(header, body);
    }

    #[test]
    fn test_slug_generation_lowercase() {
        let header = String::from("title: \"Test Article\"\ndate: \"Jan 1, 2025\"\n");
        let body = String::from("Test content");
        let article = Article::new(header, body);
        assert_eq!(article.slug, "test-article");
    }

    #[test]
    fn test_slug_generation_special_chars() {
        let header = String::from("title: \"Hello, World!\"\ndate: \"Jan 1, 2025\"\n");
        let body = String::from("Test content");
        let article = Article::new(header, body);
        assert_eq!(article.slug, "hello-world");
    }

    #[test]
    fn test_slug_generation_multiple_spaces() {
        let header = String::from("title: \"  Multiple   Spaces  \"\ndate: \"Jan 1, 2025\"\n");
        let body = String::from("Test content");
        let article = Article::new(header, body);
        assert_eq!(article.slug, "multiple-spaces");
    }

    #[test]
    fn test_slug_with_existing_dashes() {
        let header = String::from("title: \"my-article\"\ndate: \"Jan 1, 2025\"\n");
        let body = String::from("Test content");
        let article = Article::new(header, body);
        assert_eq!(article.slug, "my-article");
    }

    #[test]
    fn test_slug_unicode_stripped() {
        let header = String::from("title: \"café résumé\"\ndate: \"Jan 1, 2025\"\n");
        let body = String::from("Test content");
        let article = Article::new(header, body);
        assert_eq!(article.slug, "caf-r-sum");
    }
}
