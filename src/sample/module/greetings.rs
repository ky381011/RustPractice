pub fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub mod formal {
    pub fn hello(name: &str) -> String {
        format!("Good day, {}.", name)
    }
}
