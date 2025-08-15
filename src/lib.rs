mod blog;

use crate::blog::{Article, get_articles};
use fs_extra::{
    copy_items,
    dir::{CopyOptions, create},
};
use minijinja::{Environment, context};
use std::fs::{self, create_dir_all, read_to_string};
use std::path::Path;

pub fn generate_website() {
    init_dist();
    let mut env = Environment::new();

    let layout_template = read_to_string(Path::new("templates/layout.html")).unwrap();
    let article_template = read_to_string(Path::new("templates/article.html")).unwrap();
    env.add_template("layout.html", &layout_template).unwrap();
    env.add_template("article.html", &article_template).unwrap();

    let articles = get_articles();
    render_index(&env, &articles);
    render_blog(&env, &articles);
}

fn init_dist() {
    let dist_path = Path::new("dist");
    let static_path = Path::new("static");
    create(dist_path, true).unwrap();
    copy_items(&[static_path], dist_path, &CopyOptions::new()).unwrap();
}

pub fn render_index(env: &Environment, articles: &[Article]) {
    let src = Path::new("templates/index.html");
    let dest = Path::new("dist/index.html");

    let contents = read_to_string(src).unwrap();
    let template_str = env
        .render_str(
            &contents,
            context! {title => "matx.dev", articles => articles},
        )
        .unwrap();

    fs::write(dest, template_str).unwrap();
}

pub fn render_blog(env: &Environment, articles: &[Article]) {
    create_dir_all("dist/blog").unwrap();

    for article in articles {
        let template = env.get_template("article.html").unwrap();
        let file_str = template
            .render(
                context! {title => &article.title, date => &article.date, body => &article.body},
            )
            .unwrap();

        let path = format!("dist/blog/{}.html", &article.slug);
        fs::write(Path::new(&path), file_str).unwrap();
    }
}
