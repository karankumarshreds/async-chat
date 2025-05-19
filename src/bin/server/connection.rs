use crate::group_table;
use async_std::prelude::*;
use async_std::io::BufReader;
use async_std::net;
use async_std::sync::{Arc, Mutex};

use chat::utils::{self, ChatResult};
use chat::{FromClient, FromServer};

pub async fn serve(socket: net::TcpStream, group_table: Arc<group_table::GroupTable>) -> ChatResult<()> {
    let outbound = Arc::new(Outbound::new(socket.clone())); /* why arc? */
    let buffered = BufReader::new(socket);
    let mut from_client = utils::receive_as_json(buffered);
    while let Some(request_result) = from_client.next().await {
        let request = request_result?;
        let _result: ChatResult<()> = match request {
            FromClient::Join { group_name } => {
                group_table
                    .get_or_create(group_name) // returns a group
                    .join(Arc::clone(&outbound)); // joins the group
                Ok(())
            },
            FromClient::Post { group_name, message } => {
                group_table
                    .get_or_create(group_name)
                    .post(message);
                Ok(())
            }
        };
    }
    
    Ok(())
}

pub struct Outbound(Mutex<net::TcpStream>);

impl Outbound {
    pub fn new(to_client: net::TcpStream) -> Self {
        Self(Mutex::new(to_client))
    }
    
    pub async fn send(&self, packet: FromServer) -> ChatResult<()> {
        let mut to_server = self.0.lock().await;
        utils::send_as_json(&mut *to_server, &packet).await?;
        Ok(())
    }
}