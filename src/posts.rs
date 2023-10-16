use crate::templates::PostTemplate;
use askama::Template;
use std::fs;
use std::{error::Error, path::Path};

pub struct Post {
    pub title: String,
    pub description: String,
    pub date: String,
    pub body: String,
    pub filename: String,
}

pub fn render_posts(posts: &Vec<Post>) -> Result<(), Box<dyn Error>> {
    let target_path = Path::new("./dist/blog");
    fs::create_dir_all(target_path)?;

    for post in posts {
        let post_template = PostTemplate {
            title: &post.title,
            description: &post.description,
            date: &post.date,
            body: &post.body,
        };

        let file_path = format!("./dist/blog/{}.html", &post.filename);

        fs::write(file_path, post_template.render()?).expect("Unable to write.");
    }
    Ok(())
}

pub fn load_posts() -> Result<Vec<Post>, Box<dyn Error>> {
    let post_contents = read_posts()?;
    let mut posts = Vec::new();

    for content in post_contents {
        let (header, body) = split_header_and_body(content);
        let header: serde_yaml::Value = serde_yaml::from_str(&header)?;

        let title = header["title"].as_str().unwrap().to_string();

        let post = Post {
            title: title.clone(),
            description: header["description"].as_str().unwrap().to_string(),
            date: header["date"].as_str().unwrap().to_string(),
            body,
            filename: title.replace(" ", "-"),
        };

        posts.push(post);
    }

    Ok(posts)
}

fn read_posts() -> Result<Vec<String>, Box<dyn Error>> {
    let posts_dir = fs::read_dir("blog-posts")?;
    let mut post_contents = Vec::new();

    for file in posts_dir {
        let contents = fs::read_to_string(file?.path())?;
        post_contents.push(contents);
    }

    Ok(post_contents)
}

fn split_header_and_body(contents: String) -> (String, String) {
    let mut header = String::new();
    let mut body = String::new();
    let mut is_header = false;

    for line in contents.lines() {
        if line == "---" {
            is_header = !is_header;
        } else if is_header {
            header += line;
            header += "\n";
        } else {
            body += line;
            body += "\n";
        }
    }

    (header, body)
}
