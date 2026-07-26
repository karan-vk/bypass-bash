use portable_pty::{Child, ChildKiller, CommandBuilder, MasterPty, PtySize, native_pty_system};
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::*;
use rmcp::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::process::Stdio;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use tokio::process::Command;
use tokio::time::{Duration, sleep, timeout};

/// Execution output returned by the one-shot `shell` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ShellOutput {
    /// Standard output produced by the command.
    pub stdout: String,
    /// Standard error produced by the command.
    pub stderr: String,
    /// Exit code of the process (0 for success, non-zero for error, -1 on timeout/spawn failure).
    pub returncode: i32,
}

/// Arguments for the one-shot `shell` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ShellArgs {
    /// The bash script or command string to execute.
    pub script: String,
    /// Optional working directory for script execution.
    pub cwd: Option<String>,
    /// Optional timeout in seconds (default: 300 seconds).
    pub timeout_secs: Option<u64>,
    /// Optional environment variables to set for the process.
    pub env: Option<HashMap<String, String>>,
}

/// Arguments for `shell_start`: launch a persistent interactive PTY session.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct StartArgs {
    /// Optional command to run in the session (parsed by bash, e.g. "python3",
    /// "psql mydb", "ssh host"). When omitted, an interactive bash shell is started.
    pub command: Option<String>,
    /// Optional working directory for the session.
    pub cwd: Option<String>,
    /// Optional environment variables to set for the session.
    pub env: Option<HashMap<String, String>>,
    /// Terminal width in columns (default: 120).
    pub cols: Option<u16>,
    /// Terminal height in rows (default: 30).
    pub rows: Option<u16>,
}

/// Result of `shell_start`.
#[derive(Debug, Serialize, JsonSchema)]
pub struct StartOutput {
    /// Identifier used to address this session in later calls.
    pub session_id: String,
    /// The command backing the session.
    pub command: String,
}

/// Arguments for `shell_write`: send input to a running session.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WriteArgs {
    /// Identifier of the target session.
    pub session_id: String,
    /// Text to send to the session's stdin. Control characters are honored, so
    /// "\u{0003}" sends Ctrl-C and "\u{0004}" sends Ctrl-D (EOF).
    pub input: String,
    /// Append a newline (Enter) after the input (default: true).
    pub enter: Option<bool>,
}

/// Arguments for `shell_read`: drain output produced by a session.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReadArgs {
    /// Identifier of the target session.
    pub session_id: String,
    /// Maximum time in milliseconds to wait for new output before returning
    /// (default: 2000). Returns early once output arrives and briefly settles.
    pub timeout_ms: Option<u64>,
}

/// Output returned by `shell_read`.
#[derive(Debug, Serialize, JsonSchema)]
pub struct ReadOutput {
    /// Identifier of the session.
    pub session_id: String,
    /// New output captured since the previous read.
    pub output: String,
    /// Whether the underlying process is still running.
    pub running: bool,
    /// Exit code once the process has finished (null while still running).
    pub returncode: Option<i32>,
}

/// Arguments addressing a single session by id.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SessionRef {
    /// Identifier of the target session.
    pub session_id: String,
}

/// Arguments for `shell_resize`: change a session's terminal dimensions.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ResizeArgs {
    /// Identifier of the target session.
    pub session_id: String,
    /// New terminal width in columns.
    pub cols: u16,
    /// New terminal height in rows.
    pub rows: u16,
}

/// Summary of a session for `shell_list`.
#[derive(Debug, Serialize, JsonSchema)]
pub struct SessionInfo {
    /// Identifier of the session.
    pub session_id: String,
    /// The command backing the session.
    pub command: String,
    /// Whether the underlying process is still running.
    pub running: bool,
    /// Exit code once the process has finished (null while still running).
    pub returncode: Option<i32>,
}

/// A live interactive PTY session.
struct Session {
    command: String,
    master: Mutex<Box<dyn MasterPty + Send>>,
    writer: Mutex<Box<dyn Write + Send>>,
    child: Mutex<Box<dyn Child + Send + Sync>>,
    killer: Mutex<Box<dyn ChildKiller + Send + Sync>>,
    /// Full output history captured by the background reader thread.
    buffer: Arc<Mutex<Vec<u8>>>,
    /// Offset into `buffer` up to which output has already been returned.
    cursor: Mutex<usize>,
}

