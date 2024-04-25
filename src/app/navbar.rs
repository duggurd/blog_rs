use axum::{extract::State, response::Html};
use std::collections::HashMap;
use std::sync::Arc;
use tera::{Context, Tera};

pub async fn navbar(State(state): State<Arc<Tera>>) -> Html<String> {
    let mut navs: HashMap<&str, &str> = HashMap::new();
    navs.insert("home", "/");
    navs.insert("blog", "/blog");
    // navs.insert("home", "/");
    // navs.insert("home", "/");

    let mut context = Context::new();

    context.insert("navs", &navs);

    let rendered = state.render("views/navbar.html", &context).unwrap();

    Html(rendered)
}
