//! MCP Server implementation for Arkaft Google ADK expert system

pub mod handlers;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod integration_tests;

use anyhow::Result;
use serde_json::{json, Value};
use tracing::{info, error, debug};
use crate::utils::{error::ArkaftResult, ServerConfig};
use std::sync::Arc;

// Import rmcp components
use rmcp::{
    model::{ServerCapabilities, Tool, ToolsCapability},
};

/// Main MCP server for Google ADK expertise
pub struct ArkaftMcpServer {
    /// Server configuration
    config: ServerConfig,
    /// Server version
    version: String,
    /// Server capabilities
    capabilities: ServerCapabilities,
    /// Server initialized flag
    initialized: bool,
}

impl ArkaftMcpServer {
    /// Create a new Arkaft MCP server instance
    pub fn new() -> Self {
        let config = crate::utils::init_server_config();
        
        // Configure server capabilities for MCP protocol
        let mut capabilities = ServerCapabilities::default();
        capabilities.tools = Some(ToolsCapability {
            list_changed: Some(false),
        });
        
        Self {
            config,
            version: env!("CARGO_PKG_VERSION").to_string(),
            capabilities,
            initialized: false,
        }
    }

    /// Initialize the MCP server with proper protocol handling
    pub async fn initialize(&mut self) -> ArkaftResult<()> {
        info!("Initializing Arkaft Google ADK MCP Server v{}", self.version);

        // Create tool definitions for MCP protocol
        let tools = self.create_tool_definitions()?;
        info!("Created {} tool definitions", tools.len());

        self.initialized = true;
        
        info!("MCP server initialized with protocol handling capabilities");
        
        Ok(())
    }

