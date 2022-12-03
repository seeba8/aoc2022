/// # Panics
/// This may panic if there is a problem with the file
#[must_use] pub fn read_input(filename: &str) -> String {
    std::fs::read_to_string(std::path::Path::new("resources/").join(filename)).unwrap()
}

/// # Panics
/// This may panic if there is a problem with the file
#[must_use] pub fn read_example(filename: &str) -> String {
    std::fs::read_to_string(std::path::Path::new("examples/").join(filename)).unwrap()
}
