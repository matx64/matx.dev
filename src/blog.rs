use comrak::{Options, markdown_to_html};
use std::fs::{read_dir, read_to_string};
use yaml_rust2::YamlLoader;

pub struct Article {
    pub title: String,
    pub description: String,
    pub date: String,
    pub body: String,
    pub slug: String,
}

pub fn get_articles() -> Vec<Article> {
    let mut articles = vec![];

    for file in read_dir("articles").unwrap().flatten() {
        let contents = read_to_string(file.path()).unwrap();
        let (header, body) = split_header_and_body(contents);

        let article = Article::new(header, body);
        articles.push(article);
    }

    articles
}

impl Article {
    pub fn new(header: String, body: String) -> Self {
        let doc = YamlLoader::load_from_str(&header).expect("Invalid Article header format");
        let doc = &doc[0];

        let title = doc["title"]
            .as_str()
            .expect("Article missing title header property")
            .to_owned();
        let description = doc["description"]
            .as_str()
            .expect("Article missing description header property")
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
            description,
            date,
            body,
            slug,
        }
    }
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
