use std::{fs::{self, create_dir_all}, path::Path};
use comrak::{markdown_to_html, Options};
use minijinja::{context, Environment};

pub fn render_blog(env: &Environment) {
    create_dir_all("dist/blog").unwrap();

    for file in fs::read_dir("articles").unwrap().flatten() {
        let contents = fs::read_to_string(file.path()).unwrap();
        let (header, body) = split_header_and_body(contents);
        
        let body = markdown_to_html(&body, &Options::default());

        let template = env.get_template("post.html").unwrap();
        let file_str = template.render(context! {title => "post", body => body}).unwrap();

        fs::write(Path::new("dist/blog/post1.html"), file_str).unwrap();
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