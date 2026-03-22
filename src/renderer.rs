use crate::blog::Article;
use minijinja::{Environment, context};
use std::fs;
use std::fs::read_to_string;
use std::path::Path;

pub(crate) const DIST_DIR: &str = "dist";
pub(crate) const DIST_BLOG_DIR: &str = "dist/blog";
pub(crate) const DIST_CV_DIR: &str = "dist/cv";
pub(crate) const STATIC_DIR: &str = "static";
pub(crate) const LAYOUT_TEMPLATE_PATH: &str = "templates/layout.html";
pub(crate) const ARTICLE_TEMPLATE_PATH: &str = "templates/article.html";
pub(crate) const CV_TEMPLATE_PATH: &str = "templates/cv.html";

const INDEX_TEMPLATE_PATH: &str = "templates/index.html";
const DIST_INDEX_PATH: &str = "dist/index.html";
const DIST_BLOG_INDEX_PATH: &str = "dist/blog/index.html";
const DIST_CV_INDEX_PATH: &str = "dist/cv/index.html";
const CV_PDF_PATH: &str = "/static/files/matheus-alves-cv.pdf";

pub(crate) fn render_index(engine: &Environment, articles: &[Article]) {
    let contents =
        read_to_string(Path::new(INDEX_TEMPLATE_PATH)).expect("Failed to read index.html template");
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

pub(crate) fn render_cv(engine: &Environment) {
    let template = engine
        .get_template("cv.html")
        .expect("Failed to get cv.html template");
    let rendered = template
        .render(context! {title => "Matheus Alves Resume", cv_pdf_path => CV_PDF_PATH})
        .expect("Failed to render cv template");
    fs::write(Path::new(DIST_CV_INDEX_PATH), rendered).expect("Failed to write dist/cv/index.html");
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

#[cfg(test)]
mod tests {
    use minijinja::{Environment, context};

    #[test]
    fn cv_page_embeds_pdf_and_keeps_fallback_link() {
        let pdf_path = "/static/files/matheus-alves-cv.pdf";
        let mut engine = Environment::new();
        engine
            .add_template("layout.html", include_str!("../templates/layout.html"))
            .expect("Failed to add layout template");
        engine
            .add_template("cv.html", include_str!("../templates/cv.html"))
            .expect("Failed to add cv template");
        let html = engine
            .get_template("cv.html")
            .expect("Failed to get cv template")
            .render(context! {title => "Matheus Alves Resume", cv_pdf_path => pdf_path})
            .expect("Failed to render cv template");

        assert!(html.contains("Matheus Alves Resume"));
        assert!(html.contains("View my resume here, or open the PDF in a new tab"));
        assert!(html.contains(r#"<iframe"#));
        assert!(html.contains(r#"src="/static/files/matheus-alves-cv.pdf""#));
        assert!(html.contains("Open PDF in New Tab"));
        assert!(html.contains(r#"target="_blank""#));
        assert!(
            html.contains(
                r#"<a href="/static/files/matheus-alves-cv.pdf">open the PDF directly</a>"#
            )
        );
    }
}
