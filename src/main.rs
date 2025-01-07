use axum::{
    debug_handler, 
    extract::{State, Form},
    response::{Html, Response}, 
    routing::{get, post}, 
    Router,
    http::{header, StatusCode}
};
use serde::Deserialize;
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
    handlebars
        .register_template_file("login", "templates/login.hbs")
        .expect("Failed to register login partial");
    handlebars
        .register_template_file("logged_in", "templates/logged_in.hbs")
        .expect("Failed to register logged in template");

    let state = Arc::new(AppState {
        counter: AtomicU32::new(0),
        handlebars,
    });

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/counter", get(counter_handler))
        .route("/login", post(login_handler))
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

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

#[debug_handler]
async fn login_handler(
    State(state): State<Arc<AppState>>,
    Form(form): Form<LoginForm>,
) -> Response {
    if form.username == "richard" && form.password == "secret" {
        let data = json!({
            "username": form.username,
        });
        let rendered = state.handlebars
            .render("logged_in", &data)
            .expect("Failed to render logged in template");
        
        let auth_cookie = json!({
            "logged_in": true,
            "username": form.username
        }).to_string();
        
        Response::builder()
            .status(StatusCode::OK)
            .header(
                header::SET_COOKIE,
                format!("auth={}; Path=/; HttpOnly", auth_cookie)
            )
            .header(header::CONTENT_TYPE, "text/html")
            .body(rendered.into())
            .unwrap()
    } else {
        let rendered = state.handlebars
            .render("login", &json!({
                "error": "Invalid username or password"
            }))
            .expect("Failed to render login template");
        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/html")
            .body(rendered.into())
            .unwrap()
    }
}

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
