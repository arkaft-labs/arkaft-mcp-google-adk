use arkaft_mcp_google_adk::{ArkaftMcpServer, utils};
use anyhow::Result;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging infrastructure
    utils::init_logging()?;
    
    info!("Initializing Arkaft Google ADK MCP Server");
    
    // Create and start the MCP server
    let mut server = ArkaftMcpServer::new();
    
    if let Err(e) = server.start().await {
        error!("Failed to start server: {}", e);
        return Err(e);
    }
    
    let (name, version) = server.info();
    info!("Arkaft Google ADK MCP Server '{}' v{} foundation established", name, version);
    info!("Server structure ready for MCP protocol implementation");
    
    // Keep the server running
    tokio::signal::ctrl_c().await?;
    info!("Shutting down server");
    
    Ok(())
}
