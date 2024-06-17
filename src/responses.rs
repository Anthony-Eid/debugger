use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    /// Sequence number of the corresponding request
    request_seq: u64,

    /// Outcome of the request.
    /// If true, the request was successful and the `body` attribute may contain
    /// the result of the request.
    /// If the value is false, the attribute `message` contains the error in short
    /// form and the `body` may contain additional information (see
    /// `ErrorResponse.body.error`).
    success: bool,

    /// Requested command
    command: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    body: Option<ResponseType>,
}

#[derive(Debug, Deserialize, Serialize)]
enum ResponseType {}

#[derive(Debug, Deserialize, Serialize)]
struct InitializeResponse {}

#[derive(Debug, Deserialize, Serialize)]
struct Capabilities {}
