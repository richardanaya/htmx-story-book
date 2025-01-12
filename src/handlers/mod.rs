use axum::{
    debug_handler,
    extract::{Form, State},
    http::{header, StatusCode},
    response::{Html, Response},
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

pub mod book_handlers;

#[derive(Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[debug_handler]
pub async fn login_handler(
    State(state): State<Arc<AppState>>,
    Form(form): Form<LoginForm>,
) -> Response {
    let credentials = UserCredentials {
        username: form.username.clone(),
        password: form.password.clone(),
    };

    if state.auth_service.validate_credentials(&credentials) {
        let token = state.auth_service.create_jwt(&form.username);

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
            .header("HX-Refresh", "true")
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

pub async fn logout_handler(State(state): State<Arc<AppState>>) -> Response {
    let data = json!({
        "title": "Storybuilder",
        "heading": "Storybuilder",
    });
    let rendered = state
        .handlebars
        .render("index", &data)
        .expect("Failed to render template");

    Response::builder()
        .status(StatusCode::OK)
        .header(header::SET_COOKIE, "auth=; Path=/; HttpOnly; Max-Age=0")
        .header(header::CONTENT_TYPE, "text/html")
        .body(rendered.into())
        .unwrap()
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
