use anyhow::Result;
use clap::{Parser, ValueEnum};
use mcp_core::{
    server::Server,
    tool_text_content,
    transport::{ServerSseTransport, ServerStdioTransport},
    types::{ServerCapabilities, ToolCapabilities, ToolResponseContent},
};
use mcp_core_macros::{tool, tool_param};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Transport type to use
    #[arg(value_enum, default_value_t = TransportType::Sse)]
    transport: TransportType,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
enum TransportType {
    Stdio,
    Sse,
}

#[tool(name = "echo", description = "Echo back the message you send")]
async fn echo_tool(
    message: tool_param!(String, description = "The message to echo back"),
) -> Result<ToolResponseContent> {
    Ok(tool_text_content!(message))
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let cli = Cli::parse();

    let server_protocol = Server::builder(
        "echo".to_string(),
        "1.0".to_string(),
        mcp_core::types::ProtocolVersion::V2024_11_05,
    )
    .set_capabilities(ServerCapabilities {
        tools: Some(ToolCapabilities::default()),
        ..Default::default()
    })
    .register_tool(EchoTool::tool(), EchoTool::call())
    .build();

    match cli.transport {
        TransportType::Stdio => {
            let transport = ServerStdioTransport::new(server_protocol);
            Server::start(transport).await
        }
        TransportType::Sse => {
            let transport = ServerSseTransport::new("127.0.0.1".to_string(), 3000, server_protocol);
            Server::start(transport).await
        }
    }
}
