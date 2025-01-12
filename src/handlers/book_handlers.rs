use axum::{
    debug_handler,
    extract::State,
    http::{header, StatusCode},
    response::Response,
    Router,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::json;
use std::sync::Arc;

pub fn merge_book_routes(router: Router) -> Router {
    router
        .route("/book/{book_id}", get(book_start_handler))
        .route("/book/{book_id}/page/{page_id}", get(book_page_handler))
}

use crate::{get_jwt_secret, models::user::Claims, AppState};

#[debug_handler]
pub async fn book_start_handler(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
    axum::extract::Path(book_id): axum::extract::Path<u32>,
) -> Response {
    // Check for valid auth cookie
    let mut authenticated = false;
    if let Some(cookie) = headers.get(header::COOKIE) {
        if let Some(cookie_str) = cookie.to_str().ok() {
            if let Some(token) = cookie_str
                .split(';')
                .find(|s| s.trim().starts_with("auth="))
                .and_then(|s| s.trim().strip_prefix("auth="))
            {
                if state.auth_service.validate_jwt(token).is_some() {
                    authenticated = true;
                }
            }
        }
    }

    if !authenticated {
        return Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(header::LOCATION, "/")
            .body("Redirecting...".into())
            .unwrap();
    }

    let is_htmx = headers.get("HX-Request").is_some();
    let book = state
        .book_service
        .get_book(&state.library, book_id)
        .expect("Book not found");

    let current_page = state
        .book_service
        .get_starting_page(book)
        .expect("Starting page not found");

    let data = json!({
        "title": book.title,
        "page": current_page,
        "book_id": book.id
    });

    if is_htmx {
        let rendered = state
            .handlebars
            .render("book_page", &data)
            .expect("Failed to render book page template");

        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/html")
            .body(rendered.into())
            .unwrap()
    } else {
        // Return full page for direct browser requests
        let mut full_data = json!({
            "title": book.title,
            "heading": book.title,
            "username": null,
            "state": {
                "library": &state.library
            }
        });

        // Add username if available
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
                        full_data["username"] = json!(token_data.claims.sub);
                    }
                }
            }
        }

        // Add the book page content to the main section
        let book_page_content = state
            .handlebars
            .render("book_page", &data)
            .expect("Failed to render book page template");
        full_data["main_content"] = json!(book_page_content);

        let rendered = state
            .handlebars
            .render("index", &full_data)
            .expect("Failed to render template");

        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/html")
            .body(rendered.into())
            .unwrap()
    }
}

pub async fn book_page_handler(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
    axum::extract::Path((book_id, page_id)): axum::extract::Path<(u32, u32)>,
) -> Response {
    // Check for valid auth cookie
    let mut authenticated = false;
    if let Some(cookie) = headers.get(header::COOKIE) {
        if let Some(cookie_str) = cookie.to_str().ok() {
            if let Some(token) = cookie_str
                .split(';')
                .find(|s| s.trim().starts_with("auth="))
                .and_then(|s| s.trim().strip_prefix("auth="))
            {
                if decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(&get_jwt_secret()),
                    &Validation::default(),
                )
                .is_ok()
                {
                    authenticated = true;
                }
            }
        }
    }

    if !authenticated {
        return Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header(header::LOCATION, "/")
            .body("Redirecting...".into())
            .unwrap();
    }

    let is_htmx = headers.get("HX-Request").is_some();
    let book = state
        .book_service
        .get_book(&state.library, book_id)
        .expect("Book not found");

    let current_page = state
        .book_service
        .get_page(book, page_id)
        .expect("Page not found");

    let data = json!({
        "title": book.title,
        "page": current_page,
        "book_id": book.id
    });

    if is_htmx {
        let rendered = state
            .handlebars
            .render("book_page", &data)
            .expect("Failed to render book page template");

        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/html")
            .body(rendered.into())
            .unwrap()
    } else {
        // Return full page for direct browser requests
        let mut full_data = json!({
            "title": book.title,
            "heading": book.title,
            "username": null,
            "state": {
                "library": &state.library
            }
        });

        // Add username if available
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
                        full_data["username"] = json!(token_data.claims.sub);
                    }
                }
            }
        }

        // Add the book page content to the main section
        let book_page_content = state
            .handlebars
            .render("book_page", &data)
            .expect("Failed to render book page template");
        full_data["main_content"] = json!(book_page_content);

        let rendered = state
            .handlebars
            .render("index", &full_data)
            .expect("Failed to render template");

        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/html")
            .body(rendered.into())
            .unwrap()
    }
}
