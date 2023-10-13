mod posts;

use posts::{load_posts, render_posts};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let posts = load_posts()?;

    render_posts(posts)?;

    Ok(())
}
