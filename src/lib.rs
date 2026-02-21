mod blog;
mod renderer;

use crate::blog::get_articles;
use crate::renderer::{
    ARTICLE_TEMPLATE_PATH, DIST_BLOG_DIR, DIST_DIR, LAYOUT_TEMPLATE_PATH, STATIC_DIR,
};
use fs_extra::{
    copy_items,
    dir::{CopyOptions, create},
};
use minijinja::Environment;
use std::fs::{self, read_to_string};
use std::path::Path;

pub fn generate_website() {
    init_dist_dir();

    let layout_template = read_to_string(Path::new(LAYOUT_TEMPLATE_PATH))
        .expect("Failed to read layout.html template");
    let article_template = read_to_string(Path::new(ARTICLE_TEMPLATE_PATH))
        .expect("Failed to read article.html template");

    let mut engine = Environment::new();
    engine
        .add_template("layout.html", &layout_template)
        .expect("Failed to add layout.html template");
    engine
        .add_template("article.html", &article_template)
        .expect("Failed to add article.html template");

    let articles = get_articles();
    renderer::render_index(&engine, &articles);
    renderer::render_blog(&engine, &articles);
}

fn init_dist_dir() {
    let dist_path = Path::new(DIST_DIR);
    let static_path = Path::new(STATIC_DIR);
    let dist_blog_path = Path::new(DIST_BLOG_DIR);

    if dist_path.exists() {
        fs::remove_dir_all(dist_path).expect("Failed to clean dist directory");
    }

    create(dist_path, true).expect("Failed to create dist directory");
    create(dist_blog_path, true).expect("Failed to create dist/blog directory");
    copy_items(&[static_path], dist_path, &CopyOptions::new())
        .expect("Failed to copy static files to dist");
}
