use axum::{debug_handler, extract::State, response::Html, routing::get, Router};
use handlebars::Handlebars;
use serde_json::json;
use std::sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
};

struct AppState {
    counter: AtomicU32,
    handlebars: Handlebars<'static>,
}

#[tokio::main]
async fn main() {
    let mut handlebars = Handlebars::new();
    
    // Register templates
    handlebars
        .register_template_file("index", "templates/index.hbs")
        .expect("Failed to register index template");
    handlebars
        .register_template_file("signature", "templates/signature.hbs")
        .expect("Failed to register signature partial");

    let state = Arc::new(AppState {
        counter: AtomicU32::new(0),
        handlebars,
    });

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/counter", get(counter_handler))
        .with_state(state);

    println!("Server starting on http://localhost:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[debug_handler]
async fn counter_handler(State(state): State<Arc<AppState>>) -> String {
    let new_count = state.counter.fetch_add(1, Ordering::Relaxed) + 1;
    new_count.to_string()
}

#[debug_handler]
async fn index_handler(State(state): State<Arc<AppState>>) -> Html<String> {
    let count = state.counter.load(Ordering::Relaxed);
    
    let data = json!({
        "title": "HTMX Counter Demo",
        "heading": "HTMX Counter Demo",
        "count": count,
    });

    let rendered = state.handlebars
        .render("index", &data)
        .expect("Failed to render template");

    Html(rendered)
}
