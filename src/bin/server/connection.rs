use crate::group_table;
use async_std::io::BufReader;
use async_std::net;
use async_std::prelude::*;
use async_std::sync::Arc;

use async_std::sync::Mutex;
use chat::utils::{self, ChatResult};
use chat::{FromClient, FromServer};

pub async fn serve(
    client_socket: net::TcpStream,
    group_table: Arc<group_table::GroupTable>,
) -> ChatResult<()> {
    let outbound = Outbound::new(client_socket.clone()); // regular clone
    let outbound = Arc::new(outbound);
    let mut from_client = utils::receive_as_json(BufReader::new(client_socket));
    while let Some(request_result) = from_client.next().await {
        let request = request_result?;
        let _result: ChatResult<()> = match request {
            FromClient::Join { group_name } => {
                group_table
                    .get_or_create(group_name) // returns a group
                    .join(Arc::clone(&outbound)); // joins the group
                Ok(())
            }
            FromClient::Post {
                group_name,
                message,
            } => {
                group_table.get_or_create(group_name).post(message);
                Ok(())
            }
        };
    }

    Ok(())
}

pub struct Outbound(Mutex<net::TcpStream>);

impl Outbound {
    pub fn new(to_server: net::TcpStream) -> Self {
        Self(Mutex::new(to_server))
    }

    pub async fn send(&self, packet: FromServer) -> ChatResult<()> {
        let mut to_server = self.0.lock().await; // Arc clone: in case there are more than 1 person
                                                 // trying to send the message to the same socket at
                                                 // the same time, otherwise it would create absursion
                                                 // in the json message
        utils::send_as_json(&mut *to_server, &packet).await?;
        to_server.flush().await?;
        Ok(())
    }
}
