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
}