impl Session {
    /// Returns the exit code if the process has finished, otherwise `None`.
    fn returncode(&self) -> Option<i32> {
        let mut child = self.child.lock().unwrap();
        match child.try_wait() {
            Ok(Some(status)) => Some(status.exit_code() as i32),
            _ => None,
        }
    }
}

/// High-performance Shell MCP Server handler.
#[derive(Clone)]
pub struct ShellMcpServer {
    // Read by the `#[tool_handler]` macro to dispatch tool calls.
    #[allow(dead_code)]
    tool_router: ToolRouter<Self>,
    sessions: Arc<Mutex<HashMap<String, Arc<Session>>>>,
    next_id: Arc<AtomicU64>,
}

impl Default for ShellMcpServer {
    fn default() -> Self {
        Self::new()
    }
}

impl ShellMcpServer {
    /// Construct a new server with an empty session table.
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
            sessions: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(AtomicU64::new(1)),
        }
    }

    /// Look up a session by id, returning an MCP error if it does not exist.
    fn session(&self, id: &str) -> Result<Arc<Session>, ErrorData> {
        self.sessions
            .lock()
            .unwrap()
            .get(id)
            .cloned()
            .ok_or_else(|| ErrorData::invalid_params(format!("unknown session_id: {id}"), None))
    }
}

fn json_result<T: Serialize>(value: &T) -> Result<CallToolResult, ErrorData> {
    let json_str = serde_json::to_string_pretty(value).unwrap_or_default();
    Ok(CallToolResult::success(vec![ContentBlock::text(json_str)]))
}

