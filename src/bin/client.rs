use async_std::prelude::*;
use async_std::{io, net, sync::Arc};
use chat::{utils, FromServer};

fn main() -> utils::ChatResult<()> {
    let address = std::env::args().nth(1).expect("Usage: client ADDRESS:PORT");
    async_std::task::block_on(start(address))?;
    Ok(())
}

async fn start(address: String) -> utils::ChatResult<()> {
    let server = net::TcpStream::connect(address).await?;
    server.set_nodelay(true)?;

    let to_server = send_commands(server.clone());
    let from_server = handle_replies(server);

    from_server.race(to_server).await?; // returns any future that resolves first
    Ok(())
}

async fn send_commands(mut to_server: net::TcpStream) -> utils::ChatResult<()> {
    println!(
        "Commands:\n\
        join GROUP\n\
        post GROUP MESSAGE...\n\
    "
    );
    let mut command_lines = io::BufReader::new(io::stdin()).lines();
    while let Some(line) = command_lines.next().await {
        let request = match parse_command(&line?) {
            Some(request) => request,
            None => continue,
        };
        println!("Final request: -> -> -> {:#?}", request);
        utils::send_as_json(&mut to_server, &request).await?;
        to_server.flush().await?;
    }
    Ok(())
}

async fn handle_replies(from_server: net::TcpStream) -> utils::ChatResult<()> {
    let buffer = async_std::io::BufReader::new(from_server);
    let mut reply_stream = utils::receive_as_json(Box::new(buffer));
    while let Some(reply) = reply_stream.next().await {
        match reply? {
            FromServer::Message {
                group_name,
                message,
            } => {
                println!("RECEIVED message from group {}: {}", group_name, message);
            }
            FromServer::Error(err) => {
                println!("Error from server: {}", err);
            }
        }
    }
    Ok(())
}

use chat::FromClient;

fn parse_command(line: &str) -> Option<FromClient> {
    println!("line: {line}");
    // let (group_name, message) = get_next_token(&line)?;
    let (token, next) = line.split_once(" ").expect("Should get something");
    let (group_name, message) = get_next_token(&next)?;
    println!("group_name: {group_name}, message: {message}");
    match token {
        "post" => {
            // let (group_name, message) = get_next_token(&line)?;
            return Some(FromClient::Post {
                group_name: group_name.into(),
                message: message.into(),
            });
        }
        "join" => {
            println!("Got the join command for group_name: {group_name}");
            return Some(FromClient::Join {
                group_name: group_name.into(),
            });
        }
        _ => unimplemented!(),
    }
}

fn get_next_token(input: &str) -> Option<(&str, &str)> {
    let leading = input.trim_start();

    if leading.is_empty() {
        return None;
    }

    match input.find(char::is_whitespace) {
        Some(space) => Some((&input[0..space], &input[space..].trim_start())),
        None => Some((input, "")),
    }
}