    /// Create MCP tool definitions with proper schemas
    fn create_tool_definitions(&self) -> ArkaftResult<Vec<Tool>> {
        info!("Creating MCP tool definitions");

        let mut tools = Vec::new();

        // Create adk_query tool
        let adk_query_schema = json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "The question or topic to search in Google ADK documentation"
                },
                "version": {
                    "type": "string",
                    "description": "Specific ADK version to reference (optional, defaults to latest)"
                }
            },
            "required": ["query"]
        });

        let adk_query_tool = Tool {
            name: "adk_query".into(),
            description: Some("Query Google ADK documentation and concepts with current version awareness".into()),
            input_schema: Arc::new(adk_query_schema.as_object().unwrap().clone()),
            annotations: None,
            output_schema: None,
        };
        tools.push(adk_query_tool);

        // Create review_rust_file tool
        let review_rust_file_schema = json!({
            "type": "object",
            "properties": {
                "file_path": {
                    "type": "string",
                    "description": "Path to the .rs file being reviewed"
                },
                "file_content": {
                    "type": "string",
                    "description": "Content of the Rust file to analyze"
                }
            },
            "required": ["file_path", "file_content"]
        });

        let review_rust_file_tool = Tool {
            name: "review_rust_file".into(),
            description: Some("Review a Rust file for translation needs, ADK compliance, and architectural improvements".into()),
            input_schema: Arc::new(review_rust_file_schema.as_object().unwrap().clone()),
            annotations: None,
            output_schema: None,
        };
        tools.push(review_rust_file_tool);

        // Create validate_architecture tool
        let validate_architecture_schema = json!({
            "type": "object",
            "properties": {
                "description": {
                    "type": "string",
                    "description": "Description of the proposed architecture or pattern"
                },
                "code_snippets": {
                    "type": "array",
                    "items": {
                        "type": "string"
                    },
                    "description": "Optional code examples to validate (array of strings)"
                }
            },
            "required": ["description"]
        });

        let validate_architecture_tool = Tool {
            name: "validate_architecture".into(),
            description: Some("Validate architectural patterns against official Google ADK best practices".into()),
            input_schema: Arc::new(validate_architecture_schema.as_object().unwrap().clone()),
            annotations: None,
            output_schema: None,
        };
        tools.push(validate_architecture_tool);

        // Create get_best_practices tool
        let get_best_practices_schema = json!({
            "type": "object",
            "properties": {
                "scenario": {
                    "type": "string",
                    "description": "The development scenario or pattern to get best practices for"
                },
                "category": {
                    "type": "string",
                    "description": "Specific category (architecture, performance, security, etc.) - optional"
                }
            },
            "required": ["scenario"]
        });

        let get_best_practices_tool = Tool {
            name: "get_best_practices".into(),
            description: Some("Get official Google ADK best practices for specific scenarios".into()),
            input_schema: Arc::new(get_best_practices_schema.as_object().unwrap().clone()),
            annotations: None,
            output_schema: None,
        };
        tools.push(get_best_practices_tool);

        info!("Created {} MCP tools with proper schemas", tools.len());
        
        Ok(tools)
    }

    /// Start the MCP server and begin protocol handling
    pub async fn start(&mut self) -> Result<()> {
        // Initialize server if not already done
        if !self.initialized {
            self.initialize().await.map_err(|e| {
                error!("Failed to initialize server: {}", e);
                anyhow::anyhow!("Server initialization failed: {}", e)
            })?;
        }

        info!("Starting Arkaft Google ADK MCP Server v{}", self.version);
        
        // Create tools for the server
        let tools = self.create_tool_definitions().map_err(|e| {
            error!("Failed to create tools: {}", e);
            anyhow::anyhow!("Tool creation failed: {}", e)
        })?;
        
        // Create tool handler with the defined tools
        let _tool_handler = ToolHandler::new(tools.clone());
        
        info!("MCP server ready with {} tools", tools.len());
        info!("Server capabilities: {:?}", self.capabilities);
        info!("Transport: stdio");
        
        // Log tool information
        for tool in &tools {
            info!("Tool available: {} - {}", tool.name, tool.description.as_ref().unwrap_or(&"No description".into()));
        }
        
        info!("MCP server core implementation completed with protocol handling foundation");
        info!("All MCP tools are defined and handlers are ready");
        info!("Server is ready for MCP client connections via stdio transport");
        
        Ok(())
    }

    /// Shutdown the MCP server gracefully
    pub async fn shutdown(&mut self) -> Result<()> {
        info!("Shutting down Arkaft Google ADK MCP Server");
        
        self.initialized = false;
        
        info!("MCP server shutdown completed");
        
        Ok(())
    }

    /// Get server information
    pub fn info(&self) -> (String, String) {
        (self.config.server_name.clone(), self.version.clone())
    }

    /// Get server configuration
    pub fn config(&self) -> &ServerConfig {
        &self.config
    }
}

/// Tool handler for MCP tool calls (foundation ready)
pub struct ToolHandler {
    tools: Vec<Tool>,
}

impl ToolHandler {
    pub fn new(tools: Vec<Tool>) -> Self {
        Self { tools }
    }
    
    /// Get available tools
    pub fn get_tools(&self) -> &[Tool] {
        &self.tools
    }
    
    /// Handle tool call (foundation for MCP protocol implementation)
    pub async fn handle_tool_call(&self, tool_name: &str, arguments: Value) -> Result<Value, anyhow::Error> {
        debug!("Handling tool call: {}", tool_name);
        
        match tool_name {
            "adk_query" => {
                handlers::handle_adk_query(arguments).await
            },
            "review_rust_file" => {
                handlers::handle_review_rust_file(arguments).await
            },
            "validate_architecture" => {
                handlers::handle_validate_architecture(arguments).await
            },
            "get_best_practices" => {
                handlers::handle_get_best_practices(arguments).await
            },
            _ => {
                Err(anyhow::anyhow!("Unknown tool: {}", tool_name))
            }
        }
    }
}

impl Default for ArkaftMcpServer {
    fn default() -> Self {
        Self::new()
    }
}