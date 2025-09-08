//! Error types and handling for the MCP server

use thiserror::Error;

/// Main error type for the Arkaft MCP server
#[derive(Error, Debug)]
pub enum ArkaftMcpError {
    /// MCP protocol related errors
    #[error("MCP protocol error: {0}")]
    McpProtocol(String),
    
    /// Documentation query errors
    #[error("Documentation query error: {0}")]
    DocumentationQuery(String),
    
    /// Code review errors
    #[error("Code review error: {0}")]
    CodeReview(String),
    
    /// Best practices validation errors
    #[error("Best practices validation error: {0}")]
    BestPractices(String),
    
    /// Configuration errors
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    /// IO errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    /// JSON serialization errors
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    /// Generic errors
    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

/// Result type alias for Arkaft MCP operations
pub type ArkaftResult<T> = Result<T, ArkaftMcpError>;