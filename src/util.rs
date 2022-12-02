pub fn read_input(filename: &str) -> String {
    std::fs::read_to_string(std::path::Path::new("resources/").join(filename)).unwrap()
}

pub fn read_example(filename: &str) -> String {
    std::fs::read_to_string(std::path::Path::new("examples/").join(filename)).unwrap()
}
