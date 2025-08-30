#![allow(dead_code)]

mod broker;
mod messages;
mod publisher;
mod subscriber;
mod utils;

use crate::{broker::Broker, publisher::Publisher, subscriber::Subscriber};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// the process role (publisher or subscriber)
    #[arg(short, long, name = "publisher/subscriber")]
    role: String,

    /// a topic to publish/subscribe
    #[arg(short, long)]
    topic: String,
}

fn main() {
    let args = Args::parse();

    if args.role == "subscriber" {
        let subs = Subscriber::new(args.topic);

        subs.check_me();
    } else if args.role == "publisher" {
        let topic = "messageService";

        let pubs = Publisher {
            topic: topic.to_string(),
        };

        pubs.send_message(format!("Hello, {}", topic));
    } else {
        let broker = Broker::new();

        broker.run();
    }
}
