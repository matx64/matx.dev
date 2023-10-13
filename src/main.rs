use askama::Template;
use std::{error::Error, fs};

#[derive(Template, Debug)]
#[template(path = "post.html")]
pub struct PostTemplate<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub date: &'a str,
    pub body: &'a str,
}

fn main() -> Result<(), Box<dyn Error>> {
    let posts_dir = fs::read_dir("posts")?;

    for file in posts_dir {
        let file = file?;
        let contents = fs::read_to_string(file.path())?;

        let (header, body) = split_header_and_body(&contents);

        let header: serde_yaml::Value = serde_yaml::from_str(&header)?;

        let post = PostTemplate {
            title: header["title"].as_str().unwrap(),
            description: header["description"].as_str().unwrap(),
            date: header["date"].as_str().unwrap(),
            body: &body,
        };

        let file_path = "./dist/blog/".to_owned()
            + &file.path().file_name().unwrap().to_str().unwrap()
            + ".html";

        if let Some(parent_dir) = std::path::Path::new(&file_path).parent() {
            fs::create_dir_all(parent_dir).expect("Unable to create directory.");
        }

        fs::write(file_path, post.render()?).expect("Unable to write.");
    }

    Ok(())
}

fn split_header_and_body(contents: &str) -> (String, String) {
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
