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

/// Metrics tracking for monitoring server performance
#[derive(Debug, Default)]
pub struct ServerMetrics {
    /// Total number of tool calls processed
    pub total_tool_calls: std::sync::atomic::AtomicU64,
    /// Number of successful tool calls
    pub successful_tool_calls: std::sync::atomic::AtomicU64,
    /// Number of failed tool calls
    pub failed_tool_calls: std::sync::atomic::AtomicU64,
    /// Total response time in milliseconds
    pub total_response_time_ms: std::sync::atomic::AtomicU64,
    /// Server start time
    pub server_start_time: std::sync::OnceLock<std::time::Instant>,
}

impl ServerMetrics {
    /// Create new metrics instance
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Record a successful tool call with response time
    pub fn record_success(&self, response_time_ms: u64) {
        use std::sync::atomic::Ordering;
        
        self.total_tool_calls.fetch_add(1, Ordering::Relaxed);
        self.successful_tool_calls.fetch_add(1, Ordering::Relaxed);
        self.total_response_time_ms.fetch_add(response_time_ms, Ordering::Relaxed);
    }
    
    /// Record a failed tool call
    pub fn record_failure(&self) {
        use std::sync::atomic::Ordering;
        
        self.total_tool_calls.fetch_add(1, Ordering::Relaxed);
        self.failed_tool_calls.fetch_add(1, Ordering::Relaxed);
    }
    
    /// Get success rate as percentage
    pub fn success_rate(&self) -> f64 {
        use std::sync::atomic::Ordering;
        
        let total = self.total_tool_calls.load(Ordering::Relaxed);
        if total == 0 {
            return 100.0;
        }
        
        let successful = self.successful_tool_calls.load(Ordering::Relaxed);
        (successful as f64 / total as f64) * 100.0
    }
    
    /// Get average response time in milliseconds
    pub fn average_response_time_ms(&self) -> f64 {
        use std::sync::atomic::Ordering;
        
        let total_calls = self.successful_tool_calls.load(Ordering::Relaxed);
        if total_calls == 0 {
            return 0.0;
        }
        
        let total_time = self.total_response_time_ms.load(Ordering::Relaxed);
        total_time as f64 / total_calls as f64
    }
    
    /// Get server uptime in seconds
    pub fn uptime_seconds(&self) -> u64 {
        if let Some(start_time) = self.server_start_time.get() {
            start_time.elapsed().as_secs()
        } else {
            0
        }
    }
    
    /// Initialize server start time
    pub fn initialize_start_time(&self) {
        let _ = self.server_start_time.set(std::time::Instant::now());
    }
    
    /// Get metrics summary for health checks
    pub fn get_health_summary(&self) -> HealthSummary {
        use std::sync::atomic::Ordering;
        
        HealthSummary {
            total_requests: self.total_tool_calls.load(Ordering::Relaxed),
            successful_requests: self.successful_tool_calls.load(Ordering::Relaxed),
            failed_requests: self.failed_tool_calls.load(Ordering::Relaxed),
            success_rate: self.success_rate(),
            average_response_time_ms: self.average_response_time_ms(),
            uptime_seconds: self.uptime_seconds(),
        }
    }
}

/// Health summary for monitoring
#[derive(Debug)]
pub struct HealthSummary {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub success_rate: f64,
    pub average_response_time_ms: f64,
    pub uptime_seconds: u64,
}

/// Log error with appropriate severity level
pub fn log_error_with_severity(error: &error::ArkaftMcpError, context: &str) {
    use tracing::{error, warn, info};
    
    match error.severity() {
        error::ErrorSeverity::Critical => {
            error!("CRITICAL ERROR in {}: {} (recoverable: {})", context, error, error.is_recoverable());
        }
        error::ErrorSeverity::High => {
            error!("HIGH PRIORITY ERROR in {}: {} (recoverable: {})", context, error, error.is_recoverable());
        }
        error::ErrorSeverity::Medium => {
            warn!("MEDIUM PRIORITY ERROR in {}: {} (recoverable: {})", context, error, error.is_recoverable());
        }
        error::ErrorSeverity::Low => {
            info!("LOW PRIORITY ERROR in {}: {} (recoverable: {})", context, error, error.is_recoverable());
        }
    }
}

/// Validate server health based on metrics
pub fn validate_server_health(metrics: &ServerMetrics) -> Result<(), error::ArkaftMcpError> {
    let health = metrics.get_health_summary();
    
    // Check success rate (should be above 90%)
    if health.success_rate < 90.0 && health.total_requests > 10 {
        return Err(error::ArkaftMcpError::resource_limit(
            format!("Success rate too low: {:.1}%", health.success_rate)
        ));
    }
    
    // Check average response time (should be under 5000ms)
    if health.average_response_time_ms > 5000.0 {
        return Err(error::ArkaftMcpError::timeout(
            format!("Average response time too high: {:.1}ms", health.average_response_time_ms)
        ));
    }
    
    Ok(())
}