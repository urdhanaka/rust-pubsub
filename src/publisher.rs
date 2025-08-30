pub struct Publisher {
    /// Topic subscribed
    pub topic: String,
}

impl Publisher {
    pub fn new(topic: String) -> Self {
        Self { topic }
    }

    pub fn send_message(&self, message: String) {
        println!("{}", message)
    }
}
