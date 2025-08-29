pub struct Subscriber {
    /// Topic subscribed
    pub topic: String,
}

impl Subscriber {
    pub fn check_topic(&self) {
        println!("Topic subscribed: {}", self.topic)
    }

    pub fn change_topic(&mut self, new_topic: &str) {
        self.topic = new_topic.to_string()
    }
}