#[tool_router]
impl ShellMcpServer {
    /// Runs a string as a bash shell script (one-shot, non-interactive).
    #[tool(
        description = "Runs a string as a bash shell script and returns its stdout, stderr, \
                          and exit code once it finishes. Use this for non-interactive commands."
    )]
    async fn shell(
        &self,
        Parameters(args): Parameters<ShellArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        let output = Self::execute_script(args).await;
        json_result(&output)
    }

    /// Starts a persistent interactive shell session backed by a real PTY.
    #[tool(
        description = "Starts a persistent interactive shell session backed by a real \
                          pseudo-terminal (PTY). Programs such as REPLs (python3, node), \
                          database clients (psql), ssh, and full-screen TUIs behave as they \
                          would in a normal terminal. Returns a session_id used with \
                          shell_write, shell_read, shell_list, and shell_kill. When 'command' \
                          is omitted an interactive bash shell is started."
    )]
    async fn shell_start(
        &self,
        Parameters(args): Parameters<StartArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        let size = PtySize {
            rows: args.rows.unwrap_or(30),
            cols: args.cols.unwrap_or(120),
            pixel_width: 0,
            pixel_height: 0,
        };

        let pair = native_pty_system()
            .openpty(size)
            .map_err(|e| ErrorData::internal_error(format!("failed to open pty: {e}"), None))?;

        let command_label = args.command.clone().unwrap_or_else(|| "bash".to_string());

        let mut cmd = CommandBuilder::new("bash");
        if let Some(ref command) = args.command {
            cmd.arg("-c");
            cmd.arg(command);
        }
        if let Some(ref cwd) = args.cwd {
            cmd.cwd(cwd);
        }
        // Advertise a capable terminal so TUIs render; callers may override.
        cmd.env("TERM", "xterm-256color");
        if let Some(ref env_vars) = args.env {
            for (k, v) in env_vars {
                cmd.env(k, v);
            }
        }

        let child = pair
            .slave
            .spawn_command(cmd)
            .map_err(|e| ErrorData::internal_error(format!("failed to spawn: {e}"), None))?;
        // Slave is retained by the child; drop our handle so EOF propagates on exit.
        drop(pair.slave);

        let killer = child.clone_killer();
        let writer = pair
            .master
            .take_writer()
            .map_err(|e| ErrorData::internal_error(format!("failed to take writer: {e}"), None))?;
        let mut reader = pair
            .master
            .try_clone_reader()
            .map_err(|e| ErrorData::internal_error(format!("failed to clone reader: {e}"), None))?;

        let buffer = Arc::new(Mutex::new(Vec::<u8>::new()));
        let reader_buffer = Arc::clone(&buffer);
        // PTY reads are blocking, so drain them on a dedicated OS thread.
        std::thread::spawn(move || {
            let mut chunk = [0u8; 8192];
            loop {
                match reader.read(&mut chunk) {
                    Ok(0) | Err(_) => break,
                    Ok(n) => reader_buffer.lock().unwrap().extend_from_slice(&chunk[..n]),
                }
            }
        });

        let id = format!("sh-{}", self.next_id.fetch_add(1, Ordering::Relaxed));
        let session = Arc::new(Session {
            command: command_label.clone(),
            master: Mutex::new(pair.master),
            writer: Mutex::new(writer),
            child: Mutex::new(child),
            killer: Mutex::new(killer),
            buffer,
            cursor: Mutex::new(0),
        });
        self.sessions.lock().unwrap().insert(id.clone(), session);

        json_result(&StartOutput {
            session_id: id,
            command: command_label,
        })
    }

    /// Sends input to a running interactive session.
    #[tool(
        description = "Sends input to a running interactive session's stdin. By default a \
                          newline (Enter) is appended; set 'enter' to false to send keystrokes \
                          without a newline. Control characters are honored: send \"\\u0003\" \
                          for Ctrl-C or \"\\u0004\" for Ctrl-D (EOF). Follow with shell_read to \
                          see the resulting output."
    )]
    async fn shell_write(
        &self,
        Parameters(args): Parameters<WriteArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        let session = self.session(&args.session_id)?;
        let mut payload = args.input.into_bytes();
        if args.enter.unwrap_or(true) {
            payload.push(b'\n');
        }
        {
            let mut writer = session.writer.lock().unwrap();
            writer
                .write_all(&payload)
                .and_then(|_| writer.flush())
                .map_err(|e| ErrorData::internal_error(format!("failed to write: {e}"), None))?;
        }
        json_result(&serde_json::json!({
            "session_id": args.session_id,
            "bytes_written": payload.len(),
        }))
    }

    /// Reads output produced by a session since the previous read.
    #[tool(
        description = "Reads output produced by an interactive session since the previous \
                          read. Waits up to 'timeout_ms' (default 2000) for new output, \
                          returning early once output arrives and briefly settles. Reports \
                          whether the process is still running and its exit code once done."
    )]
    async fn shell_read(
        &self,
        Parameters(args): Parameters<ReadArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        let session = self.session(&args.session_id)?;
        let deadline_ms = args.timeout_ms.unwrap_or(2000);
        let poll = Duration::from_millis(50);
        let mut waited = 0u64;

        // Wait for the first new bytes (or timeout / process exit).
        loop {
            let available = session.buffer.lock().unwrap().len();
            let cursor = *session.cursor.lock().unwrap();
            if available > cursor || session.returncode().is_some() || waited >= deadline_ms {
                break;
            }
            sleep(poll).await;
            waited += poll.as_millis() as u64;
        }
        // Let a burst of output finish arriving before draining.
        sleep(Duration::from_millis(120)).await;

        let output = {
            let buffer = session.buffer.lock().unwrap();
            let mut cursor = session.cursor.lock().unwrap();
            let slice = &buffer[(*cursor).min(buffer.len())..];
            let text = String::from_utf8_lossy(slice).into_owned();
            *cursor = buffer.len();
            text
        };

        let returncode = session.returncode();
        json_result(&ReadOutput {
            session_id: args.session_id,
            output,
            running: returncode.is_none(),
            returncode,
        })
    }

    /// Lists all known interactive sessions.
    #[tool(
        description = "Lists all known interactive sessions with their command, running \
                          state, and exit code. Sessions persist across calls until killed."
    )]
    async fn shell_list(&self) -> Result<CallToolResult, ErrorData> {
        let sessions = self.sessions.lock().unwrap();
        let mut list: Vec<SessionInfo> = sessions
            .iter()
            .map(|(id, s)| {
                let returncode = s.returncode();
                SessionInfo {
                    session_id: id.clone(),
                    command: s.command.clone(),
                    running: returncode.is_none(),
                    returncode,
                }
            })
            .collect();
        list.sort_by(|a, b| a.session_id.cmp(&b.session_id));
        json_result(&serde_json::json!({ "sessions": list }))
    }

    /// Resizes the terminal of an interactive session.
    #[tool(
        description = "Resizes the pseudo-terminal of an interactive session. Useful for \
                          full-screen TUIs that lay out according to the terminal dimensions."
    )]
    async fn shell_resize(
        &self,
        Parameters(args): Parameters<ResizeArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        let session = self.session(&args.session_id)?;
        let size = PtySize {
            rows: args.rows,
            cols: args.cols,
            pixel_width: 0,
            pixel_height: 0,
        };
        session
            .master
            .lock()
            .unwrap()
            .resize(size)
            .map_err(|e| ErrorData::internal_error(format!("failed to resize: {e}"), None))?;
        json_result(&serde_json::json!({
            "session_id": args.session_id,
            "cols": args.cols,
            "rows": args.rows,
        }))
    }

    /// Terminates an interactive session and removes it.
    #[tool(
        description = "Terminates an interactive session (killing its process if still \
                          running) and removes it from the session table."
    )]
    async fn shell_kill(
        &self,
        Parameters(args): Parameters<SessionRef>,
    ) -> Result<CallToolResult, ErrorData> {
        let session = self.session(&args.session_id)?;
        let _ = session.killer.lock().unwrap().kill();
        self.sessions.lock().unwrap().remove(&args.session_id);
        json_result(&serde_json::json!({
            "session_id": args.session_id,
            "killed": true,
        }))
    }
}

