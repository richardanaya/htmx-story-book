use axum::{
    routing::{get, post},
    Router,
};
use dotenvy::dotenv;
use handlebars::Handlebars;
use std::env;
use std::sync::Arc;
use tower_http::services::ServeDir;

mod data;
mod handlers;
mod models;
mod services;

pub fn get_jwt_secret() -> Vec<u8> {
    dotenv().ok();
    env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set in .env file")
        .into_bytes()
}

use crate::models::book::Book;

pub struct AppState {
    handlebars: Handlebars<'static>,
    library: Vec<Book>,
    auth_service: Arc<services::auth_service::AuthService>,
    book_service: Arc<services::book_service::BookService>,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let mut handlebars = Handlebars::new();

    // Register templates
    handlebars
        .register_template_file("index", "templates/index.hbs")
        .expect("Failed to register index template");
    handlebars
        .register_template_file("login", "templates/login.hbs")
        .expect("Failed to register login partial");
    handlebars
        .register_template_file("logged_in", "templates/logged_in.hbs")
        .expect("Failed to register logged in template");
    handlebars
        .register_template_file(
            "non_logged_in_content",
            "templates/pages/non_logged_in_content.hbs",
        )
        .expect("Failed to register non logged in content template");
    handlebars
        .register_template_file("logged_in_content", "templates/pages/logged_in_content.hbs")
        .expect("Failed to register logged in content template");
    handlebars
        .register_template_file("book_page", "templates/book_page.hbs")
        .expect("Failed to register book page template");

    let state = Arc::new(AppState {
        handlebars,
        library: data::sample_data::generate_fake_library(),
        auth_service: Arc::new(services::auth_service::AuthService::new(get_jwt_secret())),
        book_service: Arc::new(services::book_service::BookService {}),
    });

    let app = Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .route("/", get(handlers::index_handler))
        .route("/login", post(handlers::login_handler))
        .route("/logout", post(handlers::logout_handler))
        .merge(handlers::book_handlers::merge_book_routes(Router::new()))
        .with_state(state);

    println!("Server starting on http://localhost:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
