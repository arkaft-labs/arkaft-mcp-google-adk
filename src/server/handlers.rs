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

/// Parameters for validate_architecture tool
#[derive(Debug, Deserialize, Serialize)]
pub struct ValidateArchitectureParams {
    /// Description of the proposed architecture
    pub description: String,
    /// Optional code snippets to validate
    pub code_snippets: Option<Vec<String>>,
    /// Optional ADK version to validate against
    pub version: Option<String>,
}

/// Handle validate_architecture tool calls
pub async fn handle_validate_architecture(params: Value) -> Result<Value> {
    info!("Handling validate_architecture request with params: {:?}", params);
    
    // Parse and validate parameters
    let validation_params: ValidateArchitectureParams = serde_json::from_value(params)
        .map_err(|e| {
            warn!("Failed to parse validate_architecture parameters: {}", e);
            anyhow!("Invalid parameters for validate_architecture. Expected 'description' (string), optional 'code_snippets' (array of strings), and optional 'version' (string). Error: {}", e)
        })?;
    
    // Validate description parameter
    if validation_params.description.trim().is_empty() {
        warn!("Empty description provided to validate_architecture");
        return Err(anyhow!("Description parameter cannot be empty"));
    }
    
    // Create Best Practices Enforcer instance
    let enforcer = crate::expert::best_practices::BestPracticesEnforcer::new();
    
    // Perform architecture validation
    match enforcer.validate_architecture(
        &validation_params.description,
        validation_params.code_snippets.as_deref(),
        validation_params.version.as_deref(),
    ).await {
        Ok(validation_result) => {
            info!("Successfully completed architecture validation");
            
            // Format the validation results
            let formatted_response = format_architecture_validation_result(&validation_result);
            
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
            error!("Error validating architecture: {}", e);
            Err(anyhow!("Failed to validate architecture: {}", e))
        }
    }
}

/// Parameters for get_best_practices tool
#[derive(Debug, Deserialize, Serialize)]
pub struct GetBestPracticesParams {
    /// The development scenario or pattern
    pub scenario: String,
    /// Optional specific category (architecture, performance, etc.)
    pub category: Option<String>,
    /// Optional ADK version to reference
    pub version: Option<String>,
}

/// Handle get_best_practices tool calls  
pub async fn handle_get_best_practices(params: Value) -> Result<Value> {
    info!("Handling get_best_practices request with params: {:?}", params);
    
    // Parse and validate parameters
    let practices_params: GetBestPracticesParams = serde_json::from_value(params)
        .map_err(|e| {
            warn!("Failed to parse get_best_practices parameters: {}", e);
            anyhow!("Invalid parameters for get_best_practices. Expected 'scenario' (string), optional 'category' (string), and optional 'version' (string). Error: {}", e)
        })?;
    
    // Validate scenario parameter
    if practices_params.scenario.trim().is_empty() {
        warn!("Empty scenario provided to get_best_practices");
        return Err(anyhow!("Scenario parameter cannot be empty"));
    }
    
    // Create Best Practices Enforcer instance
    let enforcer = crate::expert::best_practices::BestPracticesEnforcer::new();
    
    // Retrieve best practices for the scenario
    match enforcer.get_best_practices(
        &practices_params.scenario,
        practices_params.category.as_deref(),
        practices_params.version.as_deref(),
    ).await {
        Ok(practices_result) => {
            info!("Successfully retrieved best practices for scenario: {}", practices_params.scenario);
            
            // Format the best practices results
            let formatted_response = format_best_practices_result(&practices_result);
            
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
            error!("Error retrieving best practices: {}", e);
            Err(anyhow!("Failed to retrieve best practices: {}", e))
        }
    }
}

