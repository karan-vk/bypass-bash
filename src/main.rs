use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::*;
use rmcp::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Stdio;
use tokio::process::Command;
use tokio::time::{Duration, timeout};

/// Execution output returned by the `shell` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ShellOutput {
    /// Standard output produced by the command.
    pub stdout: String,
    /// Standard error produced by the command.
    pub stderr: String,
    /// Exit code of the process (0 for success, non-zero for error, -1 on timeout/spawn failure).
    pub returncode: i32,
}

/// Arguments for the `shell` tool.
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

/// High-performance Shell MCP Server handler.
#[derive(Clone, Default)]
pub struct ShellMcpServer;

#[tool_router]
impl ShellMcpServer {
    /// Runs a string as a bash shell script.
    #[tool(description = "Runs a string as a bash shell script.")]
    async fn shell(
        &self,
        Parameters(args): Parameters<ShellArgs>,
    ) -> Result<CallToolResult, ErrorData> {
        let output = Self::execute_script(args).await;
        let json_str = serde_json::to_string_pretty(&output).unwrap_or_default();
        Ok(CallToolResult::success(vec![ContentBlock::text(json_str)]))
    }
}

impl ShellMcpServer {
    /// Helper method to execute bash command with timeout, directory, and environment settings.
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

impl ServerHandler for ShellMcpServer {
    fn get_info(&self) -> InitializeResult {
        InitializeResult::new(ServerCapabilities::builder().enable_tools().build())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = ShellMcpServer;
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
}
