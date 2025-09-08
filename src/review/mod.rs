//! Code Review Engine for Rust file analysis
//! 
//! Analyzes .rs files for translation needs, ADK compliance, and architectural improvements.
//! Provides specific suggestions following Google ADK best practices.

pub mod analyzer;
pub mod suggestions;

#[cfg(test)]
mod tests;

use anyhow::Result;

/// Code Review Engine for analyzing Rust files
pub struct CodeReviewEngine {
    /// Configuration for review analysis
    pub config: ReviewConfig,
}

/// Configuration for code review analysis
#[derive(Clone)]
pub struct ReviewConfig {
    /// Enable translation opportunity detection
    pub detect_translations: bool,
    /// Enable architectural pattern checking
    pub check_architecture: bool,
    /// Enable ADK compliance validation
    pub validate_adk_compliance: bool,
}

/// Results of a code review analysis
#[derive(Debug)]
pub struct ReviewResult {
    /// Translation opportunities found
    pub translation_opportunities: Vec<TranslationOpportunity>,
    /// Architectural improvements suggested
    pub architectural_improvements: Vec<ArchitecturalImprovement>,
    /// ADK compliance issues found
    pub compliance_issues: Vec<ComplianceIssue>,
    /// File organization suggestions
    pub organization_suggestions: Vec<OrganizationSuggestion>,
}

/// A translation opportunity in the code
#[derive(Debug)]
pub struct TranslationOpportunity {
    /// Line number where opportunity exists
    pub line: usize,
    /// Description of the translation opportunity
    pub description: String,
    /// Suggested translation or improvement
    pub suggestion: String,
}

/// An architectural improvement suggestion
#[derive(Debug)]
pub struct ArchitecturalImprovement {
    /// Area of improvement
    pub area: String,
    /// Current pattern detected
    pub current_pattern: String,
    /// Recommended ADK pattern
    pub recommended_pattern: String,
    /// Rationale for the improvement
    pub rationale: String,
}

/// An ADK compliance issue
#[derive(Debug)]
pub struct ComplianceIssue {
    /// Type of compliance issue
    pub issue_type: String,
    /// Description of the issue
    pub description: String,
    /// How to fix the issue
    pub fix_suggestion: String,
}

/// A file organization suggestion
#[derive(Debug)]
pub struct OrganizationSuggestion {
    /// Type of organization improvement
    pub suggestion_type: String,
    /// Description of the suggestion
    pub description: String,
    /// Recommended action
    pub action: String,
}

impl CodeReviewEngine {
    /// Create a new Code Review Engine
    pub fn new() -> Self {
        let config = ReviewConfig {
            detect_translations: true,
            check_architecture: true,
            validate_adk_compliance: true,
        };
        
        Self { config }
    }
    
    /// Review a Rust file for improvements
    pub async fn review_file(&self, file_path: &str, file_content: &str) -> Result<ReviewResult> {
        use crate::review::analyzer::{
            analyze_translation_opportunities,
            analyze_architectural_patterns,
            analyze_adk_compliance,
            analyze_file_organization,
        };
        
        let mut result = ReviewResult {
            translation_opportunities: Vec::new(),
            architectural_improvements: Vec::new(),
            compliance_issues: Vec::new(),
            organization_suggestions: Vec::new(),
        };
        
        // Analyze translation opportunities if enabled
        if self.config.detect_translations {
            result.translation_opportunities = analyze_translation_opportunities(file_content)?;
        }
        
        // Analyze architectural patterns if enabled
        if self.config.check_architecture {
            result.architectural_improvements = analyze_architectural_patterns(file_content)?;
        }
        
        // Analyze ADK compliance if enabled
        if self.config.validate_adk_compliance {
            result.compliance_issues = analyze_adk_compliance(file_content)?;
        }
        
        // Always analyze file organization
        result.organization_suggestions = analyze_file_organization(file_path, file_content)?;
        
        Ok(result)
    }
}

impl Default for CodeReviewEngine {
    fn default() -> Self {
        Self::new()
    }
}