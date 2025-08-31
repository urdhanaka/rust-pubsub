use crate::utils;

pub struct Subscriber {
    /// Unique ID for subscriber
    id: String,

    /// Topic subscribed
    topic: String,

    /// Connection
    conn: String,
}

impl Subscriber {
    pub fn new(topic: String, conn: String) -> Self {
        let id = utils::random_string(10);

        Self { id, topic, conn }
    }
}
