use anyhow::{Context, Result};
use futures::{AsyncRead, AsyncWrite, Future};
use gpui::{AsyncAppContext, BackgroundExecutor, Task};
use parking_lot::Mutex;
use postage::barrier;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use smol::{
    channel,
    process::{self, Child},
};

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process::Stdio,
    sync::{atomic::AtomicI32, Arc},
    time::Duration,
};

use crate::configurations::{DebuggerConfig, RequestType};

const _CONTENT_LEN_HEADER: &str = "Content-Length: ";
const _DAP_REQUEST_TIMEOUT: Duration = Duration::from_secs(60 * 2);
const _SERVER_SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(5);

type NotificationHandler = Box<dyn Send + FnMut(Option<RequestId>, Value, AsyncAppContext)>;
type ResponseHandler = Box<dyn Send + FnOnce(Result<String, Error>)>;
type IoHandler = Box<dyn Send + FnMut(IoKind, &str)>;

#[derive(Debug, Clone, Copy)]
pub enum IoKind {
    StdOut,
    StdIn,
    StdErr,
}

pub struct DebugAdapter {
    server_id: DebugAdapterId,
    next_id: AtomicI32,
    outbound_tx: channel::Sender<String>,
    name: Arc<str>,
    // capabilities: ServerCapabilities,
    // notification_handlers: Arc<Mutex<HashMap<&'static str, NotificationHandler>>>,
    response_handlers: Arc<Mutex<Option<HashMap<RequestId, ResponseHandler>>>>,
    // io_handlers: Arc<Mutex<HashMap<i32, IoHandler>>>,
    executor: BackgroundExecutor,
    #[allow(clippy::type_complexity)]
    // io_tasks: Mutex<Option<(Task<Option<()>>, Task<Option<()>>)>>,
    output_done_rx: Mutex<Option<barrier::Receiver>>,
    root_path: PathBuf,
    // working_dir: PathBuf,
    server: Arc<Mutex<Option<Child>>>,
}

impl DebugAdapter {
    fn new(
        stderr_capture: Arc<Mutex<Option<String>>>,
        server_id: DebugAdapterId,
        root_path: &Path,
        cx: AsyncAppContext,
    ) -> Result<Self> {
        let config = DebuggerConfig::new()?;

        match config.request {
            RequestType::Attach => todo!(),
            RequestType::Launch => {
                let mut dap_server = process::Command::new(&config.program_type)
                    .arg(&config.type_path)
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .kill_on_drop(true)
                    .spawn()
                    .with_context(|| "Failed to spawn Debug Adapter")?;

                let stdout = dap_server.stdout.take().unwrap();
                let stdin = dap_server.stdin.take().unwrap();
                let stderr = dap_server.stderr.take().unwrap();

                Ok(Self::new_internal(
                    server_id,
                    stdin,
                    stdout,
                    Some(stderr),
                    stderr_capture,
                    Some(dap_server),
                    root_path,
                    cx,
                ))
            }
        }
    }

    fn new_internal<Stdin, Stdout, Stderr>(
        server_id: DebugAdapterId,
        stdin: Stdin,
        stdout: Stdout,
        stderr: Option<Stderr>,
        stderr_capture: Arc<Mutex<Option<String>>>,
        server: Option<Child>,
        root_path: &Path,
        cx: AsyncAppContext,
    ) -> Self
    where
        Stdin: AsyncWrite + Unpin + Send + 'static,
        Stdout: AsyncRead + Unpin + Send + 'static,
        Stderr: AsyncRead + Unpin + Send + 'static,
    {
        let (outbound_tx, outbound_rx) = channel::unbounded::<String>();
        let (output_done_tx, output_done_rx) = barrier::channel();

        let response_handlers =
            Arc::new(Mutex::new(Some(HashMap::<_, ResponseHandler>::default())));

        let stdout_input_task = cx.spawn({
            let response_handlers = response_handlers.clone();
            move |cx| Self::handle_input(stdout, response_handlers, cx)
        });

        // let input_task = cx.spawn(|_| async move {
        //     let (stdout, stderr) = futures::join!(stdout_input_task, stderr_input_task);
        //     stdout.or(stderr)
        // });

        let output_task = cx.background_executor().spawn({
            Self::handle_output(
                stdin,
                outbound_rx,
                output_done_tx,
                response_handlers.clone(),
            )
        });

        Self {
            server_id,
            response_handlers,
            name: "".into(),
            next_id: Default::default(),
            outbound_tx,
            executor: cx.background_executor().clone(),
            output_done_rx: Mutex::new(Some(output_done_rx)),
            root_path: root_path.to_path_buf(),
            server: Arc::new(Mutex::new(server)),
        }
    }

    async fn handle_output<Stdin>(
        stdin: Stdin,
        outbound_rx: channel::Receiver<String>,
        output_done_tx: barrier::Sender,
        response_handlers: Arc<Mutex<Option<HashMap<RequestId, ResponseHandler>>>>,
    ) -> Result<()>
    where
        Stdin: AsyncWrite + Unpin + Send + 'static,
    {
        todo!();
    }

    async fn handle_input<Stdout>(
        stdin: Stdout,
        response_handlers: Arc<Mutex<Option<HashMap<RequestId, ResponseHandler>>>>,
        cx: AsyncAppContext,
    ) -> Result<()>
    where
        Stdout: AsyncRead + Unpin + Send + 'static,
    {
        todo!();
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DebugAdapterId(pub usize);

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestId {
    Int(i32),
    Str(String),
}

#[derive(Debug, Serialize, Deserialize)]
struct Error {
    message: String,
}
