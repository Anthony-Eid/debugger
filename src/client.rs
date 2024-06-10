use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::atomic::AtomicU64,
};

use crate::{configurations::DebuggerConfig, requests::InitializeRequest};

#[derive(Serialize, Deserialize, Debug)]
struct AdapterResponse {
    request_seq: i32,
    success: bool,
    command: String,
    message: Option<String>,
}

struct Debugger {
    seq: AtomicU64,
}

impl Debugger {
    fn new() -> Self {
        Debugger {
            seq: AtomicU64::new(1),
        }
    }

    fn send_initial_request(self, stream: &mut TcpStream) {
        let adapter_id = "7".to_string();
        let request = InitializeRequest::new(adapter_id);
        let json_str = serde_json::to_string(&request).expect("There should be information");
        let json_str =
            format!("Content-Length: {}\r\n\r\n{}", json_str.len(), json_str).to_string();

        println!("About to send {json_str} to DAP");

        write!(stream, "{}", json_str).expect("Should be able to send content length");

        let mut buffer = [0; 512];
        let size = stream
            .read(&mut buffer)
            .expect("Should be able to read from response");

        let response_str = String::from_utf8_lossy(&buffer[..size]);

        println!("Received response: {}", response_str);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use core::time;
    use std::{
        io::{BufReader, BufWriter},
        net::TcpListener,
        process::{Child, Command},
        thread::sleep,
    };

    fn start_debug_adapter() -> (Child, String) {
        // Find an available port
        let listener = TcpListener::bind("127.0.0.1:0").expect("Get random port to use");
        let port = listener
            .local_addr()
            .expect("There should be a local address")
            .port();

        // Ensure the listener is dropped so the server can use the port
        drop(listener);

        // Start the mock debug adapter with the dynamically assigned port
        (
            Command::new("node")
                .arg("/Users/eid/Developer/open-source/vscode-mock-debug/out/debugAdapter.js")
                .arg(format!("--server={}", port.to_string()))
                .spawn()
                .expect("Failed to start mock debug adapter"),
            port.to_string(),
        )
    }

    #[test]
    fn test_request_handler() {
        let (mut debug_process, port) = start_debug_adapter();
        sleep(time::Duration::from_secs(1));
        let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port))
            .expect("Should be able to connect because python server is running");

        let client = Debugger::new();
        client.send_initial_request(&mut stream);

        debug_process.kill().expect("Should be killable");
    }
}
