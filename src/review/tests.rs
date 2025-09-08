//! Unit tests for code review functionality

use super::*;
use crate::review::analyzer::{
    analyze_translation_opportunities,
    analyze_architectural_patterns,
    analyze_adk_compliance,
    analyze_file_organization,
};

#[tokio::test]
async fn test_code_review_engine_creation() {
    let engine = CodeReviewEngine::new();
    assert!(engine.config.detect_translations);
    assert!(engine.config.check_architecture);
    assert!(engine.config.validate_adk_compliance);
}

#[tokio::test]
async fn test_review_valid_rust_file() {
    let engine = CodeReviewEngine::new();
    let file_content = r#"
        use anyhow::Result;
        
        /// A well-documented function
        pub async fn hello_world() -> Result<String> {
            Ok("Hello, World!".to_string())
        }
        
        /// Another function with proper error handling
        pub async fn process_data(input: &str) -> Result<String> {
            if input.is_empty() {
                return Err(anyhow::anyhow!("Input cannot be empty"));
            }
            Ok(format!("Processed: {}", input))
        }
    "#;
    
    let result = engine.review_file("test.rs", file_content).await;
    assert!(result.is_ok());
    
    let review_result = result.unwrap();
    // Should have minimal issues for well-structured code
    assert!(review_result.translation_opportunities.is_empty());
    // May have some architectural suggestions but should be minimal
}

#[tokio::test]
async fn test_review_file_with_unwrap() {
    let engine = CodeReviewEngine::new();
    let file_content = r#"
        pub fn risky_function() {
            let value = Some(42);
            let result = value.unwrap();
            println!("{}", result);
        }
    "#;
    
    let result = engine.review_file("risky.rs", file_content).await;
    assert!(result.is_ok());
    
    let review_result = result.unwrap();
    assert!(!review_result.translation_opportunities.is_empty());
    
    // Should detect unwrap usage
    let unwrap_opportunity = &review_result.translation_opportunities[0];
    assert!(unwrap_opportunity.description.contains("unwrap"));
    assert!(unwrap_opportunity.suggestion.contains("error handling"));
}

#[tokio::test]
async fn test_review_file_with_panic() {
    let engine = CodeReviewEngine::new();
    let file_content = r#"
        pub fn panic_function() {
            panic!("This should not happen");
        }
    "#;
    
    let result = engine.review_file("panic.rs", file_content).await;
    assert!(result.is_ok());
    
    let review_result = result.unwrap();
    assert!(!review_result.translation_opportunities.is_empty());
    assert!(!review_result.compliance_issues.is_empty());
    
    // Should detect panic usage
    let panic_issue = &review_result.compliance_issues[0];
    assert!(panic_issue.description.contains("panic"));
}

#[tokio::test]
async fn test_analyze_translation_opportunities() {
    let code_with_unwrap = r#"
        fn main() {
            let x = Some(5);
            let y = x.unwrap();
        }
    "#;
    
    let opportunities = analyze_translation_opportunities(code_with_unwrap).unwrap();
    assert!(!opportunities.is_empty());
    assert!(opportunities[0].description.contains("unwrap"));
}

#[tokio::test]
async fn test_analyze_architectural_patterns() {
    let code_without_results = r#"
        fn process_data() {
            println!("Processing...");
        }
        
        fn handle_request() {
            println!("Handling...");
        }
    "#;
    
    let improvements = analyze_architectural_patterns(code_without_results).unwrap();
    assert!(!improvements.is_empty());
    
    // Should suggest Result return types
    let error_handling_improvement = improvements.iter()
        .find(|imp| imp.area.contains("Error Handling"));
    assert!(error_handling_improvement.is_some());
}

#[tokio::test]
async fn test_analyze_adk_compliance() {
    let code_with_issues = r#"
        fn bad_function() {
            panic!("Error occurred");
            let x = Some(5).unwrap();
            todo!("Implement this");
        }
    "#;
    
    let issues = analyze_adk_compliance(code_with_issues).unwrap();
    assert!(!issues.is_empty());
    
    // Should detect multiple compliance issues
    let has_panic_issue = issues.iter().any(|issue| issue.description.contains("panic"));
    let has_todo_issue = issues.iter().any(|issue| issue.description.contains("incomplete"));
    
    assert!(has_panic_issue);
    assert!(has_todo_issue);
}

