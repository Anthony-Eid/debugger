// use serde::{Deserialize, Serialize};
// use std::{
//     error::Error,
//     io::{Read, Write},
//     net::TcpStream,
//     process::Stdio,
//     sync::atomic::AtomicU64,
//     time::Duration,
// };

// use futures::{AsyncRead, AsyncWrite};

// use anyhow::{anyhow, Context, Result};
// use async_process::{Child, ChildStdin, Command};

// use crate::{
//     configurations::{DebuggerConfig, RequestType},
//     messages,
//     requests::InitializeRequest,
// };

// const CONTENT_LEN_HEADER: &str = "Content-Length: ";

// const LSP_REQUEST_TIMEOUT: Duration = Duration::from_secs(60 * 2);
// const SERVER_SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(5);

// #[derive(Serialize, Deserialize, Debug)]
// struct AdapterResponse {
//     request_seq: i32,
//     success: bool,
//     command: String,
//     message: Option<String>,
// }

// enum ClientState {
//     Header,
//     Content,
// }

// struct Debugger<Stdin, Stdout, Stderr>
// where
//     Stdin: AsyncWrite + Unpin + Send + 'static,
//     Stdout: AsyncRead + Unpin + Send + 'static,
//     Stderr: AsyncRead + Unpin + Send + 'static,
// {
//     seq: AtomicU64,
//     configs: Option<DebuggerConfig>,
//     stream: Option<TcpStream>,
//     dap: Option<Child>,
//     input: Option<Stdin>,
//     output: Option<Stdout>,
//     errors: Option<Stderr>,
// }

// impl<
//         Stdin: AsyncWrite + Unpin + Send + 'static,
//         Stdout: AsyncRead + Unpin + Send + 'static,
//         Stderr: AsyncRead + Unpin + Send + 'static,
//     > Debugger<Stdin, Stdout, Stderr>
// {
//     fn new() -> Self {
//         Debugger {
//             seq: AtomicU64::new(1),
//             configs: DebuggerConfig::new().ok(),
//             stream: None,
//             dap: None,
//             input: None,
//             output: None,
//             errors: None,
//         }
//     }

//     fn init_debug_adapter(&mut self) -> Result<(), Box<dyn Error>> {
//         if self.configs.is_none() {
//             Err("No launch configuration found")?
//         }
//         let configs = self.configs.as_ref().unwrap();

//         match configs.request {
//             RequestType::Attach => todo!(),
//             RequestType::Launch => {
//                 let mut dap_server = Command::new(&configs.program_type)
//                     .arg(&configs.type_path)
//                     .stdin(Stdio::piped())
//                     .stdout(Stdio::piped())
//                     .stderr(Stdio::piped())
//                     .kill_on_drop(true)
//                     .spawn()?;

//                 self.input = dap_server.stdin.take().unwrap();
//                 let stdout = dap_server.stdout.take().unwrap();
//                 let stderr = dap_server.stderr.take().unwrap();
//             }
//         };

//         Ok(())
//     }

//     fn send_initial_request(&mut self) -> Result<(), Box<dyn Error>> {
//         if self.stream.is_none() {
//             Err("No Debug Adapter running to send initialization request too")?
//         }

//         let request = InitializeRequest::new("mock".to_string());
//         let json_str = serde_json::to_string(&request).expect("There should be information");
//         let message = messages::get_full_message(&json_str);
//         let stream = self.stream.as_mut().unwrap();

//         write!(stream, "{}", message).expect("Should be able to send content length");

//         // let mut buffer = [0; 1400];
//         // let size = stream.read(&mut buffer)?;

//         // let response_str = String::from_utf8_lossy(&buffer[..size]);

//         Ok(())
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use core::time;
//     use std::thread::sleep;

//     #[test]
//     fn test_request_handler() {
//         let mut debugger = Debugger::new();

//         dbg!(debugger.init_debug_adapter());
//         // assert!(debugger.init_debug_adapter().is_ok());

//         sleep(time::Duration::from_millis(1000));

//         assert!(debugger.send_initial_request().is_ok());

//         debugger.dap.unwrap().kill();
//     }
// }
