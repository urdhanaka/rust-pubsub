use crate::utils;

pub struct Subscriber {
    /// Unique ID for subscriber
    pub id: String,

    /// Topic subscribed
    pub topic: String,

    /// Connection
    pub conn: String,
}

impl Subscriber {
    pub fn new(topic: String) -> Self {
        let id = utils::random_string(10);
        let conn = "asdlasd".to_string();

        Self { id, topic, conn }
    }

    pub fn check_me(&self) {
        println!("ID: {}, topic: {}", self.id, self.topic)
    }

    pub fn check_topic(&self) {
        println!("Topic subscribed: {}", self.topic)
    }

    pub fn change_topic(&mut self, new_topic: &str) {
        self.topic = new_topic.to_string()
    }
}
