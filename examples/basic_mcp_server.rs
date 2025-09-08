// Basic MCP server example to understand the API
use rmcp::*;
use rmcp::model::*;
use rmcp::transport::*;
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating basic MCP server");
    
    // Create server capabilities
    let mut capabilities = ServerCapabilities::default();
    // Let's see what's available in ToolsCapability
    println!("Default tools capability: {:?}", capabilities.tools);
    
    println!("Capabilities: {:?}", capabilities);
    
    // Create transport
    let transport = stdio();
    println!("Transport created");
    
    // Try to create a server
    // Need to explore what's available in rmcp
    
    Ok(())
}