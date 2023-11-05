---
title: "Building a Custom Static Website Generator in Rust"
description: "How I built this website/blog in Rust"
date: "Oct 19, 2023"
---

In this blog post, we will walk through the development of a custom static website generator in Rust. This generator takes markdown blog posts as input and produces HTML files for your website. The generator's source code is organized into some modules, and we'll explore them step by step. Here's the main structure of the project:

```text
my_website/
├── src/
│   ├── lib.rs
│   ├── posts.rs
│   └── templates.rs
├── templates/
│   ├── index.html
│   └── post.html
├── blog-posts/
│   └── 2023-10-19-First-Post.md
└── static/
```

Let's start by creating the cargo project and adding the required dependencies:

```sh
cargo new my_website
cargo add askama -F markdown
cargo add serde, serde_yaml
```

### The /blog-posts files

The post files will be in this folder, and they are just regular markdown files but with a YAML header on top (constrained by `---`). This header isn't rendered in the final result and serves only for parsing some post metadata. Here's an example:

###### **`2023-10-19-Static-Blog-Tutorial.md`**
```text
---
title: "First Blog Post"
description: "An example blog post description"
date: "Oct 19, 2023"
---

~lorem ipsum~
```

### The /templates files

This folder stays at root level and our template engine, Askama, will check it for template files. Askama uses regular .html files and `{% %}` or `{{ }}` tags for handling dynamic stuff. My website has only 2 templates - index and post. For the sake of simplicity, I'll only show the important Askama parts for both.

###### **`index.html`**
```hbs
<div>
    <h1>Blog Posts</h1>
      <ul>
        {% for post in posts %}
        <details>
            <summary>
                <a href="blog/{{post.filename}}.html">{{ post.date }} - {{ post.title }}</a>
            </summary>
          <p>{{ post.description }}</p>
        </details>
        {% endfor %}
      </ul>
</div>
```

###### **`post.html`**
```hbs
<header>
  <h1>{{ title }}</h1>
  <p>{{ date }}</p>
  <h6>{{ description }}</h6>
</header>
<hr>
<main>
  {{ body|markdown }}
<main>
```

### The src/templates.rs Module

This module defines the Askama template structs for rendering the blog posts and the index page of the website: IndexTemplate and PostTemplate.

```rust
use crate::posts::Post;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub posts: Vec<Post>,
}

#[derive(Template)]
#[template(path = "post.html")]
pub struct PostTemplate<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub date: &'a str,
    pub body: &'a str,
}
```

### The src/posts.rs Module

This module handles the parsing and processing of blog posts. Let's start by importing all necessary modules and defining a Post struct:

```rust
use crate::templates::PostTemplate;
use askama::Template;
use std::fs;
use std::{error::Error, path::Path};

pub struct Post {
    pub title: String,
    pub description: String,
    pub date: String,
    pub body: String,
    pub filename: String,
}
```

Now let's add 2 helper functions:
- **read_posts():** Reads the contents of all .md files inside /blog-posts.
- **split_header_and_body():** Splits the file contents into header and body parts.

```rust
fn read_posts() -> Result<Vec<String>, Box<dyn Error>> {
    let posts_dir = fs::read_dir("blog-posts")?;
    let mut post_contents = Vec::new();

    for file in posts_dir {
        let contents = fs::read_to_string(file?.path())?;
        post_contents.push(contents);
    }

    Ok(post_contents)
}

fn split_header_and_body(contents: String) -> (String, String) {
    let mut header = String::new();
    let mut body = String::new();
    let mut is_header = false;
    let mut finished_header = false; // prevents more than 1 headers

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
```

We will now add the **`load_posts()`** function, which uses the 2 helper functions to read the files, parse the header and body and return an array of Posts.

```rust
pub fn load_posts() -> Result<Vec<Post>, Box<dyn Error>> {
    let post_contents = read_posts()?;
    let mut posts = Vec::new();

    for content in post_contents {
        let (header, body) = split_header_and_body(content);
        let header: serde_yaml::Value = serde_yaml::from_str(&header)?;

        let title = header["title"].as_str().unwrap().to_string();

        let post = Post {
            title: title.clone(),
            description: header["description"].as_str().unwrap().to_string(),
            date: header["date"].as_str().unwrap().to_string(),
            body,
            filename: title.replace(" ", "-"),
        };

        posts.push(post);
    }

    Ok(posts)
}
```

Lastly, we will add our main posts function: **`render_posts()`**. It iterates through the provided vector of blog post data, converts each post into HTML using a template, and then writes the HTML content to individual files inside `/dist/blog`.

```rust
pub fn render_posts(posts: &Vec<Post>) -> Result<(), Box<dyn Error>> {
    let target_path = Path::new("./dist/blog");
    fs::create_dir_all(target_path)?;

    for post in posts {
        let post_template = PostTemplate {
            title: &post.title,
            description: &post.description,
            date: &post.date,
            body: &post.body,
        };

        let file_path = format!("./dist/blog/{}.html", &post.filename);

        fs::write(file_path, post_template.render()?).expect("Unable to write.");
    }
    Ok(())
}
```

That's it! The **posts.rs** module is done and we are now able to handle blog posts. Let's move to the final puzzle piece of the project.

### The src/lib.rs Module

This module is the entry point of the application. I will just dump the code and explain later:

```rust
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
```

The **`main()`** function runs when the application starts and it calls the load_posts() function to load the blog posts, then generates the HTML files for each post and the index page using the render_posts() and render_index() functions, respectively. It also copies static assets from the static/ directory to the ./dist/static/ directory.

The copy_folder() function recursively copies files and directories from the static/ directory to ./dist/static/.