use axum::{
    debug_handler,
    extract::{Form, State},
    http::{header, StatusCode},
    response::{Html, Response},
    routing::{get, post},
    Router,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

use crate::{
    get_jwt_secret,
    models::user::{Claims, UserCredentials},
    AppState,
};

pub mod book;

pub fn register_index_templates(handlebars: &mut handlebars::Handlebars) {
    handlebars
        .register_template_string("index", include_str!("./index.hbs"))
        .expect("Failed to register index template");
    handlebars
        .register_template_string(
            "non_logged_in_content",
            include_str!("./non_logged_in_content.hbs"),
        )
        .expect("Failed to register non logged in content template");
    handlebars
        .register_template_string("logged_in_content", include_str!("./logged_in_content.hbs"))
        .expect("Failed to register logged in content template");
}

pub fn create_routes() -> Router<Arc<AppState>> {
    Router::new().route("/", get(index_handler))
}

pub async fn index_handler(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
) -> Html<String> {
    let mut data = json!({
        "title": "Storybuilder",
        "heading": "Storybuilder",
        "state": {
            "library": &state.library
        }
    });

    if let Some(cookie) = headers.get(header::COOKIE) {
        if let Some(cookie_str) = cookie.to_str().ok() {
            if let Some(token) = cookie_str
                .split(';')
                .find(|s| s.trim().starts_with("auth="))
                .and_then(|s| s.trim().strip_prefix("auth="))
            {
                if let Ok(token_data) = decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(&get_jwt_secret()),
                    &Validation::default(),
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
