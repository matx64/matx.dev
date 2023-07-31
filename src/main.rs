use std::{error::Error, fs};

use askama::Template;

#[derive(Template, Debug)]
#[template(path = "post.html")]
pub struct PostTemplate<'a> {
    pub title: &'a str,
    pub date: &'a str,
    pub body: &'a str,
}

fn main() -> Result<(), Box<dyn Error>> {
    let posts_dir = fs::read_dir("blog-posts")?;

    for file in posts_dir {
        let file = file?;

        let contents = fs::read_to_string(file.path())?;

        let (mut title, mut date) = (String::new(), String::new());

        let mut i: u8 = 0;
        for line in contents.lines() {
            if i == 1 {
                date = line.strip_prefix("###### ").unwrap().to_string();
                break;
            }
            title = line.strip_prefix("# ").unwrap().to_string();
            i += 1;
        }

        let post = PostTemplate {
            title: &title,
            date: &date,
            body: &contents,
        };

        fs::write(
            "./dist/pages/blog/".to_owned() + &title.to_lowercase().replace(" ", "-") + ".html",
            post.render()?,
        )
        .expect("Unable to write.");
    }

    Ok(())
}
