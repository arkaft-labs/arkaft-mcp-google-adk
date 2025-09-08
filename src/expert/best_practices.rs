//! Best Practices Enforcement System for Google ADK
//! 
//! Provides comprehensive validation and enforcement of Google ADK best practices,
//! architectural patterns, and official guidelines.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::expert::adk_knowledge::{BestPractice, ImplementationPattern, AdkKnowledgeBase};

/// Best Practices Enforcement System for Google ADK
#[derive(Clone, Debug)]
pub struct BestPracticesEnforcer {
    /// Knowledge base for ADK information
    pub knowledge_base: AdkKnowledgeBase,
    /// Validation rules for architectural patterns
    pub validation_rules: ValidationRules,
    /// Pattern matching engine for best practice enforcement
    pub pattern_matcher: PatternMatcher,
}

/// Validation rules for architectural patterns and best practices
#[derive(Clone, Debug)]
pub struct ValidationRules {
    /// Architecture validation rules
    pub architecture_rules: Vec<ArchitectureRule>,
    /// Code pattern validation rules
    pub code_pattern_rules: Vec<CodePatternRule>,
    /// Best practice enforcement rules
    pub best_practice_rules: Vec<BestPracticeRule>,
}

/// Architecture validation rule
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArchitectureRule {
    /// Rule identifier
    pub id: String,
    /// Rule name
    pub name: String,
    /// Rule description
    pub description: String,
    /// Category (architecture, performance, security, etc.)
    pub category: String,
    /// Severity level (error, warning, info)
    pub severity: ValidationSeverity,
    /// Pattern to match against
    pub pattern: String,
    /// Recommendation for compliance
    pub recommendation: String,
    /// Official documentation reference
    pub documentation_ref: String,
}

/// Code pattern validation rule
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CodePatternRule {
    /// Rule identifier
    pub id: String,
    /// Rule name
    pub name: String,
    /// Pattern to detect (regex or keyword)
    pub pattern: String,
    /// Expected replacement or improvement
    pub expected_pattern: String,
    /// Explanation of why this pattern should be used
    pub rationale: String,
    /// Category of the rule
    pub category: String,
    /// Severity level
    pub severity: ValidationSeverity,
}

/// Best practice enforcement rule
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BestPracticeRule {
    /// Rule identifier
    pub id: String,
    /// Associated best practice
    pub practice_id: String,
    /// Validation logic description
    pub validation_logic: String,
    /// Success criteria
    pub success_criteria: Vec<String>,
    /// Failure indicators
    pub failure_indicators: Vec<String>,
    /// Remediation steps
    pub remediation_steps: Vec<String>,
}

/// Validation severity levels
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ValidationSeverity {
    /// Critical issues that must be fixed
    Error,
    /// Important issues that should be addressed
    Warning,
    /// Suggestions for improvement
    Info,
}

/// Pattern matching engine for best practice enforcement
#[derive(Clone, Debug)]
pub struct PatternMatcher {
    /// Architectural patterns to match
    pub architecture_patterns: HashMap<String, ArchitecturePattern>,
    /// Code patterns to detect
    pub code_patterns: HashMap<String, CodePattern>,
}

/// Architecture pattern definition
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArchitecturePattern {
    /// Pattern name
    pub name: String,
    /// Pattern description
    pub description: String,
    /// Required components
    pub required_components: Vec<String>,
    /// Optional components
    pub optional_components: Vec<String>,
    /// Anti-patterns to avoid
    pub anti_patterns: Vec<String>,
    /// Validation criteria
    pub validation_criteria: Vec<String>,
}

/// Code pattern definition
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CodePattern {
    /// Pattern name
    pub name: String,
    /// Pattern regex or identifier
    pub pattern: String,
    /// Expected usage context
    pub context: String,
    /// Compliance indicators
    pub compliance_indicators: Vec<String>,
    /// Non-compliance indicators
    pub non_compliance_indicators: Vec<String>,
}

/// Architecture validation result
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArchitectureValidationResult {
    /// Overall compliance status
    pub is_compliant: bool,
    /// Validation score (0-100)
    pub compliance_score: u8,
    /// Detailed findings
    pub findings: Vec<ValidationFinding>,
    /// Recommendations for improvement
    pub recommendations: Vec<Recommendation>,
    /// Official documentation references
    pub documentation_refs: Vec<String>,
}

