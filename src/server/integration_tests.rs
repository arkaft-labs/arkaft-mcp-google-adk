//! Integration tests for MCP tool handlers

use super::handlers::*;
use super::ToolHandler;
use serde_json::json;

#[tokio::test]
async fn test_review_rust_file_handler_valid_input() {
    let params = json!({
        "file_path": "test.rs",
        "file_content": r#"
            pub fn hello() -> Result<String, Box<dyn std::error::Error>> {
                Ok("Hello".to_string())
            }
        "#
    });
    
    let result = handle_review_rust_file(params).await;
    assert!(result.is_ok());
    
    let response = result.unwrap();
    assert!(response["content"].is_array());
    assert!(response["content"][0]["type"] == "text");
    
    let text_content = response["content"][0]["text"].as_str().unwrap();
    assert!(text_content.contains("Rust File Review Results"));
}

#[tokio::test]
async fn test_review_rust_file_handler_with_issues() {
    let params = json!({
        "file_path": "problematic.rs",
        "file_content": r#"
            pub fn risky_function() {
                let value = Some(42);
                let result = value.unwrap();
                panic!("Something went wrong");
            }
        "#
    });
    
    let result = handle_review_rust_file(params).await;
    assert!(result.is_ok());
    
    let response = result.unwrap();
    let text_content = response["content"][0]["text"].as_str().unwrap();
    
    // Should detect unwrap and panic issues
    assert!(text_content.contains("Translation Opportunities") || text_content.contains("ADK Compliance Issues"));
}

#[tokio::test]
async fn test_review_rust_file_handler_invalid_params() {
    let params = json!({
        "file_path": "test.rs"
        // Missing file_content
    });
    
    let result = handle_review_rust_file(params).await;
    assert!(result.is_err());
    
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("Invalid parameters"));
}

#[tokio::test]
async fn test_review_rust_file_handler_empty_file_path() {
    let params = json!({
        "file_path": "",
        "file_content": "fn main() {}"
    });
    
    let result = handle_review_rust_file(params).await;
    assert!(result.is_err());
    
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("file_path parameter cannot be empty"));
}

#[tokio::test]
async fn test_review_rust_file_handler_non_rust_file() {
    let params = json!({
        "file_path": "test.py",
        "file_content": "print('hello')"
    });
    
    let result = handle_review_rust_file(params).await;
    assert!(result.is_err());
    
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("Only .rs files can be reviewed"));
}

// Additional comprehensive integration tests for complete MCP functionality

#[tokio::test]
async fn test_adk_query_handler_integration() {
    let params = json!({
        "query": "async programming patterns",
        "version": "latest"
    });
    
    let result = handle_adk_query(params).await;
    assert!(result.is_ok());
    
    let response = result.unwrap();
    assert!(response["content"].is_array());
    assert!(response["content"][0]["type"] == "text");
    
    let text_content = response["content"][0]["text"].as_str().unwrap();
    assert!(text_content.contains("Google ADK"));
    assert!(text_content.contains("async"));
}

#[tokio::test]
async fn test_validate_architecture_handler_integration() {
    let params = json!({
        "description": "Using async/await patterns with proper error handling",
        "code_snippets": [
            "async fn process_data() -> Result<String, Box<dyn std::error::Error>> { Ok(\"processed\".to_string()) }"
        ]
    });
    
    let result = handle_validate_architecture(params).await;
    assert!(result.is_ok());
    
    let response = result.unwrap();
    assert!(response["content"].is_array());
    assert!(response["content"][0]["type"] == "text");
    
    let text_content = response["content"][0]["text"].as_str().unwrap();
    assert!(text_content.contains("Architecture Validation Result"));
}

#[tokio::test]
async fn test_get_best_practices_handler_integration() {
    let params = json!({
        "scenario": "error handling",
        "category": "architecture"
    });
    
    let result = handle_get_best_practices(params).await;
    assert!(result.is_ok());
    
    let response = result.unwrap();
    assert!(response["content"].is_array());
    assert!(response["content"][0]["type"] == "text");
    
    let text_content = response["content"][0]["text"].as_str().unwrap();
    assert!(text_content.contains("Google ADK Best Practices"));
}

