use async_std::prelude::*;
use std::error::Error;
use serde::{de::DeserializeOwned,Serialize};

pub type ChatError = Box<dyn Error + Send + Sync + 'static>;
pub type ChatResult<T> = Result<T, ChatError>;

/// Will be used by the server & client to send the packets
pub async fn send_as_json<S, P>(outbound: &mut S, packet: &P) -> ChatResult<()>
where 
    S: async_std::io::Write + Unpin,
    P: Serialize,
{
    let mut json = serde_json::to_string(packet)?;
    json.push('\n');
    outbound.write_all(json.as_bytes()).await?;
    Ok(())
}

/// P -> the data type of the packet
pub fn receive_as_json<S, P>(inbound: S) -> impl Stream<Item = ChatResult<P>> 
where S: async_std::io::BufRead + Unpin,
      P: DeserializeOwned,
{
    inbound
    .lines() // will split the inbound stream on the basis of \n
             // if no \n is found, there will be no message iterated 
    .map(|line_result| {
        let line = line_result?;
        let parsed = serde_json::from_str(&line)?;
        Ok(parsed)
    })
}

