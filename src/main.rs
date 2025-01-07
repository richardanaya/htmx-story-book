use axum::{
    routing::get,
    Router,
    response::Html,
    extract::State,
};
use std::{collections::HashMap, sync::atomic::{AtomicU32, Ordering}};

#[tokio::main]
async fn main() {
    let counter = AtomicU32::new(0);
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/counter", get(counter_handler))
        .with_state(counter);

    println!("Server starting on http://localhost:3000");
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn counter_handler(State(counter): State<AtomicU32>) -> String {
    let new_count = counter.fetch_add(1, Ordering::Relaxed) + 1;
    new_count.to_string()
}

async fn index_handler(State(counter): State<AtomicU32>) -> Html<String> {
    let template = mustache::compile_str(include_str!("../templates/index.mustache"))
        .expect("Failed to compile template");

    let mut data = HashMap::new();
    data.insert("title", "HTMX Counter Demo");
    data.insert("heading", "HTMX Counter Demo");
    data.insert("count", &counter.load(Ordering::Relaxed).to_string());

    let rendered = template.render_to_string(&data)
        .expect("Failed to render template");

    Html(rendered)
}
