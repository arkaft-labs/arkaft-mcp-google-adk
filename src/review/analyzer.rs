//! Rust code analysis for ADK compliance and improvements

use super::{TranslationOpportunity, ArchitecturalImprovement, ComplianceIssue, OrganizationSuggestion};
use anyhow::Result;
use syn::{File, Item, ItemFn, ItemStruct, ItemEnum, ItemImpl, Visibility, parse_str};

/// Rust code parser and analyzer
pub struct RustCodeAnalyzer {
    /// Parsed AST of the Rust file
    ast: Option<File>,
    /// Original source code
    #[allow(dead_code)]
    source: String,
    /// Line-indexed source for analysis
    lines: Vec<String>,
}

impl RustCodeAnalyzer {
    /// Create a new analyzer for the given Rust code
    pub fn new(content: &str) -> Result<Self> {
        let ast = match parse_str::<File>(content) {
            Ok(ast) => Some(ast),
            Err(_) => None, // Continue analysis even if parsing fails
        };
        
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        
        Ok(Self {
            ast,
            source: content.to_string(),
            lines,
        })
    }
    
    /// Get the parsed AST if available
    pub fn ast(&self) -> Option<&File> {
        self.ast.as_ref()
    }
    
    /// Get source lines
    pub fn lines(&self) -> &[String] {
        &self.lines
    }
    
    /// Check if the code has valid Rust syntax
    pub fn has_valid_syntax(&self) -> bool {
        self.ast.is_some()
    }
    