impl ShellMcpServer {
    /// Helper method to execute a bash command with timeout, directory, and environment settings.
    pub async fn execute_script(args: ShellArgs) -> ShellOutput {
        let timeout_secs = args.timeout_secs.unwrap_or(300);
        let timeout_duration = Duration::from_secs(timeout_secs);

        let mut cmd = Command::new("bash");
        cmd.arg("-c").arg(&args.script);
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        if let Some(ref cwd) = args.cwd {
            cmd.current_dir(cwd);
        }

        if let Some(ref env_vars) = args.env {
            for (k, v) in env_vars {
                cmd.env(k, v);
            }
        }

        let result = timeout(timeout_duration, cmd.output()).await;

        match result {
            Ok(Ok(process_output)) => ShellOutput {
                stdout: String::from_utf8_lossy(&process_output.stdout).into_owned(),
                stderr: String::from_utf8_lossy(&process_output.stderr).into_owned(),
                returncode: process_output.status.code().unwrap_or(-1),
            },
            Ok(Err(e)) => ShellOutput {
                stdout: String::new(),
                stderr: format!("Failed to execute command: {}", e),
                returncode: -1,
            },
            Err(_) => ShellOutput {
                stdout: String::new(),
                stderr: format!("Command timed out after {} seconds", timeout_secs),
                returncode: -1,
            },
        }
    }
}