#[tokio::test]
async fn test_analyze_file_organization() {
    let large_file_content = (0..600)
        .map(|i| format!("fn function_{}() {{}}", i))
        .collect::<Vec<_>>()
        .join("\n");
    
    let suggestions = analyze_file_organization("large_file.rs", &large_file_content).unwrap();
    assert!(!suggestions.is_empty());
    
    // Should suggest splitting large file
    let size_suggestion = suggestions.iter()
        .find(|s| s.suggestion_type.contains("File Size"));
    assert!(size_suggestion.is_some());
}

#[tokio::test]
async fn test_syntax_error_handling() {
    let invalid_rust_code = r#"
        fn invalid_syntax( {
            let x = 
        }
    "#;
    
    let opportunities = analyze_translation_opportunities(invalid_rust_code).unwrap();
    assert!(!opportunities.is_empty());
    
    // Should detect syntax errors
    let syntax_error = &opportunities[0];
    assert!(syntax_error.description.contains("Syntax errors"));
}

#[tokio::test]
async fn test_async_pattern_detection() {
    let sync_io_code = r#"
        use std::fs::File;
        
        fn read_file() {
            let file = File::open("test.txt");
        }
    "#;
    
    let opportunities = analyze_translation_opportunities(sync_io_code).unwrap();
    let async_opportunity = opportunities.iter()
        .find(|opp| opp.description.contains("Synchronous I/O"));
    
    assert!(async_opportunity.is_some());
    assert!(async_opportunity.unwrap().suggestion.contains("async"));
}

#[tokio::test]
async fn test_clean_code_no_issues() {
    let clean_code = r#"
        use anyhow::Result;
        
        /// A well-documented function
        pub async fn process_data(input: &str) -> Result<String> {
            if input.is_empty() {
                return Err(anyhow::anyhow!("Input cannot be empty"));
            }
            
            Ok(format!("Processed: {}", input))
        }
        
        /// Another async function
        pub async fn handle_request() -> Result<()> {
            Ok(())
        }
    "#;
    
    let opportunities = analyze_translation_opportunities(clean_code).unwrap();
    let improvements = analyze_architectural_patterns(clean_code).unwrap();
    let issues = analyze_adk_compliance(clean_code).unwrap();
    
    // Debug what issues are found
    for issue in &issues {
        println!("Issue: {} - {}", issue.issue_type, issue.description);
    }
    
    // Clean code should have minimal or no issues
    assert!(opportunities.is_empty());
    // May have some architectural suggestions but should be reasonable
    assert!(improvements.len() <= 2); // Allow for some suggestions
    // Allow for documentation compliance issues since we're checking for missing docs
    assert!(issues.len() <= 1); // May have documentation compliance issue
}

#[tokio::test]
async fn test_format_review_suggestions() {
    use crate::review::suggestions::format_review_suggestions;
    
    let review_result = ReviewResult {
        translation_opportunities: vec![
            TranslationOpportunity {
                line: 5,
                description: "Test opportunity".to_string(),
                suggestion: "Test suggestion".to_string(),
            }
        ],
        architectural_improvements: vec![
            ArchitecturalImprovement {
                area: "Test Area".to_string(),
                current_pattern: "Current".to_string(),
                recommended_pattern: "Recommended".to_string(),
                rationale: "Test rationale".to_string(),
            }
        ],
        compliance_issues: vec![
            ComplianceIssue {
                issue_type: "Test Issue".to_string(),
                description: "Test description".to_string(),
                fix_suggestion: "Test fix".to_string(),
            }
        ],
        organization_suggestions: vec![
            OrganizationSuggestion {
                suggestion_type: "Test Organization".to_string(),
                description: "Test org description".to_string(),
                action: "Test action".to_string(),
            }
        ],
    };
    
    let formatted = format_review_suggestions(&review_result);
    
    assert!(formatted.contains("# Rust File Review Results"));
    assert!(formatted.contains("Translation Opportunities"));
    assert!(formatted.contains("Architectural Improvements"));
    assert!(formatted.contains("ADK Compliance Issues"));
    assert!(formatted.contains("File Organization Suggestions"));
    assert!(formatted.contains("Test opportunity"));
    assert!(formatted.contains("Test suggestion"));
}