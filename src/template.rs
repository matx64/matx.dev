use crate::blog::{Article, get_articles};

use minijinja::{Environment, context};
use std::{
    fs::{self, create_dir_all},
    path::Path,
};

pub fn generate_website() {
    let mut env = Environment::new();
    env.add_template("layout.html", include_str!("../templates/layout.html"))
        .unwrap();
    env.add_template("post.html", include_str!("../templates/post.html"))
        .unwrap();

    let articles = get_articles();

    render_index(&env);
    render_blog(&env, &articles);
}

pub fn render_index(env: &Environment) {
    fs::write(
        Path::new("dist/index.html"),
        env.render_str(
            include_str!("../templates/index.html"),
            context! {title => "matx.dev"},
        )
        .unwrap(),
    )
    .unwrap();
}

pub fn render_blog(env: &Environment, articles: &[Article]) {
    create_dir_all("dist/blog").unwrap();

    for article in articles {
        let template = env.get_template("post.html").unwrap();
        let file_str = template
            .render(context! {title => "post", body => &article.body})
            .unwrap();

        let path = format!("dist/blog/{}.html", &article.slug);
        fs::write(Path::new(&path), file_str).unwrap();
    }
}