#[tokio::test]
async fn test_tool_handler_integration_all_tools() {
    // Create tool handler with all tools
    let tools = vec![]; // Tools would be created by server
    let metrics = std::sync::Arc::new(crate::utils::ServerMetrics::new());
    let handler = ToolHandler::new(tools, metrics);
    
    // Test adk_query through handler
    let adk_params = json!({
        "query": "testing patterns"
    });
    
    let result = handler.handle_tool_call("adk_query", adk_params).await;
    assert!(result.is_ok());
    
    // Test review_rust_file through handler
    let review_params = json!({
        "file_path": "test.rs",
        "file_content": "fn main() { println!(\"Hello\"); }"
    });
    
    let result = handler.handle_tool_call("review_rust_file", review_params).await;
    assert!(result.is_ok());
    
    // Test validate_architecture through handler
    let validate_params = json!({
        "description": "Simple function architecture"
    });
    
    let result = handler.handle_tool_call("validate_architecture", validate_params).await;
    assert!(result.is_ok());
    
    // Test get_best_practices through handler
    let practices_params = json!({
        "scenario": "testing"
    });
    
    let result = handler.handle_tool_call("get_best_practices", practices_params).await;
    assert!(result.is_ok());
    
    // Test unknown tool error handling
    let result = handler.handle_tool_call("unknown_tool", json!({})).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Unknown tool"));
}

#[tokio::test]
async fn test_complete_mcp_server_integration() {
    // Test complete server lifecycle with all components
    let mut server = crate::server::ArkaftMcpServer::new();
    
    // Test server initialization
    let init_result = server.initialize().await;
    assert!(init_result.is_ok());
    
    // Test server startup
    let start_result = server.start().await;
    assert!(start_result.is_ok());
    
    // Test server metrics and health
    let metrics = server.metrics();
    let health = server.health_check();
    assert!(health.is_ok());
    
    let health_summary = health.unwrap();
    assert_eq!(health_summary.total_requests, 0); // No requests yet
    assert_eq!(health_summary.success_rate, 100.0); // Default success rate
    
    // Test server info
    let (name, version) = server.info();
    assert_eq!(name, "arkaft-google-adk");
    assert_eq!(version, env!("CARGO_PKG_VERSION"));
    
    // Test server shutdown
    let shutdown_result = server.shutdown().await;
    assert!(shutdown_result.is_ok());
}

#[tokio::test]
async fn test_error_handling_and_monitoring_integration() {
    use crate::utils::{ServerMetrics, log_error_with_severity, validate_server_health};
    use crate::utils::error::ArkaftMcpError;
    
    // Test comprehensive error handling
    let metrics = std::sync::Arc::new(ServerMetrics::new());
    metrics.initialize_start_time();
    
    // Test error logging with different severities
    let critical_error = ArkaftMcpError::server_initialization("Test critical error".to_string());
    log_error_with_severity(&critical_error, "test_context");
    assert_eq!(critical_error.severity(), crate::utils::error::ErrorSeverity::Critical);
    assert!(!critical_error.is_recoverable());
    
    let recoverable_error = ArkaftMcpError::parameter_validation("Test parameter error".to_string());
    log_error_with_severity(&recoverable_error, "test_context");
    assert_eq!(recoverable_error.severity(), crate::utils::error::ErrorSeverity::Low);
    assert!(recoverable_error.is_recoverable());
    
    // Test metrics tracking
    metrics.record_success(100);
    metrics.record_success(200);
    metrics.record_failure();
    
    let health_summary = metrics.get_health_summary();
    assert_eq!(health_summary.total_requests, 3);
    assert_eq!(health_summary.successful_requests, 2);
    assert_eq!(health_summary.failed_requests, 1);
    assert!((health_summary.success_rate - 66.67).abs() < 0.1);
    assert_eq!(health_summary.average_response_time_ms, 150.0);
    
    // Test health validation
    let health_result = validate_server_health(&metrics);
    assert!(health_result.is_ok()); // Should pass with current metrics
}

#[tokio::test]
async fn test_review_rust_file_handler_empty_content() {
    let params = json!({
        "file_path": "empty.rs",
        "file_content": ""
    });
    
    let result = handle_review_rust_file(params).await;
    assert!(result.is_err());
    
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("file_content parameter cannot be empty"));
}