#![allow(dead_code)]

mod broker;
mod messages;
mod publisher;
mod subscriber;
mod utils;

use crate::{broker::Broker, publisher::Publisher, subscriber::Subscriber};
use clap::Parser;
use log::error;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// the process role (publisher or subscriber)
    #[arg(short, long, name = "publisher/subscriber/broker")]
    role: String,

    /// a topic to publish/subscribe
    #[arg(short, long)]
    topic: Option<String>,
}

fn main() {
    let args = Args::parse();

    // log instance
    env_logger::builder()
        .filter(None, log::LevelFilter::Info) // allow INFO as the lowest log level to be printed
        .init();

    if args.role == "subscriber" {
        let _subs = Subscriber::new(args.topic.unwrap());

        // subs.check_me();
    } else if args.role == "publisher" {
        let topic = "messageService";

        let pubs = Publisher {
            topic: topic.to_string(),
        };

        pubs.send_message(format!("Hello, {}", topic));
    } else if args.role == "broker" {
        let broker = Broker::new();

        let _ = broker.run();
    } else {
        error!("Role {} is not available", args.role)
    }
}
