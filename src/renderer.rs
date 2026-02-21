use crate::blog::Article;
use minijinja::{Environment, context};
use std::fs;
use std::fs::read_to_string;
use std::path::Path;

pub(crate) const DIST_DIR: &str = "dist";
pub(crate) const DIST_BLOG_DIR: &str = "dist/blog";
pub(crate) const STATIC_DIR: &str = "static";
pub(crate) const LAYOUT_TEMPLATE_PATH: &str = "templates/layout.html";
pub(crate) const ARTICLE_TEMPLATE_PATH: &str = "templates/article.html";

const INDEX_TEMPLATE_PATH: &str = "templates/index.html";
const DIST_INDEX_PATH: &str = "dist/index.html";
const DIST_BLOG_INDEX_PATH: &str = "dist/blog/index.html";

pub(crate) fn render_index(engine: &Environment, articles: &[Article]) {
    let contents = read_to_string(Path::new(INDEX_TEMPLATE_PATH))
        .expect("Failed to read index.html template");
    let rendered = engine
        .render_str(
            &contents,
            context! {title => "matx.dev", articles => articles},
        )
        .expect("Failed to render index template");

    fs::write(Path::new(DIST_INDEX_PATH), &rendered).expect("Failed to write dist/index.html");
    fs::write(Path::new(DIST_BLOG_INDEX_PATH), rendered)
        .expect("Failed to write dist/blog/index.html");
}

pub(crate) fn render_blog(engine: &Environment, articles: &[Article]) {
    let template = engine
        .get_template("article.html")
        .expect("Failed to get article.html template");

    for article in articles {
        let rendered = template
            .render(
                context! {title => &article.title, date => &article.date, body => &article.body},
            )
            .expect("Failed to render article template");

        let path = format!("dist/blog/{}.html", &article.slug);
        fs::write(Path::new(&path), rendered).expect("Failed to write article HTML");
    }
}
