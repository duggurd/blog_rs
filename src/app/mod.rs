use axum::extract::State;
use axum::response::Html;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;
use tera::Context;
use tera::Tera;

mod blog;
use blog::{blog, blog_by_slug};

mod navbar;
use navbar::navbar;

pub mod index;
use index::gen_index;

#[axum::debug_handler]
async fn index(State(state): State<Arc<Tera>>) -> Html<String> {
    let mut context = Context::new();
    context.insert("dummy", "value");

    let home = state.render("views/home.html", &context).unwrap();

    // let mut context = Context::new();
    // let navbar = navbar(State(state.clone())).await;

    // context.insert("view", &home);
    // context.insert("navbar", &navbar.0);
    // let rendered = state.render("views/index.html", &context).unwrap();

    gen_index(state, home).await
}

pub fn app_router(state: Arc<Tera>) -> Router {
    let app = Router::new()
        .route("/blog", get(blog))
        .route("/blog/:slug", get(blog_by_slug))
        .route("/", get(index))
        .route("/navbar", get(navbar))
        .with_state(state.clone());

    app
}
