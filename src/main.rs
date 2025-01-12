use axum::Router;
use dotenvy::dotenv;
use handlebars::Handlebars;
use std::env;
use std::sync::Arc;
use tower_http::services::ServeDir;

mod components;
mod models;
mod pages;
mod services;

pub fn get_jwt_secret() -> Vec<u8> {
    dotenv().ok();
    env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set in .env file")
        .into_bytes()
}


pub struct AppState {
    handlebars: Handlebars<'static>,
    auth_service: Arc<services::auth_service::AuthService>,
    book_service: Arc<services::book_service::BookService>,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let mut handlebars = Handlebars::new();
    components::register_templates(&mut handlebars);
    pages::register_templates(&mut handlebars);
    pages::index::register_templates(&mut handlebars);
    pages::book::register_templates(&mut handlebars);

    let book_service = Arc::new(services::book_service::BookService::new());
    let state = Arc::new(AppState {
        handlebars,
        auth_service: Arc::new(services::auth_service::AuthService::new(get_jwt_secret())),
        book_service,
    });

    let app = Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .merge(components::create_routes())
        .merge(pages::index::create_routes())
        .merge(pages::book::create_routes())
        .with_state(state);

    println!("Server starting on http://localhost:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
