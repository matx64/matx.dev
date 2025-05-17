mod blog;

use std::{fs, path::Path};
use blog::render_blog;
use fs_extra::{copy_items, dir::{create, CopyOptions}};
use minijinja::{context, Environment};

fn main() {
    let dist_path = Path::new("dist");
    let static_path = Path::new("static");

    create(dist_path, true).unwrap();
    copy_items(&[static_path], dist_path, &CopyOptions::new()).unwrap();

    let mut env = Environment::new();
    env.add_template("layout.html", include_str!("../templates/layout.html")).unwrap();
    env.add_template("post.html", include_str!("../templates/post.html")).unwrap();

    render_blog(&env);

    fs::write(Path::new("dist/index.html"), env.render_str(include_str!("../templates/index.html"), context! {title => "matx.dev"}).unwrap()).unwrap();
}

