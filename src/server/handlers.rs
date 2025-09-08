//! MCP request handlers for Google ADK tools

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::{info, warn, error};
use crate::expert::DocumentationExpert;

/// Parameters for adk_query tool
#[derive(Debug, Deserialize, Serialize)]
pub struct AdkQueryParams {
    /// The question or topic to search in ADK documentation
    pub query: String,
    /// Optional specific ADK version to reference (defaults to latest)
    pub version: Option<String>,
}

/// Handle adk_query tool calls with comprehensive ADK documentation expertise
pub async fn handle_adk_query(params: Value) -> Result<Value> {
    info!("Handling adk_query request with params: {:?}", params);
    
    // Parse and validate parameters
    let query_params: AdkQueryParams = serde_json::from_value(params)
        .map_err(|e| {
            warn!("Failed to parse adk_query parameters: {}", e);
            anyhow!("Invalid parameters for adk_query. Expected 'query' (string) and optional 'version' (string). Error: {}", e)
        })?;
    
    // Validate query parameter
    if query_params.query.trim().is_empty() {
        warn!("Empty query provided to adk_query");
        return Err(anyhow!("Query parameter cannot be empty"));
    }
    
    // Create Documentation Expert instance
    let expert = DocumentationExpert::new();
    
    // Process the query with version-specific information retrieval
    match expert.query_documentation(&query_params.query, query_params.version.as_deref()).await {
        Ok(response) => {
            info!("Successfully processed adk_query for: {}", query_params.query);
            Ok(serde_json::json!({
                "content": [
                    {
                        "type": "text",
                        "text": response
                    }
                ]
            }))
        }
        Err(e) => {
            error!("Error processing adk_query: {}", e);
            Err(anyhow!("Failed to process ADK documentation query: {}", e))
        }
    }
}

/// Parameters for review_rust_file tool
#[derive(Debug, Deserialize, Serialize)]
pub struct ReviewRustFileParams {
    /// Path to the .rs file being reviewed
    pub file_path: String,
    /// Content of the Rust file to analyze
    pub file_content: String,
}

/// Handle review_rust_file tool calls
pub async fn handle_review_rust_file(params: Value) -> Result<Value> {
    info!("Handling review_rust_file request with params: {:?}", params);
    
    // Parse and validate parameters
    let review_params: ReviewRustFileParams = serde_json::from_value(params)
        .map_err(|e| {
            warn!("Failed to parse review_rust_file parameters: {}", e);
            anyhow!("Invalid parameters for review_rust_file. Expected 'file_path' (string) and 'file_content' (string). Error: {}", e)
        })?;
    
    // Validate parameters
    if review_params.file_path.trim().is_empty() {
        warn!("Empty file_path provided to review_rust_file");
        return Err(anyhow!("file_path parameter cannot be empty"));
    }
    
    if review_params.file_content.trim().is_empty() {
        warn!("Empty file_content provided to review_rust_file");
        return Err(anyhow!("file_content parameter cannot be empty"));
    }
    
    // Validate that it's a Rust file
    if !review_params.file_path.ends_with(".rs") {
        warn!("Non-Rust file provided to review_rust_file: {}", review_params.file_path);
        return Err(anyhow!("Only .rs files can be reviewed. Provided file: {}", review_params.file_path));
    }
    
    // Create Code Review Engine instance
    let review_engine = crate::review::CodeReviewEngine::new();
    
    // Perform comprehensive file analysis
    match review_engine.review_file(&review_params.file_path, &review_params.file_content).await {
        Ok(review_result) => {
            info!("Successfully completed review for file: {}", review_params.file_path);
            
            // Format the review results using the suggestions module
            let formatted_response = crate::review::suggestions::format_review_suggestions(&review_result);
            
            Ok(serde_json::json!({
                "content": [
                    {
                        "type": "text",
                        "text": formatted_response
                    }
                ]
            }))
        }
        Err(e) => {
            error!("Error reviewing Rust file {}: {}", review_params.file_path, e);
            Err(anyhow!("Failed to review Rust file: {}", e))
        }
    }
}

/// Handle validate_architecture tool calls
pub async fn handle_validate_architecture(_params: Value) -> Result<Value> {
    info!("Handling validate_architecture request");
    
    // TODO: Implement architecture validation logic
    // This will be implemented when the Best Practices Enforcer is built
    
    Ok(serde_json::json!({
        "content": [
            {
                "type": "text",
                "text": "Architecture validation functionality is currently being implemented. Please check back soon for Google ADK best practices validation."
            }
        ]
    }))
}

/// Handle get_best_practices tool calls  
pub async fn handle_get_best_practices(_params: Value) -> Result<Value> {
    info!("Handling get_best_practices request");
    
    // TODO: Implement best practices retrieval logic
    // This will be implemented when the Best Practices Enforcer is built
    
    Ok(serde_json::json!({
        "content": [
            {
                "type": "text",
                "text": "Best practices retrieval functionality is currently being implemented. Please check back soon for official Google ADK guidelines and recommendations."
            }
        ]
    }))
}