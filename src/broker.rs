use crate::subscriber::Subscriber;
use std::{collections::HashMap, sync::Mutex};

struct Broker {
    /// Mutex for read/write
    lock: Mutex<bool>,

    /// Map of topics and the subscriber list
    topics: HashMap<String, Vec<Subscriber>>,
}

impl Broker {
    /// Add new topic
    pub fn new_topic(&mut self, topic: String) {
        // check if topic is already present
        if self.topics.contains_key(&topic) {
            return;
        }

        let curr_lock = self.lock.try_lock();
        if curr_lock.is_err() {
            println!("Could not get mutex lock: {}", curr_lock.err().unwrap())
        }

        self.topics.insert(topic, Vec::new());
    }

    /// Add subscriber to a topic that exists
    pub fn add_subscriber_to_topic(&mut self, topic: String, subs: Subscriber) {
        if !self.topics.contains_key(&topic) {
            println!("Topic is not available");
            return;
        }

        self.topics.get_mut(&topic).unwrap().push(subs);
    }
}
