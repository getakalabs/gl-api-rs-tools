use actix_cors::Cors;

/// Returns cors setup
pub fn stage<T: ToString>(methods: &[T]) -> Cors {
    // Set bindings
    let binding = methods.iter().map(|s| s.to_string()).collect::<Vec<String>>();
    let methods: Vec<&str> = binding.iter().map(|s| s.as_str()).collect();

    // Return cors
    Cors::default()
        .allow_any_origin()
        .allowed_methods(methods)
        .allow_any_header()
        .max_age(3600)
}