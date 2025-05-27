use async_std::sync::Arc;
use chat::FromServer;
use tokio::sync::broadcast;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::broadcast::Receiver;

use crate::connection::Outbound;

const CAPACITY: usize = 1000;

pub struct Group {
    name: String,                      // Why do we need the name here?
    sender: broadcast::Sender<String>, // Channel with string payload
}

impl Group {
    pub fn new(name: String) -> Self {
        let (tx, _) = broadcast::channel(CAPACITY);
        Group { name, sender: tx }
    }

    pub fn join(&self, outbound: Arc<Outbound>) {
        let receiver = self.sender.subscribe(); // subscribing to the channel for the group
                                                // Keep a async loop actively running for the outbound tcp connection and send any messages for the group
        async_std::task::spawn(handle_subscriber(self.name.clone(), receiver, outbound));
        // listen to the incoming messages on this channel
    }

    pub fn post(&self, message: String) {
        self.sender.send(message).expect("Should send the message");
    }
}

async fn handle_subscriber(
    group_name: String,
    mut receiver: Receiver<String>,
    outbound: Arc<Outbound>,
) {
    loop {
        let packet = match receiver.recv().await {
            Ok(message) => FromServer::Message {
                group_name: group_name.clone(),
                message,
            },
            Err(RecvError::Lagged(n)) => {
                FromServer::Error(format!("Dropped {} messages from {}.", n, group_name))
            }
            Err(RecvError::Closed) => break,
        };
        if outbound.send(packet).await.is_err() {
            break;
        }
    }
}
