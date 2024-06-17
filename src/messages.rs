use crate::{events, requests, responses};
use serde::{Deserialize, Serialize};

/// Base of all messages sent between the adapter & editor/debugger
#[derive(Debug, Deserialize, Serialize)]
struct ProtocolMessage {
    seq: u64,
    _type: MessageType,
}

#[derive(Debug, Deserialize, Serialize)]
enum MessageType {
    Request(requests::Request),
    Response(responses::Response),
    Event(),
}

pub fn get_full_message(body: &str) -> String {
    format!("Content-Length: {}\r\n\r\n{}", body.len(), body).to_string()
}

pub fn remove_message_header(message: &str) -> Result<&str, ()> {
    if let Some(len) = message.find("\r\n\r\n") {
        Ok(message.split_at(len).1)
    } else {
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_message_header() {
        let message = r#"Content-Length: 1245

            {"seq":1,"type":"response","request_seq":1,"command":"initialize","success":true,"body":{"supportsConfigurationDoneRequest":true,}"#;

        let body = remove_message_header(message);

        assert!(body.is_ok());

        assert_eq!(
            body.unwrap(),
            r#"{"seq":1,"type":"response","request_seq":1,"command":"initialize","success":true,"body":{"supportsConfigurationDoneRequest":true,}"#
        );
    }
}
