use std::sync::Arc;

use axum::Router;

use crate::AppState;

pub mod login;

pub fn register_templates(handlebars: &mut handlebars::Handlebars) {
    login::register_templates(handlebars);
}

pub fn create_routes() -> Router<Arc<AppState>> {
    Router::new().merge(login::create_routes())
}
