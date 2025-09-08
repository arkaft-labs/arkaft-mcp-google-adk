//! Documentation utilities and reference generation

use crate::expert::adk_knowledge::{AdkKnowledgeBase, DocumentationUrls, ConceptInfo};

/// Documentation reference generator for version-aware official links
pub struct DocumentationReferenceGenerator {
    knowledge_base: AdkKnowledgeBase,
}

impl DocumentationReferenceGenerator {
    /// Create new reference generator with knowledge base
    pub fn new(knowledge_base: AdkKnowledgeBase) -> Self {
        Self { knowledge_base }
    }
    
    /// Generate official documentation references for specific version
    pub fn generate_official_references(&self, version: Option<&str>) -> Vec<String> {
        if let Some(urls) = self.knowledge_base.get_official_urls(version) {
            urls.get_all_urls()
        } else {
            // Fallback to default URLs
            DocumentationUrls::default().get_all_urls()
        }
    }
    
    /// Generate category-specific references
    pub fn generate_category_references(&self, category: &str, version: Option<&str>) -> Vec<String> {
        if let Some(urls) = self.knowledge_base.get_official_urls(version) {
            urls.get_urls_by_category(category)
        } else {
            DocumentationUrls::default().get_urls_by_category(category)
        }
    }
    
    /// Generate references for specific concepts
    pub fn generate_concept_references(&self, concept_name: &str, version: Option<&str>) -> Vec<String> {
        let version_str = version.unwrap_or(&self.knowledge_base.default_version);
        
        if let Some(docs) = self.knowledge_base.get_version_docs(version_str) {
            if let Some(concept) = docs.concepts.get(concept_name) {
                return concept.documentation_refs.clone();
            }
        }
        
        // Fallback to general references
        self.generate_official_references(version)
    }
}

/// Format documentation response with proper references and version information
pub fn format_documentation_response(
    query: &str,
    content: &str,
    version: &str,
    references: &[String],
) -> String {
    format!(
        "## Google ADK Documentation Query: {}\n\n\
        **Version:** {}\n\n\
        {}\n\n\
        ### Official References:\n{}\n\n\
        *Information based on official Google ADK documentation (version: {})*",
        query,
        version,
        content,
        references
            .iter()
            .map(|url| format!("- [{}]({})", extract_url_title(url), url))
            .collect::<Vec<_>>()
            .join("\n"),
        version
    )
}

/// Format concept information with references
pub fn format_concept_response(
    concept: &ConceptInfo,
    version: &str,
) -> String {
    let examples_text = if concept.examples.is_empty() {
        String::new()
    } else {
        format!(
            "\n\n### Examples:\n{}",
            concept.examples
                .iter()
                .map(|example| format!("- {}", example))
                .collect::<Vec<_>>()
                .join("\n")
        )
    };
    
    let related_text = if concept.related_concepts.is_empty() {
        String::new()
    } else {
        format!(
            "\n\n### Related Concepts:\n{}",
            concept.related_concepts
                .iter()
                .map(|related| format!("- {}", related))
                .collect::<Vec<_>>()
                .join("\n")
        )
    };
    
    format!(
        "## {}\n\n\
        **Version:** {}\n\n\
        {}{}{}\n\n\
        ### Official References:\n{}\n\n\
        *Information based on official Google ADK documentation (version: {})*",
        concept.name,
        version,
        concept.description,
        examples_text,
        related_text,
        concept.documentation_refs
            .iter()
            .map(|url| format!("- [{}]({})", extract_url_title(url), url))
            .collect::<Vec<_>>()
            .join("\n"),
        version
    )
}

/// Extract a readable title from URL for display
fn extract_url_title(url: &str) -> String {
    if url.contains("quickstart") {
        "Google ADK Quickstart Guide".to_string()
    } else if url.contains("api") {
        "Google ADK API Reference".to_string()
    } else if url.contains("tutorials") {
        "Google ADK Tutorials".to_string()
    } else if url.contains("best-practices") {
        "Google ADK Best Practices".to_string()
    } else if url.contains("migration") {
        "Google ADK Migration Guide".to_string()
    } else {
        "Google ADK Documentation".to_string()
    }
}

/// Generate comprehensive documentation links for query results
pub fn generate_comprehensive_links(
    query: &str,
    version: &str,
    knowledge_base: &AdkKnowledgeBase,
) -> Vec<String> {
    let mut links = Vec::new();
    
    // Always include quickstart
    if let Some(urls) = knowledge_base.get_official_urls(Some(version)) {
        links.push(urls.quickstart.clone());
        
        // Add relevant category links based on query content
        if query.to_lowercase().contains("api") {
            links.extend(urls.api_reference.clone());
        }
        if query.to_lowercase().contains("tutorial") || query.to_lowercase().contains("guide") {
            links.extend(urls.tutorials.clone());
        }
        if query.to_lowercase().contains("best") || query.to_lowercase().contains("practice") {
            links.extend(urls.best_practices.clone());
        }
    }
    
    // Remove duplicates while preserving order
    let mut unique_links = Vec::new();
    for link in links {
        if !unique_links.contains(&link) {
            unique_links.push(link);
        }
    }
    
    unique_links
}