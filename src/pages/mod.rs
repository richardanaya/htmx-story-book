pub mod book;
pub mod index;

pub fn register_templates(handlebars: &mut handlebars::Handlebars) {
    handlebars
        .register_template_string("index", include_str!("./index.hbs"))
        .expect("Failed to register index template");
}
