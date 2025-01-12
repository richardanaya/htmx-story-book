use axum::{extract::State, http::header, response::Html, routing::get, Router};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::json;
use std::sync::Arc;

use crate::{get_jwt_secret, models::user::Claims, AppState};

pub fn register_templates(handlebars: &mut handlebars::Handlebars) {
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
            "library": state.book_service.get_library()
        },
        "main_content": ""
    });

    let content_template = if let Some(cookie) = headers.get(header::COOKIE) {
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
                    "logged_in_content"
                } else {
                    "non_logged_in_content"
                }
            } else {
                "non_logged_in_content"
            }
        } else {
            "non_logged_in_content"
        }
    } else {
        "non_logged_in_content"
    };

    data["main_content"] = json!(state
        .handlebars
        .render(content_template, &data)
        .expect("Failed to render content template"));

    let rendered = state
        .handlebars
        .render("layout", &data)
        .expect("Failed to render template");

    Html(rendered)
}
