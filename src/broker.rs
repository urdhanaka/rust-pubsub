use crate::subscriber::Subscriber;
use anyhow::Result;
use log::{error, info};
use std::{
    collections::HashMap,
    error::Error,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

pub struct Broker {
    /// Map of topics and the subscriber's connection list
    topics: Arc<Mutex<HashMap<String, Vec<Subscriber>>>>,
}

impl Broker {
    pub fn new() -> Self {
        let topics = Arc::new(Mutex::new(HashMap::new()));

        Self { topics }
    }

    /// Add new topic
    pub fn new_topic(&mut self, topic: String) {
        // clone first
        let arc_clone = Arc::clone(&self.topics);
        let mut topic_map = arc_clone.lock().unwrap();

        // return early if topic already exist
        if topic_map.contains_key(&topic) {
            info!("Topic {} already exists", topic);
            return;
        }

        topic_map.insert(topic, Vec::new());
    }

    /// Add subscriber to a topic that exists
    pub fn add_subscriber_to_topic(&mut self, topic: String, subs: Subscriber) {
        // clone first
        let arc_clone = Arc::clone(&self.topics);
        let mut topic_map = arc_clone.lock().unwrap();

        // return early if topic doesn't exist
        if !topic_map.contains_key(&topic) {
            error!("Topic is not available");
            return;
        }

        let topic_vec = topic_map.get_mut(&topic).unwrap();
        topic_vec.push(subs)
    }

    #[tokio::main]
    pub async fn run(&self) -> Result<(), String> {
        let listener = TcpListener::bind("127.0.0.1:8080").await;
        info!("Broker is running on port :8080");

        Err("Broker run error".to_string())

        // loop {
        //     let (stream, addr) = listener.accept().await?;
        //
        //     tokio::spawn(async move {
        //         if let Err(e) = handle_client(stream, addr).await {
        //             error!("Client error: {}", e)
        //         }
        //     });
        // }
    }
}

async fn handle_client(stream: TcpStream, addr: SocketAddr) -> Result<()> {
    let (reader, writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut writer = writer;
    let mut line = String::new();

    loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line).await?;

        if bytes_read == 0 {
            break;
        }

        let parts: Vec<&str> = line.trim().splitn(3, ' ').collect();

        match parts.as_slice() {
            ["SUBSCRIBE", topic] => {
                info!("Getting subscriber for {} topic, conn: {:?}", topic, addr)
            }

            ["PUBLISHER", topic, message] => {
                info!(
                    "Getting publisher for {} topic, conn: {:?}, message: {}",
                    topic, addr, message
                )
            }

            _ => writer.write_all(b"Unknown command\n").await?,
        }
    }

    Ok(())
}
