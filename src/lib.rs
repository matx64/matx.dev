mod blog;
mod template;

use fs_extra::{
    copy_items,
    dir::{CopyOptions, create},
};
use std::path::Path;
use template::generate_website;

fn main() {
    init_dist();
    generate_website();
}

fn init_dist() {
    let dist_path = Path::new("dist");
    let static_path = Path::new("static");
    create(dist_path, true).unwrap();
    copy_items(&[static_path], dist_path, &CopyOptions::new()).unwrap();
}