/// Individual validation finding
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValidationFinding {
    /// Finding identifier
    pub id: String,
    /// Rule that generated this finding
    pub rule_id: String,
    /// Severity level
    pub severity: ValidationSeverity,
    /// Finding description
    pub description: String,
    /// Location or context where found
    pub location: Option<String>,
    /// Suggested fix
    pub suggested_fix: Option<String>,
}

/// Recommendation for improvement
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Recommendation {
    /// Recommendation identifier
    pub id: String,
    /// Category
    pub category: String,
    /// Priority level (1-5, 1 being highest)
    pub priority: u8,
    /// Recommendation description
    pub description: String,
    /// Implementation steps
    pub implementation_steps: Vec<String>,
    /// Expected benefits
    pub benefits: Vec<String>,
    /// Official documentation reference
    pub documentation_ref: String,
}

/// Best practices retrieval result
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BestPracticesResult {
    /// Scenario or category requested
    pub scenario: String,
    /// Applicable best practices
    pub practices: Vec<BestPractice>,
    /// Implementation patterns
    pub patterns: Vec<ImplementationPattern>,
    /// Official documentation references
    pub documentation_refs: Vec<String>,
    /// Version information
    pub version: String,
}

impl BestPracticesEnforcer {
    /// Create a new Best Practices Enforcer with default configuration
    pub fn new() -> Self {
        let knowledge_base = AdkKnowledgeBase::new();
        let validation_rules = ValidationRules::new();
        let pattern_matcher = PatternMatcher::new();
        
        Self {
            knowledge_base,
            validation_rules,
            pattern_matcher,
        }
    }
    
    /// Create enforcer with custom knowledge base
    pub fn with_knowledge_base(knowledge_base: AdkKnowledgeBase) -> Self {
        let validation_rules = ValidationRules::new();
        let pattern_matcher = PatternMatcher::new();
        
        Self {
            knowledge_base,
            validation_rules,
            pattern_matcher,
        }
    }
    
    /// Validate architecture against Google ADK best practices
    pub async fn validate_architecture(
        &self,
        description: &str,
        code_snippets: Option<&[String]>,
        version: Option<&str>,
    ) -> anyhow::Result<ArchitectureValidationResult> {
        let resolved_version = version
            .map(|v| self.knowledge_base.resolve_version(v))
            .unwrap_or_else(|| self.knowledge_base.default_version.clone());
        
        let mut findings = Vec::new();
        let mut compliance_score = 100u8;
        
        // Validate against architecture rules
        for rule in &self.validation_rules.architecture_rules {
            if let Some(finding) = self.check_architecture_rule(rule, description, &resolved_version) {
                // Reduce compliance score based on severity
                match finding.severity {
                    ValidationSeverity::Error => compliance_score = compliance_score.saturating_sub(20),
                    ValidationSeverity::Warning => compliance_score = compliance_score.saturating_sub(10),
                    ValidationSeverity::Info => compliance_score = compliance_score.saturating_sub(5),
                }
                findings.push(finding);
            }
        }
        
        // Validate code snippets if provided
        if let Some(snippets) = code_snippets {
            for (index, snippet) in snippets.iter().enumerate() {
                let snippet_findings = self.validate_code_snippet(snippet, index, &resolved_version);
                for finding in snippet_findings {
                    match finding.severity {
                        ValidationSeverity::Error => compliance_score = compliance_score.saturating_sub(15),
                        ValidationSeverity::Warning => compliance_score = compliance_score.saturating_sub(8),
                        ValidationSeverity::Info => compliance_score = compliance_score.saturating_sub(3),
                    }
                    findings.push(finding);
                }
            }
        }
        
        // Generate recommendations based on findings
        let recommendations = self.generate_recommendations(&findings, &resolved_version);
        
        // Get official documentation references
        let documentation_refs = self.get_architecture_documentation_refs(&resolved_version);
        
        let is_compliant = compliance_score >= 80 && !findings.iter().any(|f| f.severity == ValidationSeverity::Error);
        
        Ok(ArchitectureValidationResult {
            is_compliant,
            compliance_score,
            findings,
            recommendations,
            documentation_refs,
        })
    }
    