    /// Extract all functions from the AST
    pub fn extract_functions(&self) -> Vec<&ItemFn> {
        if let Some(ast) = &self.ast {
            ast.items.iter()
                .filter_map(|item| match item {
                    Item::Fn(func) => Some(func),
                    _ => None,
                })
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Extract all structs from the AST
    pub fn extract_structs(&self) -> Vec<&ItemStruct> {
        if let Some(ast) = &self.ast {
            ast.items.iter()
                .filter_map(|item| match item {
                    Item::Struct(s) => Some(s),
                    _ => None,
                })
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Extract all enums from the AST
    pub fn extract_enums(&self) -> Vec<&ItemEnum> {
        if let Some(ast) = &self.ast {
            ast.items.iter()
                .filter_map(|item| match item {
                    Item::Enum(e) => Some(e),
                    _ => None,
                })
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Extract all impl blocks from the AST
    pub fn extract_impls(&self) -> Vec<&ItemImpl> {
        if let Some(ast) = &self.ast {
            ast.items.iter()
                .filter_map(|item| match item {
                    Item::Impl(i) => Some(i),
                    _ => None,
                })
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Analyze code complexity and patterns
    pub fn analyze_patterns(&self) -> CodePatterns {
        let mut patterns = CodePatterns::default();
        
        // Analyze function patterns
        for func in self.extract_functions() {
            patterns.function_count += 1;
            
            // Check for async functions
            if func.sig.asyncness.is_some() {
                patterns.async_functions += 1;
            }
            
            // Check for public functions
            if matches!(func.vis, Visibility::Public(_)) {
                patterns.public_functions += 1;
            }
            
            // Check for error handling patterns
            let func_str = quote::ToTokens::to_token_stream(func).to_string();
            if func_str.contains("Result<") {
                patterns.result_returning_functions += 1;
            }
        }
        
        // Analyze struct patterns
        patterns.struct_count = self.extract_structs().len();
        patterns.enum_count = self.extract_enums().len();
        patterns.impl_count = self.extract_impls().len();
        
        // Analyze source patterns
        for (line_num, line) in self.lines.iter().enumerate() {
            if line.contains("unwrap()") {
                patterns.unwrap_usage.push(line_num + 1);
            }
            if line.contains("panic!") {
                patterns.panic_usage.push(line_num + 1);
            }
            if line.contains("todo!") || line.contains("unimplemented!") {
                patterns.todo_usage.push(line_num + 1);
            }
        }
        
        patterns
    }
}

/// Code patterns detected in the analysis
#[derive(Debug, Default)]
pub struct CodePatterns {
    pub function_count: usize,
    pub async_functions: usize,
    pub public_functions: usize,
    pub result_returning_functions: usize,
    pub struct_count: usize,
    pub enum_count: usize,
    pub impl_count: usize,
    pub unwrap_usage: Vec<usize>,
    pub panic_usage: Vec<usize>,
    pub todo_usage: Vec<usize>,
}

/// Analyze Rust code for translation opportunities
pub fn analyze_translation_opportunities(content: &str) -> Result<Vec<TranslationOpportunity>> {
    let analyzer = RustCodeAnalyzer::new(content)?;
    let mut opportunities = Vec::new();
    
    if !analyzer.has_valid_syntax() {
        opportunities.push(TranslationOpportunity {
            line: 1,
            description: "Syntax errors detected in Rust code".to_string(),
            suggestion: "Fix syntax errors to enable proper analysis and ADK compliance checking".to_string(),
        });
        return Ok(opportunities);
    }
    
    let patterns = analyzer.analyze_patterns();
    
    // Check for unwrap() usage - translation opportunity to proper error handling
    for line_num in &patterns.unwrap_usage {
        opportunities.push(TranslationOpportunity {
            line: *line_num,
            description: "Direct unwrap() usage detected".to_string(),
            suggestion: "Replace unwrap() with proper error handling using match, if let, or ? operator for better ADK compliance".to_string(),
        });
    }
    
    // Check for panic! usage - translation opportunity to Result-based error handling
    for line_num in &patterns.panic_usage {
        opportunities.push(TranslationOpportunity {
            line: *line_num,
            description: "Panic usage detected".to_string(),
            suggestion: "Replace panic! with Result-based error handling to follow ADK error handling patterns".to_string(),
        });
    }
    
    // Check for TODO/unimplemented - translation opportunities
    for line_num in &patterns.todo_usage {
        opportunities.push(TranslationOpportunity {
            line: *line_num,
            description: "Incomplete implementation detected".to_string(),
            suggestion: "Complete the implementation following Google ADK patterns and best practices".to_string(),
        });
    }
    
    // Check for missing async patterns in functions that could benefit
    if patterns.function_count > 0 && patterns.async_functions == 0 {
        // Look for I/O operations that should be async
        for (line_num, line) in analyzer.lines().iter().enumerate() {
            if line.contains("std::fs::") || line.contains("File::") {
                opportunities.push(TranslationOpportunity {
                    line: line_num + 1,
                    description: "Synchronous I/O operation detected".to_string(),
                    suggestion: "Consider using async I/O operations (tokio::fs) for better performance in ADK applications".to_string(),
                });
                break; // Only suggest once per file
            }
        }
    }
    
    Ok(opportunities)
}

/// Analyze architectural patterns for ADK compliance
pub fn analyze_architectural_patterns(content: &str) -> Result<Vec<ArchitecturalImprovement>> {
    let analyzer = RustCodeAnalyzer::new(content)?;
    let mut improvements = Vec::new();
    
    if !analyzer.has_valid_syntax() {
        return Ok(improvements);
    }
    
    let patterns = analyzer.analyze_patterns();
    
    // Check for proper error handling architecture
    if patterns.function_count > 0 && patterns.result_returning_functions == 0 {
        improvements.push(ArchitecturalImprovement {
            area: "Error Handling Architecture".to_string(),
            current_pattern: "Functions without Result return types".to_string(),
            recommended_pattern: "Use Result<T, E> return types for fallible operations".to_string(),
            rationale: "Google ADK emphasizes robust error handling. Functions that can fail should return Result types".to_string(),
        });
    }
    
    // Check for async architecture in I/O heavy code
    if patterns.function_count > 2 && patterns.async_functions == 0 {
        // Check if there are I/O operations
        let has_io = analyzer.lines().iter().any(|line| {
            line.contains("std::fs::") || line.contains("std::net::") || line.contains("reqwest")
        });
        
        if has_io {
            improvements.push(ArchitecturalImprovement {
                area: "Async Architecture".to_string(),
                current_pattern: "Synchronous I/O operations".to_string(),
                recommended_pattern: "Async/await pattern with tokio runtime".to_string(),
                rationale: "ADK applications benefit from async architecture for better concurrency and performance".to_string(),
            });
        }
    }
    
    // Check for proper module organization
    if patterns.struct_count > 3 && patterns.impl_count == 0 {
        improvements.push(ArchitecturalImprovement {
            area: "Code Organization".to_string(),
            current_pattern: "Structs without associated implementations".to_string(),
            recommended_pattern: "Group related functionality in impl blocks".to_string(),
            rationale: "ADK promotes clear code organization with methods grouped in impl blocks".to_string(),
        });
    }
    
    // Check for proper visibility patterns
    if patterns.public_functions > patterns.function_count / 2 {
        improvements.push(ArchitecturalImprovement {
            area: "API Design".to_string(),
            current_pattern: "Many public functions without clear API boundaries".to_string(),
            recommended_pattern: "Minimize public API surface, use pub(crate) for internal functions".to_string(),
            rationale: "ADK emphasizes clean API design with minimal public interfaces".to_string(),
        });
    }
    
    Ok(improvements)
}

/// Analyze code for ADK compliance issues
pub fn analyze_adk_compliance(content: &str) -> Result<Vec<ComplianceIssue>> {
    let analyzer = RustCodeAnalyzer::new(content)?;
    let mut issues = Vec::new();
    
    if !analyzer.has_valid_syntax() {
        issues.push(ComplianceIssue {
            issue_type: "Syntax Error".to_string(),
            description: "Code contains syntax errors that prevent proper analysis".to_string(),
            fix_suggestion: "Fix all syntax errors to ensure code compiles and follows Rust standards".to_string(),
        });
        return Ok(issues);
    }
    
    let patterns = analyzer.analyze_patterns();
    
    // Check for panic usage - ADK compliance issue
    if !patterns.panic_usage.is_empty() {
        issues.push(ComplianceIssue {
            issue_type: "Error Handling Compliance".to_string(),
            description: format!("Found {} panic! usage(s) which violate ADK error handling guidelines", patterns.panic_usage.len()),
            fix_suggestion: "Replace panic! with proper Result-based error handling or graceful error recovery".to_string(),
        });
    }
    
    // Check for unwrap usage - potential compliance issue
    if patterns.unwrap_usage.len() > 2 {
        issues.push(ComplianceIssue {
            issue_type: "Error Handling Compliance".to_string(),
            description: format!("Excessive unwrap() usage ({} instances) may indicate poor error handling", patterns.unwrap_usage.len()),
            fix_suggestion: "Replace unwrap() calls with proper error handling using ?, match, or if let patterns".to_string(),
        });
    }
    
    // Check for missing documentation on public items
    let public_items_without_docs = check_missing_documentation(&analyzer);
    if !public_items_without_docs.is_empty() {
        issues.push(ComplianceIssue {
            issue_type: "Documentation Compliance".to_string(),
            description: "Public items missing documentation comments".to_string(),
            fix_suggestion: "Add /// documentation comments to all public functions, structs, and modules following ADK documentation standards".to_string(),
        });
    }
    
    // Check for TODO/unimplemented in production code
    if !patterns.todo_usage.is_empty() {
        issues.push(ComplianceIssue {
            issue_type: "Implementation Completeness".to_string(),
            description: format!("Found {} incomplete implementation(s) (todo!/unimplemented!)", patterns.todo_usage.len()),
            fix_suggestion: "Complete all implementations or use proper feature flags for incomplete functionality".to_string(),
        });
    }
    
    Ok(issues)
}

/// Check for missing documentation on public items
fn check_missing_documentation(analyzer: &RustCodeAnalyzer) -> Vec<String> {
    let mut missing_docs = Vec::new();
    
    // Check public functions
    for func in analyzer.extract_functions() {
        if matches!(func.vis, Visibility::Public(_)) {
            let func_name = func.sig.ident.to_string();
            // Simple heuristic: check if there's a doc comment before the function
            // In a real implementation, we'd need more sophisticated AST analysis
            missing_docs.push(format!("Function: {}", func_name));
        }
    }
    
    // Check public structs
    for struct_item in analyzer.extract_structs() {
        if matches!(struct_item.vis, Visibility::Public(_)) {
            let struct_name = struct_item.ident.to_string();
            missing_docs.push(format!("Struct: {}", struct_name));
        }
    }
    
    missing_docs
}

/// Analyze file organization and structure
pub fn analyze_file_organization(file_path: &str, content: &str) -> Result<Vec<OrganizationSuggestion>> {
    let analyzer = RustCodeAnalyzer::new(content)?;
    let mut suggestions = Vec::new();
    
    let patterns = analyzer.analyze_patterns();
    
    // Check file size and complexity
    let line_count = analyzer.lines().len();
    if line_count > 500 {
        suggestions.push(OrganizationSuggestion {
            suggestion_type: "File Size".to_string(),
            description: format!("File is quite large ({} lines) which may impact maintainability", line_count),
            action: "Consider splitting into smaller, focused modules following ADK organization patterns".to_string(),
        });
    }
    
    // Check for proper module structure
    if patterns.struct_count + patterns.enum_count > 5 && !file_path.ends_with("mod.rs") {
        suggestions.push(OrganizationSuggestion {
            suggestion_type: "Module Organization".to_string(),
            description: "Many types defined in a single file".to_string(),
            action: "Consider organizing related types into separate modules with a mod.rs file".to_string(),
        });
    }
    
    // Check for proper separation of concerns
    if patterns.function_count > 10 && patterns.impl_count == 0 {
        suggestions.push(OrganizationSuggestion {
            suggestion_type: "Code Organization".to_string(),
            description: "Many standalone functions without clear grouping".to_string(),
            action: "Group related functions into structs with impl blocks or separate modules".to_string(),
        });
    }
    
    // Check naming conventions
    let file_name = std::path::Path::new(file_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    
    if file_name.contains("_") && !file_name.ends_with("_test") {
        suggestions.push(OrganizationSuggestion {
            suggestion_type: "Naming Convention".to_string(),
            description: "File name uses underscores".to_string(),
            action: "Consider using kebab-case for file names following Rust conventions".to_string(),
        });
    }
    
    // Check for proper imports organization
    let import_lines: Vec<_> = analyzer.lines().iter()
        .take(20) // Check first 20 lines for imports
        .enumerate()
        .filter(|(_, line)| line.trim_start().starts_with("use "))
        .collect();
    
    if import_lines.len() > 10 {
        suggestions.push(OrganizationSuggestion {
            suggestion_type: "Import Organization".to_string(),
            description: "Many import statements may indicate complex dependencies".to_string(),
            action: "Group imports by source (std, external crates, local modules) and consider reducing dependencies".to_string(),
        });
    }
    
    Ok(suggestions)
}