use axum::{
    routing::get,
    Router,
    response::Html,
    extract::State,
    debug_handler,
};
use std::{collections::HashMap, sync::{atomic::{AtomicU32, Ordering}, Arc}};

#[tokio::main]
async fn main() {
    let counter = Arc::new(AtomicU32::new(0));
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/counter", get(counter_handler))
        .with_state(counter);

    println!("Server starting on http://localhost:3000");
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[debug_handler]
async fn counter_handler(State(counter): State<Arc<AtomicU32>>) -> String {
    let new_count = counter.fetch_add(1, Ordering::Relaxed) + 1;
    new_count.to_string()
}

#[debug_handler]
async fn index_handler(State(counter): State<Arc<AtomicU32>>) -> Html<String> {
    let signature = include_str!("../templates/signature.mustache");
    let mut compiler = mustache::compile_str(include_str!("../templates/index.mustache"))
        .expect("Failed to compile template");
    compiler.register_partial("signature", signature);
    
    let mut data = HashMap::new();
    data.insert("title", "HTMX Counter Demo");
    data.insert("heading", "HTMX Counter Demo");
    let count = counter.load(Ordering::Relaxed);
    let count_str = count.to_string();
    data.insert("count", &count_str);
    
    let rendered = compiler.render_to_string(&data)
        .expect("Failed to render template");

    Html(rendered)
}
