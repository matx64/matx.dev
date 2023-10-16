mod posts;
mod templates;

use askama::Template;
use posts::{load_posts, render_posts, Post};
use std::{error::Error, fs, io, path::Path};
use templates::IndexTemplate;

pub fn main() -> Result<(), Box<dyn Error>> {
    let target_path = Path::new("./dist");

    if target_path.is_dir() {
        fs::remove_dir_all(target_path)?;
    }

    let posts = load_posts()?;

    render_posts(&posts)?;
    render_index(posts)?;

    copy_folder(Path::new("./static"), Path::new("./dist/static"))?;

    println!("\n✅ Website successfully generated in /dist folder.");

    Ok(())
}

fn render_index(posts: Vec<Post>) -> Result<(), Box<dyn Error>> {
    let index_template = IndexTemplate { posts };

    fs::write("./dist/index.html", index_template.render()?).expect("Unable to write.");

    Ok(())
}

fn copy_folder(src: &Path, dest: &Path) -> io::Result<()> {
    if src.is_dir() {
        // Create the destination directory if it doesn't exist
        fs::create_dir_all(dest)?;

        // Iterate over the entries in the source directory
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let entry_path = entry.path();
            let new_dest = dest.join(entry.file_name());

            if entry_path.is_dir() {
                // Recursively copy subdirectories
                copy_folder(&entry_path, &new_dest)?;
            } else {
                // Copy files
                fs::copy(&entry_path, &new_dest)?;
            }
        }
    }

    Ok(())
}
