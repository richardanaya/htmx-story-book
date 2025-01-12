use axum::{
    debug_handler,
    extract::{Form, State},
    http::{header, StatusCode},
    response::Response,
    routing::post,
    Router,
};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

use crate::{models::user::UserCredentials, AppState};

#[derive(Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

pub fn register_templates(handlebars: &mut handlebars::Handlebars) {
    handlebars
        .register_template_string("login", include_str!("./login.hbs"))
        .expect("Failed to register login partial");
    handlebars
        .register_template_string("logged_in", include_str!("./logged_in.hbs"))
        .expect("Failed to register logged in template");
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

pub fn create_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/components/login", post(login_handler))
        .route("/components/logout", post(logout_handler))
}
