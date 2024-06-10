use serde::{Deserialize, Serialize};

enum Request {
    InitializeRequest(InitializeRequestArguments),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeRequest {
    command: String,
    arguments: InitializeRequestArguments,
    _type: String,
    seq: i32,
}

impl InitializeRequest {
    pub fn new(adapter_id: String) -> Self {
        InitializeRequest {
            command: "initialize".to_string(),
            arguments: InitializeRequestArguments {
                client_id: Some("zed".to_string()),
                adapter_id: adapter_id.to_string(),
                path_format: Some("path".to_string()),
            },
            _type: "request".to_string(),
            seq: 1,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct InitializeRequestArguments {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id: Option<String>,
    adapter_id: String,
    path_format: Option<String>,
}