/// Format architecture validation result for display
fn format_architecture_validation_result(result: &crate::expert::best_practices::ArchitectureValidationResult) -> String {
    let mut response = String::new();
    
    // Header with compliance status
    response.push_str(&format!(
        "# Architecture Validation Result\n\n**Compliance Status:** {}\n**Compliance Score:** {}/100\n\n",
        if result.is_compliant { "✅ COMPLIANT" } else { "❌ NON-COMPLIANT" },
        result.compliance_score
    ));
    
    // Findings section
    if !result.findings.is_empty() {
        response.push_str("## Validation Findings\n\n");
        
        for finding in &result.findings {
            let severity_icon = match finding.severity {
                crate::expert::best_practices::ValidationSeverity::Error => "🔴",
                crate::expert::best_practices::ValidationSeverity::Warning => "🟡",
                crate::expert::best_practices::ValidationSeverity::Info => "🔵",
            };
            
            response.push_str(&format!(
                "### {} {}\n\n{}\n\n",
                severity_icon,
                finding.description,
                finding.location.as_ref().map(|l| format!("**Location:** {}\n\n", l)).unwrap_or_default()
            ));
            
            if let Some(fix) = &finding.suggested_fix {
                response.push_str(&format!("**Suggested Fix:** {}\n\n", fix));
            }
            
            response.push_str("---\n\n");
        }
    }
    
    // Recommendations section
    if !result.recommendations.is_empty() {
        response.push_str("## Recommendations\n\n");
        
        for rec in &result.recommendations {
            response.push_str(&format!(
                "### {} (Priority: {})\n\n{}\n\n",
                rec.description,
                rec.priority,
                rec.category
            ));
            
            if !rec.implementation_steps.is_empty() {
                response.push_str("**Implementation Steps:**\n");
                for step in &rec.implementation_steps {
                    response.push_str(&format!("- {}\n", step));
                }
                response.push('\n');
            }
            
            if !rec.benefits.is_empty() {
                response.push_str("**Benefits:**\n");
                for benefit in &rec.benefits {
                    response.push_str(&format!("- {}\n", benefit));
                }
                response.push('\n');
            }
            
            response.push_str(&format!("**Reference:** [{}]({})\n\n", rec.documentation_ref, rec.documentation_ref));
            response.push_str("---\n\n");
        }
    }
    
    // Documentation references
    if !result.documentation_refs.is_empty() {
        response.push_str("## Official Documentation References\n\n");
        for doc_ref in &result.documentation_refs {
            response.push_str(&format!("- [{}]({})\n", doc_ref, doc_ref));
        }
        response.push('\n');
    }
    
    response.push_str("---\n\n*This validation is based on official Google ADK best practices and architectural guidelines.*");
    
    response
}

/// Format best practices result for display
fn format_best_practices_result(result: &crate::expert::best_practices::BestPracticesResult) -> String {
    let mut response = String::new();
    
    // Header
    response.push_str(&format!(
        "# Google ADK Best Practices\n\n**Scenario:** {}\n**Version:** {}\n\n",
        result.scenario,
        result.version
    ));
    
    // Best practices section
    if !result.practices.is_empty() {
        response.push_str("## Best Practices\n\n");
        
        for practice in &result.practices {
            response.push_str(&format!(
                "### {}\n\n**Category:** {}\n\n{}\n\n",
                practice.title,
                practice.category,
                practice.description
            ));
            
            if !practice.examples.is_empty() {
                response.push_str("**Examples:**\n");
                for example in &practice.examples {
                    response.push_str(&format!("- {}\n", example));
                }
                response.push('\n');
            }
            
            response.push_str(&format!("**Reference:** [{}]({})\n\n", practice.documentation_ref, practice.documentation_ref));
            response.push_str("---\n\n");
        }
    }
    
    // Implementation patterns section
    if !result.patterns.is_empty() {
        response.push_str("## Implementation Patterns\n\n");
        
        for pattern in &result.patterns {
            response.push_str(&format!(
                "### {}\n\n{}\n\n",
                pattern.name,
                pattern.description
            ));
            
            if !pattern.use_cases.is_empty() {
                response.push_str("**Use Cases:**\n");
                for use_case in &pattern.use_cases {
                    response.push_str(&format!("- {}\n", use_case));
                }
                response.push('\n');
            }
            
            if !pattern.code_examples.is_empty() {
                response.push_str("**Code Examples:**\n\n");
                for example in &pattern.code_examples {
                    response.push_str(&format!(
                        "#### {}\n\n```{}\n{}\n```\n\n{}\n\n",
                        example.title,
                        example.language,
                        example.code,
                        example.explanation
                    ));
                }
            }
            
            if !pattern.related_practices.is_empty() {
                response.push_str("**Related Practices:**\n");
                for related in &pattern.related_practices {
                    response.push_str(&format!("- {}\n", related));
                }
                response.push('\n');
            }
            
            response.push_str("---\n\n");
        }
    }
    
    // Documentation references
    if !result.documentation_refs.is_empty() {
        response.push_str("## Official Documentation References\n\n");
        for doc_ref in &result.documentation_refs {
            response.push_str(&format!("- [{}]({})\n", doc_ref, doc_ref));
        }
        response.push('\n');
    }
    
    response.push_str("---\n\n*These best practices are based on official Google ADK documentation and guidelines.*");
    
    response
}