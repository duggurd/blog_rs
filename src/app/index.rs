use axum::extract::State;
use axum::response::Html;
use std::sync::Arc;
use tera::{Context, Tera};

use super::navbar;

pub async fn gen_index(state: Arc<Tera>, html: String) -> Html<String> {
    let mut context = Context::new();

    let navbar = navbar(State(state.clone())).await.0;
    context.insert("view", &html);
    context.insert("navbar", &navbar);

    let rendered = state.render("views/index.html", &context).unwrap();

    Html(rendered)
}