    /// Get best practices for specific scenario
    pub async fn get_best_practices(
        &self,
        scenario: &str,
        category: Option<&str>,
        version: Option<&str>,
    ) -> anyhow::Result<BestPracticesResult> {
        let resolved_version = version
            .map(|v| self.knowledge_base.resolve_version(v))
            .unwrap_or_else(|| self.knowledge_base.default_version.clone());
        
        // Get practices by category if specified, otherwise get all relevant practices
        let practices = if let Some(cat) = category {
            self.knowledge_base.get_best_practices_by_category(cat, Some(&resolved_version))
        } else {
            // Get practices relevant to the scenario
            self.get_scenario_relevant_practices(scenario, &resolved_version)
        };
        
        // Get relevant implementation patterns
        let patterns = self.get_scenario_patterns(scenario, &resolved_version);
        
        // Get official documentation references
        let documentation_refs = self.get_best_practices_documentation_refs(scenario, &resolved_version);
        
        Ok(BestPracticesResult {
            scenario: scenario.to_string(),
            practices: practices.into_iter().cloned().collect(),
            patterns,
            documentation_refs,
            version: resolved_version,
        })
    }
    
    /// Check a single architecture rule against the description
    fn check_architecture_rule(
        &self,
        rule: &ArchitectureRule,
        description: &str,
        _version: &str,
    ) -> Option<ValidationFinding> {
        // Simple pattern matching - in a real implementation this would be more sophisticated
        let description_lower = description.to_lowercase();
        let pattern_lower = rule.pattern.to_lowercase();
        
        // Check if the pattern indicates a potential issue
        if description_lower.contains(&pattern_lower) {
            Some(ValidationFinding {
                id: format!("arch_{}", rule.id),
                rule_id: rule.id.clone(),
                severity: rule.severity.clone(),
                description: format!("{}: {}", rule.name, rule.description),
                location: Some("Architecture Description".to_string()),
                suggested_fix: Some(rule.recommendation.clone()),
            })
        } else {
            None
        }
    }
    
    /// Validate a code snippet against best practices
    fn validate_code_snippet(&self, snippet: &str, index: usize, _version: &str) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();
        
        // Check against code pattern rules
        for rule in &self.validation_rules.code_pattern_rules {
            if snippet.contains(&rule.pattern) {
                findings.push(ValidationFinding {
                    id: format!("code_{}_{}", index, rule.id),
                    rule_id: rule.id.clone(),
                    severity: rule.severity.clone(),
                    description: format!("{}: {}", rule.name, rule.rationale),
                    location: Some(format!("Code Snippet {}", index + 1)),
                    suggested_fix: Some(format!("Consider using: {}", rule.expected_pattern)),
                });
            }
        }
        
