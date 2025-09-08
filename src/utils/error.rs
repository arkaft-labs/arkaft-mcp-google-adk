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
    
    /// Parameter validation errors
    #[error("Parameter validation error: {0}")]
    ParameterValidation(String),
    
    /// Tool execution errors
    #[error("Tool execution error: {0}")]
    ToolExecution(String),
    
    /// Server initialization errors
    #[error("Server initialization error: {0}")]
    ServerInitialization(String),
    
    /// Resource limit errors
    #[error("Resource limit exceeded: {0}")]
    ResourceLimit(String),
    
    /// Timeout errors
    #[error("Operation timeout: {0}")]
    Timeout(String),
    
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

impl ArkaftMcpError {
    /// Create a parameter validation error
    pub fn parameter_validation<S: Into<String>>(msg: S) -> Self {
        Self::ParameterValidation(msg.into())
    }
    
    /// Create a tool execution error
    pub fn tool_execution<S: Into<String>>(msg: S) -> Self {
        Self::ToolExecution(msg.into())
    }
    
    /// Create a server initialization error
    pub fn server_initialization<S: Into<String>>(msg: S) -> Self {
        Self::ServerInitialization(msg.into())
    }
    
    /// Create a resource limit error
    pub fn resource_limit<S: Into<String>>(msg: S) -> Self {
        Self::ResourceLimit(msg.into())
    }
    
    /// Create a timeout error
    pub fn timeout<S: Into<String>>(msg: S) -> Self {
        Self::Timeout(msg.into())
    }
    
    /// Check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            Self::ParameterValidation(_) | 
            Self::DocumentationQuery(_) | 
            Self::CodeReview(_) | 
            Self::BestPractices(_) |
            Self::Timeout(_)
        )
    }
    
    /// Get error severity level
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            Self::McpProtocol(_) | Self::ServerInitialization(_) => ErrorSeverity::Critical,
            Self::Configuration(_) | Self::ResourceLimit(_) => ErrorSeverity::High,
            Self::ToolExecution(_) | Self::Io(_) => ErrorSeverity::Medium,
            Self::ParameterValidation(_) | Self::DocumentationQuery(_) | 
            Self::CodeReview(_) | Self::BestPractices(_) | Self::Timeout(_) => ErrorSeverity::Low,
            Self::Json(_) | Self::Internal(_) => ErrorSeverity::Medium,
        }
    }
}

/// Error severity levels for monitoring and alerting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSeverity {
    /// Critical errors that require immediate attention
    Critical,
    /// High priority errors that should be addressed quickly
    High,
    /// Medium priority errors that should be investigated
    Medium,
    /// Low priority errors that are typically user-related
    Low,
}

/// Result type alias for Arkaft MCP operations
pub type ArkaftResult<T> = Result<T, ArkaftMcpError>;