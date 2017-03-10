pub fn hello(name: Option<&str>) -> String {
    match name {
        Some(x) => format!("Hello, {}!", x),
        None => "Hello, World!".to_string()
    }
}
