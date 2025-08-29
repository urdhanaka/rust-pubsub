pub struct Publisher {
    /// Topic subscribed
    pub topic: String,

    /// Message to be published
    pub messages: String,
}

impl Publisher {
    pub fn check_topic(&self) {
        println!("Publishing topic: {}", self.topic)
    }

    pub fn check_message(&self) {
        println!("Publishing message: {}", self.messages)
    }
}
