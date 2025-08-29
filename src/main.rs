#![allow(dead_code)]

mod broker;
mod messages;
mod publisher;
mod subscriber;

use crate::{publisher::Publisher, subscriber::Subscriber};
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
        let mut subs = Subscriber { topic: args.topic };

        subs.check_topic();
        subs.change_topic("newMessageService");
        subs.check_topic();
    } else if args.role == "publisher" {
        let pubs = Publisher {
            topic: args.topic,
            messages: "Hello, World!".to_string(),
        };

        pubs.check_topic();
        pubs.check_message();
    }
}