        findings
    }
    
    /// Generate recommendations based on validation findings
    fn generate_recommendations(&self, findings: &[ValidationFinding], version: &str) -> Vec<Recommendation> {
        let mut recommendations = Vec::new();
        let mut rec_id = 1;
        
        // Group findings by category and generate recommendations
        let error_count = findings.iter().filter(|f| f.severity == ValidationSeverity::Error).count();
        let warning_count = findings.iter().filter(|f| f.severity == ValidationSeverity::Warning).count();
        
        if error_count > 0 {
            recommendations.push(Recommendation {
                id: format!("rec_{}", rec_id),
                category: "Critical Issues".to_string(),
                priority: 1,
                description: format!("Address {} critical architecture issues that prevent ADK compliance", error_count),
                implementation_steps: vec![
                    "Review all error-level findings".to_string(),
                    "Implement suggested fixes for critical issues".to_string(),
                    "Validate changes against ADK guidelines".to_string(),
                ],
                benefits: vec![
                    "Ensures ADK compliance".to_string(),
                    "Prevents runtime issues".to_string(),
                    "Follows official best practices".to_string(),
                ],
                documentation_ref: format!("https://google.github.io/adk-docs/best-practices/?version={}", version),
            });
            rec_id += 1;
        }
        
        if warning_count > 0 {
            recommendations.push(Recommendation {
                id: format!("rec_{}", rec_id),
                category: "Improvements".to_string(),
                priority: 2,
                description: format!("Consider addressing {} warning-level improvements for better ADK alignment", warning_count),
                implementation_steps: vec![
                    "Review warning-level findings".to_string(),
                    "Prioritize improvements based on impact".to_string(),
                    "Implement changes incrementally".to_string(),
                ],
                benefits: vec![
                    "Improves code quality".to_string(),
                    "Better alignment with ADK patterns".to_string(),
                    "Enhanced maintainability".to_string(),
                ],
                documentation_ref: format!("https://google.github.io/adk-docs/best-practices/?version={}", version),
            });
        }
        
        recommendations
    }
    
    /// Get practices relevant to a specific scenario
    fn get_scenario_relevant_practices(&self, scenario: &str, version: &str) -> Vec<&BestPractice> {
        if let Some(docs) = self.knowledge_base.get_version_docs(version) {
            docs.best_practices
                .iter()
                .filter(|practice| {
                    let scenario_lower = scenario.to_lowercase();
                    practice.title.to_lowercase().contains(&scenario_lower) ||
                    practice.description.to_lowercase().contains(&scenario_lower) ||
                    practice.category.to_lowercase().contains(&scenario_lower)
                })
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Get implementation patterns relevant to a scenario
    fn get_scenario_patterns(&self, scenario: &str, version: &str) -> Vec<ImplementationPattern> {
        if let Some(docs) = self.knowledge_base.get_version_docs(version) {
            docs.implementation_patterns
                .values()
                .filter(|pattern| {
                    let scenario_lower = scenario.to_lowercase();
                    pattern.name.to_lowercase().contains(&scenario_lower) ||
                    pattern.description.to_lowercase().contains(&scenario_lower) ||
                    pattern.use_cases.iter().any(|uc| uc.to_lowercase().contains(&scenario_lower))
                })
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Get architecture documentation references
    fn get_architecture_documentation_refs(&self, version: &str) -> Vec<String> {
        if let Some(urls) = self.knowledge_base.get_official_urls(Some(version)) {
            let mut refs = vec![urls.quickstart.clone()];
            refs.extend(urls.best_practices.clone());
            refs
        } else {
            vec!["https://google.github.io/adk-docs/get-started/quickstart/".to_string()]
        }
    }
    
    /// Get best practices documentation references
    fn get_best_practices_documentation_refs(&self, _scenario: &str, version: &str) -> Vec<String> {
        if let Some(urls) = self.knowledge_base.get_official_urls(Some(version)) {
            let mut refs = vec![urls.quickstart.clone()];
            refs.extend(urls.best_practices.clone());
            refs.extend(urls.tutorials.clone());
            refs
        } else {
            vec!["https://google.github.io/adk-docs/get-started/quickstart/".to_string()]
        }
    }
}

impl ValidationRules {
    /// Create new validation rules with Google ADK defaults
    pub fn new() -> Self {
        Self {
            architecture_rules: Self::create_default_architecture_rules(),
            code_pattern_rules: Self::create_default_code_pattern_rules(),
            best_practice_rules: Self::create_default_best_practice_rules(),
        }
    }
    
    /// Create default architecture validation rules
    fn create_default_architecture_rules() -> Vec<ArchitectureRule> {
        vec![
            ArchitectureRule {
                id: "adk_structure".to_string(),
                name: "ADK Project Structure".to_string(),
                description: "Project should follow official ADK structure guidelines".to_string(),
                category: "architecture".to_string(),
                severity: ValidationSeverity::Warning,
                pattern: "non-standard".to_string(),
                recommendation: "Follow the official ADK project structure as documented in the quickstart guide".to_string(),
                documentation_ref: "https://google.github.io/adk-docs/get-started/quickstart/".to_string(),
            },
            ArchitectureRule {
                id: "async_patterns".to_string(),
                name: "Async Pattern Usage".to_string(),
                description: "Should use proper async patterns as recommended by ADK".to_string(),
                category: "architecture".to_string(),
                severity: ValidationSeverity::Error,
                pattern: "blocking operations".to_string(),
                recommendation: "Use async/await patterns and non-blocking operations as specified in ADK guidelines".to_string(),
                documentation_ref: "https://google.github.io/adk-docs/best-practices/".to_string(),
            },
            ArchitectureRule {
                id: "error_handling".to_string(),
                name: "Error Handling Patterns".to_string(),
                description: "Should implement proper error handling following ADK conventions".to_string(),
                category: "error_handling".to_string(),
                severity: ValidationSeverity::Error,
                pattern: "panic".to_string(),
                recommendation: "Use Result types and proper error propagation instead of panic! calls".to_string(),
                documentation_ref: "https://google.github.io/adk-docs/best-practices/".to_string(),
            },
        ]
    }
    
    /// Create default code pattern validation rules
    fn create_default_code_pattern_rules() -> Vec<CodePatternRule> {
        vec![
            CodePatternRule {
                id: "unwrap_usage".to_string(),
                name: "Avoid unwrap() calls".to_string(),
                pattern: ".unwrap()".to_string(),
                expected_pattern: "proper error handling with ? operator or match".to_string(),
                rationale: "unwrap() can cause panics; use proper error handling instead".to_string(),
                category: "error_handling".to_string(),
                severity: ValidationSeverity::Warning,
            },
            CodePatternRule {
                id: "panic_usage".to_string(),
                name: "Avoid panic! macro".to_string(),
                pattern: "panic!".to_string(),
                expected_pattern: "Result<T, E> return types with proper error handling".to_string(),
                rationale: "panic! should be avoided in favor of recoverable error handling".to_string(),
                category: "error_handling".to_string(),
                severity: ValidationSeverity::Error,
            },
            CodePatternRule {
                id: "todo_usage".to_string(),
                name: "Remove TODO markers".to_string(),
                pattern: "todo!".to_string(),
                expected_pattern: "complete implementation".to_string(),
                rationale: "TODO markers indicate incomplete implementation".to_string(),
                category: "completeness".to_string(),
                severity: ValidationSeverity::Info,
            },
        ]
    }
    
    /// Create default best practice enforcement rules
    fn create_default_best_practice_rules() -> Vec<BestPracticeRule> {
        vec![
            BestPracticeRule {
                id: "official_patterns".to_string(),
                practice_id: "follow_official_patterns".to_string(),
                validation_logic: "Check adherence to official ADK architectural patterns".to_string(),
                success_criteria: vec![
                    "Uses recommended project structure".to_string(),
                    "Follows naming conventions".to_string(),
                    "Implements proper async patterns".to_string(),
                ],
                failure_indicators: vec![
                    "Non-standard directory structure".to_string(),
                    "Inconsistent naming".to_string(),
                    "Blocking operations in async context".to_string(),
                ],
                remediation_steps: vec![
                    "Review official ADK documentation".to_string(),
                    "Restructure project to match guidelines".to_string(),
                    "Update code to use recommended patterns".to_string(),
                ],
            },
        ]
    }
}

impl PatternMatcher {
    /// Create new pattern matcher with default patterns
    pub fn new() -> Self {
        Self {
            architecture_patterns: Self::create_default_architecture_patterns(),
            code_patterns: Self::create_default_code_patterns(),
        }
    }
    
    /// Create default architecture patterns
    fn create_default_architecture_patterns() -> HashMap<String, ArchitecturePattern> {
        let mut patterns = HashMap::new();
        
        patterns.insert("adk_standard".to_string(), ArchitecturePattern {
            name: "Standard ADK Architecture".to_string(),
            description: "Standard architectural pattern recommended by Google ADK".to_string(),
            required_components: vec![
                "Proper project structure".to_string(),
                "Configuration management".to_string(),
                "Error handling".to_string(),
            ],
            optional_components: vec![
                "Logging framework".to_string(),
                "Monitoring integration".to_string(),
            ],
            anti_patterns: vec![
                "Blocking operations in async context".to_string(),
                "Panic-based error handling".to_string(),
            ],
            validation_criteria: vec![
                "Follows ADK project structure".to_string(),
                "Uses async/await patterns".to_string(),
                "Implements proper error handling".to_string(),
            ],
        });
        
        patterns
    }
    
    /// Create default code patterns
    fn create_default_code_patterns() -> HashMap<String, CodePattern> {
        let mut patterns = HashMap::new();
        
        patterns.insert("error_handling".to_string(), CodePattern {
            name: "Proper Error Handling".to_string(),
            pattern: r"Result<.*>".to_string(),
            context: "Function return types and error propagation".to_string(),
            compliance_indicators: vec![
                "Uses Result<T, E> return types".to_string(),
                "Proper error propagation with ? operator".to_string(),
            ],
            non_compliance_indicators: vec![
                "Uses unwrap() or expect()".to_string(),
                "Uses panic! macro".to_string(),
            ],
        });
        
        patterns
    }
}

impl Default for BestPracticesEnforcer {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ValidationRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for PatternMatcher {
    fn default() -> Self {
        Self::new()
    }
}