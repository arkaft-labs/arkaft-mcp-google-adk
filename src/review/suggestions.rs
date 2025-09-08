//! Improvement suggestions generation for code review

use super::ReviewResult;

/// Generate formatted suggestions from review results
pub fn format_review_suggestions(result: &ReviewResult) -> String {
    let mut output = String::new();
    
    output.push_str("# Rust File Review Results\n\n");
    
    if !result.translation_opportunities.is_empty() {
        output.push_str("## Translation Opportunities\n\n");
        for opportunity in &result.translation_opportunities {
            output.push_str(&format!(
                "**Line {}**: {}\n*Suggestion*: {}\n\n",
                opportunity.line,
                opportunity.description,
                opportunity.suggestion
            ));
        }
    }
    
    if !result.architectural_improvements.is_empty() {
        output.push_str("## Architectural Improvements\n\n");
        for improvement in &result.architectural_improvements {
            output.push_str(&format!(
                "**{}**\n*Current*: {}\n*Recommended*: {}\n*Rationale*: {}\n\n",
                improvement.area,
                improvement.current_pattern,
                improvement.recommended_pattern,
                improvement.rationale
            ));
        }
    }
    
    if !result.compliance_issues.is_empty() {
        output.push_str("## ADK Compliance Issues\n\n");
        for issue in &result.compliance_issues {
            output.push_str(&format!(
                "**{}**: {}\n*Fix*: {}\n\n",
                issue.issue_type,
                issue.description,
                issue.fix_suggestion
            ));
        }
    }
    
    if !result.organization_suggestions.is_empty() {
        output.push_str("## File Organization Suggestions\n\n");
        for suggestion in &result.organization_suggestions {
            output.push_str(&format!(
                "**{}**: {}\n*Action*: {}\n\n",
                suggestion.suggestion_type,
                suggestion.description,
                suggestion.action
            ));
        }
    }
    
    if result.translation_opportunities.is_empty() 
        && result.architectural_improvements.is_empty()
        && result.compliance_issues.is_empty()
        && result.organization_suggestions.is_empty() {
        output.push_str("No issues found. The code appears to follow good practices.\n");
    }
    
    output
}