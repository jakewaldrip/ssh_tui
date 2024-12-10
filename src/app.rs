pub struct App {
    pub fact: String,
}

impl App {
    pub fn new() -> App {
        Self {
            fact: String::from("A fact would go here about kris because it's good"),
        }
    }
}
