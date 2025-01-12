pub mod book;
pub mod index;

pub fn register_templates(handlebars: &mut handlebars::Handlebars) {
    handlebars
        .register_template_string("layout", include_str!("./layout.hbs"))
        .expect("Failed to register index template");
}
