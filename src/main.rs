use axum::{
    debug_handler,
    extract::{Form, State},
    http::{header, StatusCode},
    http::header::COOKIE,
    response::{Html, Response},
    routing::{get, post},
    Router,
};
use handlebars::Handlebars;
use serde::Deserialize;
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
        .route("/logout", post(logout_handler))
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
        let rendered = state
            .handlebars
            .render("logged_in", &data)
            .expect("Failed to render logged in template");

        let auth_cookie = json!({
            "logged_in": true,
            "username": form.username
        })
        .to_string();

        Response::builder()
            .status(StatusCode::OK)
            .header(
                header::SET_COOKIE,
                format!("auth={}; Path=/; HttpOnly", auth_cookie),
            )
            .header(header::CONTENT_TYPE, "text/html")
            .body(rendered.into())
            .unwrap()
    } else {
        let rendered = state
            .handlebars
            .render(
                "login",
                &json!({
                    "error": "Invalid username or password"
                }),
            )
            .expect("Failed to render login template");
        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/html")
            .body(rendered.into())
            .unwrap()
    }
}

async fn logout_handler(State(state): State<Arc<AppState>>) -> Response {
    let data = json!({});
    let rendered = state
        .handlebars
        .render("index", &data)
        .expect("Failed to render template");

    Response::builder()
        .status(StatusCode::OK)
        .header(
            header::SET_COOKIE,
            "auth=; Path=/; HttpOnly; Max-Age=0"
        )
        .header(header::CONTENT_TYPE, "text/html")
        .body(rendered.into())
        .unwrap()
}

async fn index_handler(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
) -> Html<String> {
    let count = state.counter.load(Ordering::Relaxed);

    let mut data = json!({
        "title": "HTMX Counter Demo",
        "heading": "HTMX Counter Demo",
        "count": count,
    });

    if let Some(cookie) = headers.get(COOKIE) {
        if let Some(cookie_str) = cookie.to_str().ok() {
            if let Some(auth_cookie) = cookie_str
                .split(';')
                .find(|s| s.trim().starts_with("auth="))
                .and_then(|s| s.trim().strip_prefix("auth="))
            {
                if let Ok(auth_data) = serde_json::from_str::<serde_json::Value>(auth_cookie) {
                    if auth_data["logged_in"].as_bool().unwrap_or(false) {
                        if let Some(username) = auth_data["username"].as_str() {
                            data["username"] = json!(username);
                        }
                    }
                }
            }
        }
    }

    let rendered = state
        .handlebars
        .render("index", &data)
        .expect("Failed to render template");

    Html(rendered)
}
