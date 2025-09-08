//! Unit tests for Best Practices Enforcement System

#[cfg(test)]
mod tests {
    use super::super::best_practices::*;
    use crate::expert::adk_knowledge::AdkKnowledgeBase;

    #[tokio::test]
    async fn test_best_practices_enforcer_creation() {
        let enforcer = BestPracticesEnforcer::new();
        assert!(!enforcer.knowledge_base.default_version.is_empty());
        assert!(!enforcer.validation_rules.architecture_rules.is_empty());
        assert!(!enforcer.validation_rules.code_pattern_rules.is_empty());
    }

    #[tokio::test]
    async fn test_validate_architecture_compliant() {
        let enforcer = BestPracticesEnforcer::new();
        let description = "A well-structured ADK application using async patterns and proper error handling with Result types";
        
        let result = enforcer.validate_architecture(description, None, None).await.unwrap();
        
        assert!(result.compliance_score > 50); // Should have decent score for good description
        assert!(!result.documentation_refs.is_empty());
    }

    #[tokio::test]
    async fn test_validate_architecture_with_issues() {
        let enforcer = BestPracticesEnforcer::new();
        let description = "Application with blocking operations and panic-based error handling";
        
        let result = enforcer.validate_architecture(description, None, None).await.unwrap();
        
        // Should detect issues with blocking operations and panic handling
        assert!(!result.findings.is_empty());
        assert!(!result.recommendations.is_empty());
    }

    #[tokio::test]
    async fn test_validate_architecture_with_code_snippets() {
        let enforcer = BestPracticesEnforcer::new();
        let description = "Standard ADK application";
        let code_snippets = vec![
            "fn main() { panic!(\"This is bad\"); }".to_string(),
            "let result = some_operation().unwrap();".to_string(),
        ];
        
        let result = enforcer.validate_architecture(description, Some(&code_snippets), None).await.unwrap();
        
        // Should detect panic! and unwrap() usage
        assert!(!result.findings.is_empty());
        let has_panic_finding = result.findings.iter().any(|f| f.description.contains("panic"));
        let has_unwrap_finding = result.findings.iter().any(|f| f.description.contains("unwrap"));
        assert!(has_panic_finding || has_unwrap_finding);
    }

    #[tokio::test]
    async fn test_get_best_practices_general() {
        let enforcer = BestPracticesEnforcer::new();
        let scenario = "application development";
        
        let result = enforcer.get_best_practices(scenario, None, None).await.unwrap();
        
        assert_eq!(result.scenario, scenario);
        assert!(!result.version.is_empty());
        assert!(!result.documentation_refs.is_empty());
    }

    #[tokio::test]
    async fn test_get_best_practices_by_category() {
        let enforcer = BestPracticesEnforcer::new();
        let scenario = "development";
        let category = "architecture";
        
        let result = enforcer.get_best_practices(scenario, Some(category), None).await.unwrap();
        
        assert_eq!(result.scenario, scenario);
        // Should filter practices by architecture category
        for practice in &result.practices {
            assert_eq!(practice.category, category);
        }
    }

    #[tokio::test]
    async fn test_get_best_practices_with_version() {
        let enforcer = BestPracticesEnforcer::new();
        let scenario = "development";
        let version = "1.0.0";
        
        let result = enforcer.get_best_practices(scenario, None, Some(version)).await.unwrap();
        
        assert_eq!(result.version, version);
    }

    #[tokio::test]
    async fn test_validation_rules_creation() {
        let rules = ValidationRules::new();
        
        assert!(!rules.architecture_rules.is_empty());
        assert!(!rules.code_pattern_rules.is_empty());
        assert!(!rules.best_practice_rules.is_empty());
        
        // Check that we have expected rule categories
        let has_error_handling = rules.architecture_rules.iter()
            .any(|r| r.category == "error_handling");
        assert!(has_error_handling);
        
        let has_unwrap_rule = rules.code_pattern_rules.iter()
            .any(|r| r.pattern.contains("unwrap"));
        assert!(has_unwrap_rule);
    }

    #[tokio::test]
    async fn test_pattern_matcher_creation() {
        let matcher = PatternMatcher::new();
        
        assert!(!matcher.architecture_patterns.is_empty());
        assert!(!matcher.code_patterns.is_empty());
        
        // Check for expected patterns
        assert!(matcher.architecture_patterns.contains_key("adk_standard"));
        assert!(matcher.code_patterns.contains_key("error_handling"));
    }

    #[tokio::test]
    async fn test_validation_severity_levels() {
        let rules = ValidationRules::new();
        
        // Should have rules of different severity levels
        let has_error = rules.architecture_rules.iter()
            .any(|r| r.severity == ValidationSeverity::Error);
        let has_warning = rules.architecture_rules.iter()
            .any(|r| r.severity == ValidationSeverity::Warning);
        
        assert!(has_error);
        assert!(has_warning);
    }

    #[tokio::test]
    async fn test_architecture_validation_result_structure() {
        let enforcer = BestPracticesEnforcer::new();
        let description = "Test architecture description";
        
        let result = enforcer.validate_architecture(description, None, None).await.unwrap();
        
        // Verify result structure
        assert!(result.compliance_score <= 100);
        assert!(!result.documentation_refs.is_empty());
        
        // Check that documentation refs contain expected URLs
        let has_quickstart = result.documentation_refs.iter()
            .any(|url| url.contains("quickstart"));
        assert!(has_quickstart);
    }

    #[tokio::test]
    async fn test_best_practices_result_structure() {
        let enforcer = BestPracticesEnforcer::new();
        let scenario = "test scenario";
        
        let result = enforcer.get_best_practices(scenario, None, None).await.unwrap();
        
        // Verify result structure
        assert_eq!(result.scenario, scenario);
        assert!(!result.version.is_empty());
        assert!(!result.documentation_refs.is_empty());
        
        // Check that documentation refs are valid URLs
        for doc_ref in &result.documentation_refs {
            assert!(doc_ref.starts_with("http"));
        }
    }

    #[tokio::test]
    async fn test_enforcer_with_custom_knowledge_base() {
        let knowledge_base = AdkKnowledgeBase::new();
        let enforcer = BestPracticesEnforcer::with_knowledge_base(knowledge_base);
        
        let result = enforcer.get_best_practices("test", None, None).await.unwrap();
        assert!(!result.version.is_empty());
    }

    #[tokio::test]
    async fn test_validation_finding_creation() {
        let enforcer = BestPracticesEnforcer::new();
        let description = "Application with panic-based error handling";
        
        let result = enforcer.validate_architecture(description, None, None).await.unwrap();
        
        if !result.findings.is_empty() {
            let finding = &result.findings[0];
            assert!(!finding.id.is_empty());
            assert!(!finding.rule_id.is_empty());
            assert!(!finding.description.is_empty());
        }
    }

    #[tokio::test]
    async fn test_recommendation_generation() {
        let enforcer = BestPracticesEnforcer::new();
        let code_snippets = vec!["panic!(\"error\")".to_string()];
        
        let result = enforcer.validate_architecture("test", Some(&code_snippets), None).await.unwrap();
        
        if !result.recommendations.is_empty() {
            let rec = &result.recommendations[0];
            assert!(!rec.id.is_empty());
            assert!(!rec.description.is_empty());
            assert!(!rec.implementation_steps.is_empty());
            assert!(rec.priority >= 1 && rec.priority <= 5);
        }
    }
}