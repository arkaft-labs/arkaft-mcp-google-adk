//! Google ADK knowledge base and version management

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// ADK knowledge base structure for storing comprehensive documentation knowledge
#[derive(Clone, Debug)]
pub struct AdkKnowledgeBase {
    /// Version-specific documentation references
    pub version_docs: HashMap<String, VersionDocs>,
    /// Current default version
    pub default_version: String,
    /// Configuration for version tracking
    pub version_config: VersionConfig,
}

/// Documentation references for a specific ADK version
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VersionDocs {
    /// Version identifier
    pub version: String,
    /// Official documentation URLs with categorization
    pub official_urls: DocumentationUrls,
    /// Key concepts and features with detailed explanations
    pub concepts: HashMap<String, ConceptInfo>,
    /// Best practices for this version
    pub best_practices: Vec<BestPractice>,
    /// Implementation patterns and examples
    pub implementation_patterns: HashMap<String, ImplementationPattern>,
    /// Version-specific features and changes
    pub version_features: Vec<VersionFeature>,
}

/// Categorized official documentation URLs
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DocumentationUrls {
    /// Main quickstart guide
    pub quickstart: String,
    /// API reference documentation
    pub api_reference: Vec<String>,
    /// Tutorial and guide URLs
    pub tutorials: Vec<String>,
    /// Best practices documentation
    pub best_practices: Vec<String>,
    /// Migration guides
    pub migration_guides: Vec<String>,
}

/// Detailed concept information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConceptInfo {
    /// Concept name
    pub name: String,
    /// Detailed description
    pub description: String,
    /// Usage examples
    pub examples: Vec<String>,
    /// Related concepts
    pub related_concepts: Vec<String>,
    /// Official documentation references
    pub documentation_refs: Vec<String>,
}

/// Best practice information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BestPractice {
    /// Practice title
    pub title: String,
    /// Detailed description
    pub description: String,
    /// Category (architecture, performance, security, etc.)
    pub category: String,
    /// Code examples demonstrating the practice
    pub examples: Vec<String>,
    /// Official documentation reference
    pub documentation_ref: String,
}

/// Implementation pattern information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImplementationPattern {
    /// Pattern name
    pub name: String,
    /// Pattern description
    pub description: String,
    /// Use cases where this pattern applies
    pub use_cases: Vec<String>,
    /// Code examples
    pub code_examples: Vec<CodeExample>,
    /// Related best practices
    pub related_practices: Vec<String>,
}

/// Code example with context
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CodeExample {
    /// Example title
    pub title: String,
    /// Programming language
    pub language: String,
    /// Code content
    pub code: String,
    /// Explanation of the example
    pub explanation: String,
}

/// Version-specific feature information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VersionFeature {
    /// Feature name
    pub name: String,
    /// Feature description
    pub description: String,
    /// Version introduced
    pub introduced_in: String,
    /// Deprecation information if applicable
    pub deprecated_in: Option<String>,
    /// Migration notes
    pub migration_notes: Option<String>,
}

/// Configuration for version management and tracking
#[derive(Clone, Debug)]
pub struct VersionConfig {
    /// Default ADK version to use when none specified
    pub default_version: String,
    /// Available versions in order of preference
    pub available_versions: Vec<String>,
    /// Version aliases (e.g., "stable" -> "1.2.0")
    pub version_aliases: HashMap<String, String>,
    /// Auto-update configuration
    pub auto_update_enabled: bool,
}

impl AdkKnowledgeBase {
    /// Create a new knowledge base with default ADK information
    pub fn new() -> Self {
        let mut version_docs = HashMap::new();
        
        // Initialize version configuration
        let version_config = VersionConfig::new();
        let default_version = version_config.resolve_version("latest");
        
        // Initialize with latest version information
        let latest_docs = VersionDocs {
            version: default_version.clone(),
            official_urls: DocumentationUrls::default(),
            concepts: Self::initialize_default_concepts(),
            best_practices: Self::initialize_default_best_practices(),
            implementation_patterns: Self::initialize_default_patterns(),
            version_features: Vec::new(),
        };
        
        version_docs.insert(default_version.clone(), latest_docs);
        
        Self {
            version_docs,
            default_version,
            version_config,
        }
    }
    
