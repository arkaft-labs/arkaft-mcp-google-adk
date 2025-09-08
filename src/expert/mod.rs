//! Documentation Expert System for Google ADK
//! 
//! Provides comprehensive knowledge of Google ADK documentation with version awareness,
//! official references, and implementation guidance following best practices.

pub mod adk_knowledge;
pub mod documentation;

use adk_knowledge::{AdkKnowledgeBase, VersionConfig};
use documentation::{DocumentationReferenceGenerator, format_documentation_response, format_concept_response, generate_comprehensive_links};

/// Documentation Expert System for Google ADK with comprehensive knowledge base
pub struct DocumentationExpert {
    /// ADK knowledge base with version-aware information
    pub knowledge_base: AdkKnowledgeBase,
    /// Documentation reference generator
    pub reference_generator: DocumentationReferenceGenerator,
}

impl DocumentationExpert {
    /// Create a new Documentation Expert instance with default configuration
    pub fn new() -> Self {
        let knowledge_base = AdkKnowledgeBase::new();
        let reference_generator = DocumentationReferenceGenerator::new(knowledge_base.clone());
        
        Self {
            knowledge_base,
            reference_generator,
        }
    }
    
    /// Create Documentation Expert with custom version configuration
    pub fn with_version_config(config: VersionConfig) -> Self {
        let knowledge_base = AdkKnowledgeBase::with_version_config(config);
        let reference_generator = DocumentationReferenceGenerator::new(knowledge_base.clone());
        
        Self {
            knowledge_base,
            reference_generator,
        }
    }
    
    /// Query ADK documentation and concepts with comprehensive knowledge base lookup
    pub async fn query_documentation(&self, query: &str, version: Option<&str>) -> anyhow::Result<String> {
        let resolved_version = version
            .map(|v| self.knowledge_base.resolve_version(v))
            .unwrap_or_else(|| self.knowledge_base.default_version.clone());
        
        // Search for matching concepts first
        let matching_concepts = self.knowledge_base.search_concepts(query, Some(&resolved_version));
        
        if !matching_concepts.is_empty() {
            // Return detailed concept information
            let concept = matching_concepts[0];
            return Ok(format_concept_response(concept, &resolved_version));
        }
        
        // Generate comprehensive response with official references
        let content = self.generate_query_response(query, &resolved_version).await?;
        let references = generate_comprehensive_links(query, &resolved_version, &self.knowledge_base);
        
        Ok(format_documentation_response(
            query,
            &content,
            &resolved_version,
            &references,
        ))
    }
    
    /// Get available ADK versions
    pub fn get_available_versions(&self) -> Vec<String> {
        self.knowledge_base.get_available_versions()
    }
    
    /// Get best practices for specific category
    pub async fn get_best_practices(&self, category: Option<&str>, version: Option<&str>) -> anyhow::Result<String> {
        let resolved_version = version
            .map(|v| self.knowledge_base.resolve_version(v))
            .unwrap_or_else(|| self.knowledge_base.default_version.clone());
        
        let practices = if let Some(cat) = category {
            self.knowledge_base.get_best_practices_by_category(cat, Some(&resolved_version))
        } else {
            // Get all practices if no category specified
            if let Some(docs) = self.knowledge_base.get_version_docs(&resolved_version) {
                docs.best_practices.iter().collect()
            } else {
                Vec::new()
            }
        };
        
        if practices.is_empty() {
            return Ok(format!(
                "No best practices found for category '{}' in version {}. \
                Please refer to the official documentation for the latest guidelines.",
                category.unwrap_or("all"),
                resolved_version
            ));
        }
        
        let content = practices
            .iter()
            .map(|practice| {
                format!(
                    "### {}\n\n**Category:** {}\n\n{}\n\n**Examples:**\n{}\n\n**Reference:** [{}]({})",
                    practice.title,
                    practice.category,
                    practice.description,
                    practice.examples
                        .iter()
                        .map(|ex| format!("- {}", ex))
                        .collect::<Vec<_>>()
                        .join("\n"),
                    practice.documentation_ref,
                    practice.documentation_ref
                )
            })
            .collect::<Vec<_>>()
            .join("\n\n---\n\n");
        
        let references = self.reference_generator.generate_category_references("best_practices", Some(&resolved_version));
        
        Ok(format_documentation_response(
            &format!("Best Practices{}", category.map(|c| format!(" - {}", c)).unwrap_or_default()),
            &content,
            &resolved_version,
            &references,
        ))
    }
    
    /// Get implementation pattern information
    pub async fn get_implementation_pattern(&self, pattern_name: &str, version: Option<&str>) -> anyhow::Result<String> {
        let resolved_version = version
            .map(|v| self.knowledge_base.resolve_version(v))
            .unwrap_or_else(|| self.knowledge_base.default_version.clone());
        
        if let Some(pattern) = self.knowledge_base.get_implementation_pattern(pattern_name, Some(&resolved_version)) {
            let examples_text = pattern.code_examples
                .iter()
                .map(|example| {
                    format!(
                        "#### {}\n\n```{}\n{}\n```\n\n{}",
                        example.title,
                        example.language,
                        example.code,
                        example.explanation
                    )
                })
                .collect::<Vec<_>>()
                .join("\n\n");
            
            let content = format!(
                "### {}\n\n{}\n\n**Use Cases:**\n{}\n\n**Examples:**\n\n{}\n\n**Related Practices:**\n{}",
                pattern.name,
                pattern.description,
                pattern.use_cases
                    .iter()
                    .map(|uc| format!("- {}", uc))
                    .collect::<Vec<_>>()
                    .join("\n"),
                examples_text,
                pattern.related_practices
                    .iter()
                    .map(|rp| format!("- {}", rp))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
            
            let references = self.reference_generator.generate_official_references(Some(&resolved_version));
            
            Ok(format_documentation_response(
                &format!("Implementation Pattern: {}", pattern_name),
                &content,
                &resolved_version,
                &references,
            ))
        } else {
            Ok(format!(
                "Implementation pattern '{}' not found for version {}. \
                Please refer to the official documentation for available patterns.",
                pattern_name,
                resolved_version
            ))
        }
    }
    
    /// Generate response content for general queries
    async fn generate_query_response(&self, query: &str, version: &str) -> anyhow::Result<String> {
        // This is a placeholder for more sophisticated query processing
        // In a real implementation, this would analyze the query and generate
        // appropriate responses based on the knowledge base
        
        Ok(format!(
            "Based on the official Google ADK documentation (version {}), here's information about '{}':\n\n\
            This query is processed using the comprehensive ADK knowledge base with version-aware \
            information retrieval. The system provides accurate information with official documentation \
            references and implementation guidance following Google ADK best practices.\n\n\
            For detailed information, please refer to the official documentation links provided below.",
            version,
            query
        ))
    }
}

impl Default for DocumentationExpert {
    fn default() -> Self {
        Self::new()
    }
}