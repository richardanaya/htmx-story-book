use axum::{
    routing::get,
    Router,
    response::Html,
};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index_handler));

    println!("Server starting on http://localhost:3000");
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index_handler() -> Html<String> {
    let template = mustache::compile_str(include_str!("../templates/index.mustache"))
        .expect("Failed to compile template");

    let mut data = HashMap::new();
    data.insert("title", "Welcome");
    data.insert("heading", "Hello from Mustache!");
    data.insert("message", "This is a simple template example.");

    let rendered = template.render_to_string(&data)
        .expect("Failed to render template");

    Html(rendered)
}
