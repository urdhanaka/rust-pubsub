pub struct Publisher {
    /// Topic subscribed
    pub topic: String,

    /// Broker connection information
    conn: String,
}

impl Publisher {
    pub fn new(topic: String, conn: String) -> Self {
        Self { topic, conn }
    }

    pub fn send_message(&self, message: String) {
        println!("{}", message)
    }
}
