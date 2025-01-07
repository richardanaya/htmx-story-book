use axum::{
    routing::get,
    Router,
    response::Html,
    extract::State,
    debug_handler,
};
use std::{collections::HashMap, sync::{atomic::{AtomicU32, Ordering}, Arc, Mutex}};

struct AppState {
    counter: AtomicU32,
    template: Mutex<mustache::Template>,
}

#[tokio::main]
async fn main() {
    // Create and compile template at startup
    let template = mustache::compile_file("templates/index.mustache")
        .expect("Failed to compile template");
    
    // Register the partial template
    template.register_template_file("signature", "templates/signature.mustache")
        .expect("Failed to register partial");
    
    let state = Arc::new(AppState {
        counter: AtomicU32::new(0),
        template: Mutex::new(template),
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
    let mut data = HashMap::new();
    data.insert("title", "HTMX Counter Demo");
    data.insert("heading", "HTMX Counter Demo");
    let count = state.counter.load(Ordering::Relaxed);
    let count_str = count.to_string();
    data.insert("count", &count_str);
    
    let template = state.template.lock().unwrap();
    let rendered = template.render_to_string(&data)
        .expect("Failed to render template");

    Html(rendered)
}