    /// Create knowledge base with custom version configuration
    pub fn with_version_config(config: VersionConfig) -> Self {
        let mut kb = Self::new();
        kb.version_config = config;
        kb.default_version = kb.version_config.resolve_version("latest");
        kb
    }
    
    /// Get documentation for a specific version with fallback to default
    pub fn get_version_docs(&self, version: &str) -> Option<&VersionDocs> {
        let resolved_version = self.version_config.resolve_version(version);
        self.version_docs.get(&resolved_version)
            .or_else(|| self.version_docs.get(&self.default_version))
    }
    
    /// Add or update version documentation
    pub fn update_version_docs(&mut self, version: String, docs: VersionDocs) {
        self.version_docs.insert(version, docs);
    }
    
    /// Get available versions
    pub fn get_available_versions(&self) -> Vec<String> {
        self.version_config.available_versions.clone()
    }
    
    /// Resolve version alias to actual version
    pub fn resolve_version(&self, version: &str) -> String {
        self.version_config.resolve_version(version)
    }
    
    /// Search concepts by query string
    pub fn search_concepts(&self, query: &str, version: Option<&str>) -> Vec<&ConceptInfo> {
        let version = version.unwrap_or(&self.default_version);
        if let Some(docs) = self.get_version_docs(version) {
            docs.concepts
                .values()
                .filter(|concept| {
                    concept.name.to_lowercase().contains(&query.to_lowercase()) ||
                    concept.description.to_lowercase().contains(&query.to_lowercase())
                })
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Get best practices by category
    pub fn get_best_practices_by_category(&self, category: &str, version: Option<&str>) -> Vec<&BestPractice> {
        let version = version.unwrap_or(&self.default_version);
        if let Some(docs) = self.get_version_docs(version) {
            docs.best_practices
                .iter()
                .filter(|practice| practice.category.eq_ignore_ascii_case(category))
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Get implementation pattern by name
    pub fn get_implementation_pattern(&self, pattern_name: &str, version: Option<&str>) -> Option<&ImplementationPattern> {
        let version = version.unwrap_or(&self.default_version);
        self.get_version_docs(version)?
            .implementation_patterns
            .get(pattern_name)
    }
    
    /// Get official documentation URLs for version
    pub fn get_official_urls(&self, version: Option<&str>) -> Option<&DocumentationUrls> {
        let version = version.unwrap_or(&self.default_version);
        self.get_version_docs(version).map(|docs| &docs.official_urls)
    }
    
    /// Initialize default ADK concepts
    fn initialize_default_concepts() -> HashMap<String, ConceptInfo> {
        let mut concepts = HashMap::new();
        
        // Core ADK concepts - these will be expanded with actual Google ADK information
        concepts.insert("application_development_kit".to_string(), ConceptInfo {
            name: "Application Development Kit (ADK)".to_string(),
            description: "Google's comprehensive toolkit for building applications with best practices and official patterns.".to_string(),
            examples: vec![
                "Basic ADK project setup".to_string(),
                "ADK configuration patterns".to_string(),
            ],
            related_concepts: vec!["best_practices".to_string(), "architecture_patterns".to_string()],
            documentation_refs: vec!["https://google.github.io/adk-docs/get-started/quickstart/".to_string()],
        });
        
        concepts.insert("best_practices".to_string(), ConceptInfo {
            name: "ADK Best Practices".to_string(),
            description: "Official Google ADK recommended practices for application development.".to_string(),
            examples: vec![
                "Code organization patterns".to_string(),
                "Performance optimization techniques".to_string(),
            ],
            related_concepts: vec!["architecture_patterns".to_string(), "application_development_kit".to_string()],
            documentation_refs: vec!["https://google.github.io/adk-docs/get-started/quickstart/".to_string()],
        });
        
        concepts
    }
    
    /// Initialize default best practices
    fn initialize_default_best_practices() -> Vec<BestPractice> {
        vec![
            BestPractice {
                title: "Follow Official ADK Patterns".to_string(),
                description: "Always use official Google ADK architectural patterns and conventions.".to_string(),
                category: "architecture".to_string(),
                examples: vec![
                    "Use recommended project structure".to_string(),
                    "Follow naming conventions".to_string(),
                ],
                documentation_ref: "https://google.github.io/adk-docs/get-started/quickstart/".to_string(),
            },
            BestPractice {
                title: "Version-Aware Development".to_string(),
                description: "Always specify and track ADK versions for consistent development.".to_string(),
                category: "versioning".to_string(),
                examples: vec![
                    "Pin ADK version in configuration".to_string(),
                    "Use version-specific features appropriately".to_string(),
                ],
                documentation_ref: "https://google.github.io/adk-docs/get-started/quickstart/".to_string(),
            },
        ]
    }
    
    /// Initialize default implementation patterns
    fn initialize_default_patterns() -> HashMap<String, ImplementationPattern> {
        let mut patterns = HashMap::new();
        
        patterns.insert("basic_setup".to_string(), ImplementationPattern {
            name: "Basic ADK Setup".to_string(),
            description: "Standard pattern for setting up a new ADK project.".to_string(),
            use_cases: vec![
                "New project initialization".to_string(),
                "ADK integration into existing projects".to_string(),
            ],
            code_examples: vec![
                CodeExample {
                    title: "Basic Configuration".to_string(),
                    language: "rust".to_string(),
                    code: "// ADK setup example - to be populated with actual patterns".to_string(),
                    explanation: "Basic ADK project configuration following official guidelines.".to_string(),
                },
            ],
            related_practices: vec!["Follow Official ADK Patterns".to_string()],
        });
        
        patterns
    }
}

impl Default for AdkKnowledgeBase {
    fn default() -> Self {
        Self::new()
    }
}

impl VersionConfig {
    /// Create new version configuration with defaults
    pub fn new() -> Self {
        let default_version = std::env::var("ADK_DOCS_VERSION")
            .unwrap_or_else(|_| "latest".to_string());
            
        let mut version_aliases = HashMap::new();
        version_aliases.insert("latest".to_string(), "1.0.0".to_string());
        version_aliases.insert("stable".to_string(), "1.0.0".to_string());
        
        Self {
            default_version: default_version.clone(),
            available_versions: vec!["1.0.0".to_string(), "latest".to_string()],
            version_aliases,
            auto_update_enabled: true,
        }
    }
    
    /// Resolve version string to actual version, handling aliases
    pub fn resolve_version(&self, version: &str) -> String {
        self.version_aliases
            .get(version)
            .cloned()
            .unwrap_or_else(|| version.to_string())
    }
    
    /// Check if version is available
    pub fn is_version_available(&self, version: &str) -> bool {
        let resolved = self.resolve_version(version);
        self.available_versions.contains(&resolved)
    }
    
    /// Add new version to available versions
    pub fn add_version(&mut self, version: String) {
        if !self.available_versions.contains(&version) {
            self.available_versions.push(version);
        }
    }
    
    /// Set version alias
    pub fn set_alias(&mut self, alias: String, target_version: String) {
        self.version_aliases.insert(alias, target_version);
    }
}

impl Default for VersionConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl DocumentationUrls {
    /// Create new documentation URLs with Google ADK defaults
    pub fn new() -> Self {
        Self {
            quickstart: "https://google.github.io/adk-docs/get-started/quickstart/".to_string(),
            api_reference: vec![
                "https://google.github.io/adk-docs/api/".to_string(),
            ],
            tutorials: vec![
                "https://google.github.io/adk-docs/tutorials/".to_string(),
            ],
            best_practices: vec![
                "https://google.github.io/adk-docs/best-practices/".to_string(),
            ],
            migration_guides: vec![
                "https://google.github.io/adk-docs/migration/".to_string(),
            ],
        }
    }
    
    /// Get all URLs as a flat list
    pub fn get_all_urls(&self) -> Vec<String> {
        let mut urls = vec![self.quickstart.clone()];
        urls.extend(self.api_reference.clone());
        urls.extend(self.tutorials.clone());
        urls.extend(self.best_practices.clone());
        urls.extend(self.migration_guides.clone());
        urls
    }
    
    /// Get URLs by category
    pub fn get_urls_by_category(&self, category: &str) -> Vec<String> {
        match category.to_lowercase().as_str() {
            "quickstart" => vec![self.quickstart.clone()],
            "api" | "reference" => self.api_reference.clone(),
            "tutorials" | "guides" => self.tutorials.clone(),
            "best_practices" | "practices" => self.best_practices.clone(),
            "migration" => self.migration_guides.clone(),
            _ => self.get_all_urls(),
        }
    }
}

impl Default for DocumentationUrls {
    fn default() -> Self {
        Self::new()
    }
}