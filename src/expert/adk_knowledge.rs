//! Google ADK knowledge base and version management

use std::collections::HashMap;

/// ADK knowledge base structure
pub struct AdkKnowledgeBase {
    /// Version-specific documentation references
    pub version_docs: HashMap<String, VersionDocs>,
    /// Current default version
    pub default_version: String,
}

/// Documentation references for a specific ADK version
#[derive(Clone)]
pub struct VersionDocs {
    /// Version identifier
    pub version: String,
    /// Official documentation URLs
    pub official_urls: Vec<String>,
    /// Key concepts and features
    pub concepts: HashMap<String, String>,
    /// Best practices for this version
    pub best_practices: Vec<String>,
}

impl AdkKnowledgeBase {
    /// Create a new knowledge base with default ADK information
    pub fn new() -> Self {
        let mut version_docs = HashMap::new();
        
        // Initialize with latest version information
        let latest_docs = VersionDocs {
            version: "latest".to_string(),
            official_urls: vec![
                "https://google.github.io/adk-docs/get-started/quickstart/".to_string(),
                // Additional official URLs will be added as the knowledge base grows
            ],
            concepts: HashMap::new(), // Will be populated with ADK concepts
            best_practices: Vec::new(), // Will be populated with official best practices
        };
        
        version_docs.insert("latest".to_string(), latest_docs);
        
        Self {
            version_docs,
            default_version: "latest".to_string(),
        }
    }
    
    /// Get documentation for a specific version
    pub fn get_version_docs(&self, version: &str) -> Option<&VersionDocs> {
        self.version_docs.get(version)
            .or_else(|| self.version_docs.get(&self.default_version))
    }
    
    /// Add or update version documentation
    pub fn update_version_docs(&mut self, version: String, docs: VersionDocs) {
        self.version_docs.insert(version, docs);
    }
}

impl Default for AdkKnowledgeBase {
    fn default() -> Self {
        Self::new()
    }
}