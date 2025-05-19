// use async_std::sync::Arc;
use serde::{Deserialize, Serialize};

pub mod utils;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum FromClient {
    Join { group_name: String },
    Post { group_name: String, message: String },
}

#[derive(Debug, Deserialize, Serialize)]
pub enum FromServer {
    Message { group_name: String, message: String },
    Error(String),
}

#[test]
fn test_fromclient_json() {
    let from_client = FromClient::Post {
        group_name: "test".to_string(),
        message: "message".to_string(),
    };
    let json = serde_json::to_string(&from_client).unwrap();
    assert_eq!(
        json,
        r#"{"Post":{"group_name":"test","message":"message"}}"#
    );
    assert_eq!(
        serde_json::from_str::<FromClient>(&json).unwrap(),
        from_client
    );
}
