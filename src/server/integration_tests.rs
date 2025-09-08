//! Integration tests for MCP tool handlers

use super::handlers::*;
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