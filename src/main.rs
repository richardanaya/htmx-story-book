use axum::{
    debug_handler,
    extract::{Form, State},
    http::{header, StatusCode},
    http::header::COOKIE,
    response::{Html, Response},
    routing::{get, post},
    Router,
};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use std::time::{SystemTime, UNIX_EPOCH};
use tower_http::services::ServeDir;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize}; 
use serde_json::json;
use std::sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
};

use dotenvy::dotenv;
use std::env;

fn get_jwt_secret() -> Vec<u8> {
    dotenv().ok();
    env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set in .env file")
        .into_bytes()
}

#[derive(Debug, Serialize, Deserialize)] 
struct Claims {
    sub: String,  // username
    exp: usize,   // expiration time
    iat: usize,   // issued at
}

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
        .nest_service("/static", ServeDir::new("static"))
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

fn validate_credentials(username: &str, password: &str) -> bool {
    // For now, we have a single hardcoded user
    // In a real application, this would check against a database
    username == "richard" && password == "secret"
}

#[debug_handler]
async fn login_handler(
    State(state): State<Arc<AppState>>,
    Form(form): Form<LoginForm>,
) -> Response {
    if validate_credentials(&form.username, &form.password) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
            
        let claims = Claims {
            sub: form.username.clone(),
            exp: now + 3600, // Token expires in 1 hour
            iat: now,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&get_jwt_secret())
        ).unwrap();

        let data = json!({
            "username": form.username,
            "error": null
        });
        
        let rendered = state
            .handlebars
            .render("logged_in", &data)
            .expect("Failed to render logged in template");

        Response::builder()
            .status(StatusCode::OK)
            .header(
                header::SET_COOKIE,
                format!("auth={}; Path=/; HttpOnly; SameSite=Strict", token),
            )
            .header(header::CONTENT_TYPE, "text/html")
            .header("HX-Trigger", "login-success")
            .header("HX-Refresh", "true")  // Add this header for full page refresh
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
    let count = state.counter.load(Ordering::Relaxed);
    let data = json!({
        "title": "Storybuilder",
        "heading": "Storybuilder",
        "count": count,
    });
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
        "title": "Storybuilder",
        "heading": "Storybuilder",
        "count": count,
    });

    if let Some(cookie) = headers.get(COOKIE) {
        if let Some(cookie_str) = cookie.to_str().ok() {
            if let Some(token) = cookie_str
                .split(';')
                .find(|s| s.trim().starts_with("auth="))
                .and_then(|s| s.trim().strip_prefix("auth="))
            {
                if let Ok(token_data) = decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(&get_jwt_secret()),
                    &Validation::default()
                ) {
                    data["username"] = json!(token_data.claims.sub);
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