#[tool_handler]
impl ServerHandler for ShellMcpServer {
    fn get_info(&self) -> InitializeResult {
        InitializeResult::new(ServerCapabilities::builder().enable_tools().build())
            .with_instructions(
                "Raw shell execution. Use `shell` for one-shot commands. For interactive \
                 programs (REPLs, database clients, ssh, TUIs) start a session with \
                 `shell_start`, drive it with `shell_write`, observe output with `shell_read`, \
                 and clean up with `shell_kill`.",
            )
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = ShellMcpServer::new();
    let service = server.serve(transport::stdio()).await?;
    service.waiting().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_execute_basic_command() {
        let args = ShellArgs {
            script: "echo 'hello world'".to_string(),
            cwd: None,
            timeout_secs: Some(10),
            env: None,
        };
        let res = ShellMcpServer::execute_script(args).await;
        assert_eq!(res.returncode, 0);
        assert_eq!(res.stdout.trim(), "hello world");
        assert!(res.stderr.is_empty());
    }

    #[tokio::test]
    async fn test_execute_custom_env() {
        let mut env = HashMap::new();
        env.insert("FOO".to_string(), "BAR".to_string());
        let args = ShellArgs {
            script: "echo $FOO".to_string(),
            cwd: None,
            timeout_secs: Some(10),
            env: Some(env),
        };
        let res = ShellMcpServer::execute_script(args).await;
        assert_eq!(res.returncode, 0);
        assert_eq!(res.stdout.trim(), "BAR");
    }

    #[tokio::test]
    async fn test_execute_timeout() {
        let args = ShellArgs {
            script: "sleep 5".to_string(),
            cwd: None,
            timeout_secs: Some(1),
            env: None,
        };
        let res = ShellMcpServer::execute_script(args).await;
        assert_eq!(res.returncode, -1);
        assert!(res.stderr.contains("timed out"));
    }

    /// Drives an interactive session end-to-end: start bash, run a command, read output.
    #[tokio::test]
    async fn test_interactive_session_roundtrip() {
        let server = ShellMcpServer::new();

        let start = server
            .shell_start(Parameters(StartArgs {
                command: None,
                cwd: None,
                env: None,
                cols: None,
                rows: None,
            }))
            .await
            .expect("start session");
        let id = extract_session_id(&start);

        server
            .shell_write(Parameters(WriteArgs {
                session_id: id.clone(),
                input: "echo interactive-ok".to_string(),
                enter: Some(true),
            }))
            .await
            .expect("write to session");

        let output = read_until(&server, &id, "interactive-ok").await;
        assert!(
            output.contains("interactive-ok"),
            "expected echoed output, got: {output:?}"
        );

        server
            .shell_kill(Parameters(SessionRef {
                session_id: id.clone(),
            }))
            .await
            .expect("kill session");

        assert!(server.session(&id).is_err(), "session should be removed");
    }

    /// A REPL keeps state across writes within one session.
    #[tokio::test]
    async fn test_interactive_repl_state_persists() {
        let server = ShellMcpServer::new();
        let start = server
            .shell_start(Parameters(StartArgs {
                command: None,
                cwd: None,
                env: None,
                cols: None,
                rows: None,
            }))
            .await
            .expect("start session");
        let id = extract_session_id(&start);

        for cmd in ["X=42", "echo value-is-$X"] {
            server
                .shell_write(Parameters(WriteArgs {
                    session_id: id.clone(),
                    input: cmd.to_string(),
                    enter: Some(true),
                }))
                .await
                .expect("write");
        }

        let output = read_until(&server, &id, "value-is-42").await;
        assert!(
            output.contains("value-is-42"),
            "shell variable should persist across writes, got: {output:?}"
        );

        let _ = server
            .shell_kill(Parameters(SessionRef { session_id: id }))
            .await;
    }

    fn result_text(result: &CallToolResult) -> String {
        result.content[0]
            .as_text()
            .expect("expected text content")
            .text
            .clone()
    }

    fn extract_session_id(result: &CallToolResult) -> String {
        let parsed: serde_json::Value = serde_json::from_str(&result_text(result)).unwrap();
        parsed["session_id"].as_str().unwrap().to_string()
    }

    /// Reads repeatedly, accumulating output until `needle` appears or attempts run out.
    async fn read_until(server: &ShellMcpServer, id: &str, needle: &str) -> String {
        let mut acc = String::new();
        for _ in 0..10 {
            let res = server
                .shell_read(Parameters(ReadArgs {
                    session_id: id.to_string(),
                    timeout_ms: Some(1000),
                }))
                .await
                .expect("read");
            let parsed: serde_json::Value = serde_json::from_str(&result_text(&res)).unwrap();
            acc.push_str(parsed["output"].as_str().unwrap_or(""));
            if acc.contains(needle) {
                break;
            }
        }
        acc
    }
}
