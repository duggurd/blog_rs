use axum::Router;
use std::error::Error;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

use tera::Tera;
mod app;

use app::app_router;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    let address = "127.0.0.1:1234";

    let tera = match Tera::new("./templates/**/*") {
        Ok(t) => {
            println!("Succesfully templated tera");
            println!("{:?}", t.get_template_names().collect::<Vec<&str>>());
            t
        }
        Err(e) => {
            println!("failed to create tera: {}", e);
            ::std::process::exit(1);
        }
    };

    let assets = ServeDir::new("./assets");

    let shared_state = Arc::new(tera);

    let app = Router::new()
        .with_state(shared_state.clone())
        .nest_service("/assets", assets)
        .nest("/", app_router(shared_state.clone()));

    let listener = TcpListener::bind("127.0.0.1:1234").await?;
    println!("listening on: {}", address);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
