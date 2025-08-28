use comrak::{Options, markdown_to_html};
use serde::Serialize;
use std::fs::{read_dir, read_to_string};
use yaml_rust2::YamlLoader;

#[derive(Serialize)]
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
        let body = markdown_to_html(&body, &Options::default());
        let slug = title
            .split(' ')
            .collect::<Vec<&str>>()
            .join("-")
            .as_str()
            .to_owned();

        Self {
            title,
            date,
            body,
            slug,
        }
    }
}

pub fn get_articles() -> Vec<Article> {
    let mut entries: Vec<_> = read_dir("articles").unwrap().flatten().collect();
    entries.sort_by_key(|b| std::cmp::Reverse(b.file_name()));

    let mut articles = Vec::new();

    for file in entries {
        let contents = read_to_string(file.path()).unwrap();
        let (header, body) = split_header_and_body(contents);

        let article = Article::new(header, body);
        articles.push(article);
    }

    articles
}

fn split_header_and_body(contents: String) -> (String, String) {
    let mut header = String::new();
    let mut body = String::new();
    let mut is_header = false;
    let mut finished_header = false;

    for line in contents.lines() {
        if line == "---" && !finished_header {
            if is_header {
                finished_header = true;
            }
            is_header = !is_header;
        } else if is_header && !finished_header {
            header += line;
            header += "\n";
        } else {
            body += line;
            body += "\n";
        }
    }

    (header, body)
}
