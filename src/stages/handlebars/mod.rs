use handlebars::Handlebars as HBS;

const ASSET_PATH: &str = "./assets/templates";
const EXTENSION: &str = "hbs";

// Stage handlebar instance
pub fn stage() -> HBS<'static> {
    // Initialize handlebars
    let mut handlebars = HBS::new();

    // Register directories
    handlebars
        .register_templates_directory(EXTENSION, ASSET_PATH)
        .expect("Invalid template directory");

    // Return handlebars
    handlebars
}