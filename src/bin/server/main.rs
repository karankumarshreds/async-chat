use async_std::prelude::*;
use async_std::{net, sync::Arc, task};
use chat::utils::ChatResult;

mod connection;
mod group;
mod group_table;

fn main() -> ChatResult<()> {
    let port = std::env::args().nth(1).expect("Usage: 'server PORT'");
    task::block_on(start(port))?;
    Ok(())
}

async fn start(port: String) -> ChatResult<()> {
    let listener = net::TcpListener::bind(port).await?;
    let chat_group_table = Arc::new(group_table::GroupTable::new());
    let mut new_connections = listener.incoming();
    while let Some(socket_result) = new_connections.next().await {
        let client_socket = socket_result?;
        let groups = chat_group_table.clone();
        task::spawn(async {
            if let Err(err) = connection::serve(client_socket, groups).await {
                eprintln!("Error while serving: {}", err);
            }
        });
    }
    Ok(())
}
