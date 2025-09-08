//! Documentation utilities and reference generation

/// Generate official documentation references
pub fn generate_official_references(_version: &str) -> Vec<String> {
    vec![
        "https://google.github.io/adk-docs/get-started/quickstart/".to_string(),
        // Additional version-specific URLs can be added here
    ]
}

/// Format documentation response with proper references
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
            .map(|url| format!("- [{}]({})", url, url))
            .collect::<Vec<_>>()
            .join("\n"),
        version
    )
}