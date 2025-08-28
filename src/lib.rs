mod blog;

use crate::blog::{Article, get_articles};
use fs_extra::{
    copy_items,
    dir::{CopyOptions, create},
};
use minijinja::{Environment, context};
use std::fs::{self, read_to_string};
use std::path::Path;

pub fn generate_website() {
    init_dist_dir();

    let mut engine = Environment::new();

    let layout_template = read_to_string(Path::new("templates/layout.html")).unwrap();
    let article_template = read_to_string(Path::new("templates/article.html")).unwrap();

    engine
        .add_template("layout.html", &layout_template)
        .unwrap();
    engine
        .add_template("article.html", &article_template)
        .unwrap();

    let articles = get_articles();
    render_index(&engine, &articles);
    render_blog(&engine, &articles);
}

fn init_dist_dir() {
    let dist_path = Path::new("dist");
    let static_path = Path::new("static");
    let dist_blog_path = Path::new("dist/blog");

    create(dist_path, true).unwrap();
    create(dist_blog_path, true).unwrap();
    copy_items(&[static_path], dist_path, &CopyOptions::new()).unwrap();
}

pub fn render_index(engine: &Environment, articles: &[Article]) {
    let src = Path::new("templates/index.html");
    let dest = Path::new("dist/index.html");
    let dest_blog = Path::new("dist/blog/index.html");

    let contents = read_to_string(src).unwrap();
    let template_str = engine
        .render_str(
            &contents,
            context! {title => "matx.dev", articles => articles},
        )
        .unwrap();

    fs::write(dest, &template_str).unwrap();
    fs::write(dest_blog, template_str).unwrap();
}

pub fn render_blog(engine: &Environment, articles: &[Article]) {
    for article in articles {
        let template = engine.get_template("article.html").unwrap();
        let file_str = template
            .render(
                context! {title => &article.title, date => &article.date, body => &article.body},
            )
            .unwrap();

        let path = format!("dist/blog/{}.html", &article.slug);
        fs::write(Path::new(&path), file_str).unwrap();
    }
}
