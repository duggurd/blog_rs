use axum::{
    extract::{Path, State},
    response::Html,
};
use comrak::{markdown_to_html, Options};
use std::sync::Arc;
use tera::{Context, Tera};
use tokio::fs::{read_dir, read_to_string};

use super::index::gen_index;

pub async fn blog(State(state): State<Arc<Tera>>) -> Html<String> {
    let mut context = Context::new();
    let mut articles = Vec::new();

    let mut article_dir = read_dir("templates/articles").await.unwrap();

    while let Some(article) = article_dir.next_entry().await.unwrap() {
        articles.push(
            article
                .file_name()
                .into_string()
                .unwrap()
                .replace(".md", ""),
        );
    }

    context.insert("articles", &articles);

    let rendered = state.render("views/blog.html", &context).unwrap();

    gen_index(state, rendered).await
}

pub async fn render_md(path: String) -> String {
    let file = read_to_string(path).await.unwrap();

    markdown_to_html(file.as_str(), &Options::default())
}

pub async fn blog_by_slug(
    Path(slug): Path<String>,
    State(state): State<Arc<Tera>>,
) -> Html<String> {
    let base_path = "templates/articles/";

    let fp = format!("{}{}.md", base_path, slug);

    let blog_md = render_md(fp).await;

    gen_index(state, blog_md).await
}
