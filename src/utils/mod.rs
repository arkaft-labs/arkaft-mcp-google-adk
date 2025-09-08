//! Utility functions and error handling

pub mod error;

use anyhow::Result;

/// Initialize logging for the application
pub fn init_logging() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"))
        )
        .init();
    
    Ok(())
}

/// Get environment variable with default fallback
pub fn get_env_or_default(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

/// Initialize server configuration from environment variables
pub fn init_server_config() -> ServerConfig {
    ServerConfig {
        adk_docs_version: get_env_or_default("ADK_DOCS_VERSION", "latest"),
        log_level: get_env_or_default("RUST_LOG", "info"),
        server_name: get_env_or_default("MCP_SERVER_NAME", "arkaft-google-adk"),
    }
}

/// Server configuration structure
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Default ADK documentation version to reference
    pub adk_docs_version: String,
    /// Logging level
    pub log_level: String,
    /// Server name
    pub server_name: String,
}