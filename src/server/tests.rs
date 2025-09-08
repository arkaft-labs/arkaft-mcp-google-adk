//! Unit tests for MCP server protocol compliance

#[cfg(test)]
mod tests {
    use crate::ArkaftMcpServer;
    use crate::server::ToolHandler;
    use serde_json::json;
    use tokio;

    #[tokio::test]
    async fn test_server_initialization() {
        let mut server = ArkaftMcpServer::new();
        
        // Test server creation
        assert!(!server.initialized);
        assert_eq!(server.version, env!("CARGO_PKG_VERSION"));
        
        // Test initialization
        let result = server.initialize().await;
        assert!(result.is_ok());
        assert!(server.initialized);
    }

    #[tokio::test]
    async fn test_server_capabilities() {
        let server = ArkaftMcpServer::new();
        
        // Test that server has tools capability
        assert!(server.capabilities.tools.is_some());
        
        // Test capability configuration
        let tools_cap = server.capabilities.tools.as_ref().unwrap();
        assert_eq!(tools_cap.list_changed, Some(false));
    }

    #[tokio::test]
    async fn test_tool_definitions() {
        let server = ArkaftMcpServer::new();
        
        // Test tool creation
        let tools = server.create_tool_definitions().unwrap();
        assert_eq!(tools.len(), 4);
        
        // Test tool names
        let tool_names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();
        assert!(tool_names.contains(&"adk_query"));
        assert!(tool_names.contains(&"review_rust_file"));
        assert!(tool_names.contains(&"validate_architecture"));
        assert!(tool_names.contains(&"get_best_practices"));
    }

    #[tokio::test]
    async fn test_adk_query_tool_schema() {
        let server = ArkaftMcpServer::new();
        let tools = server.create_tool_definitions().unwrap();
        
        let adk_query_tool = tools.iter().find(|t| t.name == "adk_query").unwrap();
        
        // Test tool has proper description
        assert!(adk_query_tool.description.is_some());
        assert!(adk_query_tool.description.as_ref().unwrap().contains("Google ADK"));
        
        // Test input schema has required fields
        let schema = adk_query_tool.input_schema.as_ref();
        assert!(schema.contains_key("properties"));
        assert!(schema.contains_key("required"));
    }

    #[tokio::test]
    async fn test_tool_handler_creation() {
        let server = ArkaftMcpServer::new();
        let tools = server.create_tool_definitions().unwrap();
        
        let handler = ToolHandler::new(tools.clone());
        
        // Test handler has correct number of tools
        assert_eq!(handler.get_tools().len(), 4);
    }

    #[tokio::test]
    async fn test_tool_handler_adk_query() {
        let server = ArkaftMcpServer::new();
        let tools = server.create_tool_definitions().unwrap();
        let handler = ToolHandler::new(tools);
        
        // Test adk_query tool call
        let args = json!({
            "query": "What is Google ADK?"
        });
        
        let result = handler.handle_tool_call("adk_query", args).await;
        assert!(result.is_ok());
        
        // Test result contains expected structure
        let response = result.unwrap();
        assert!(response.is_object());
    }

    #[tokio::test]
    async fn test_tool_handler_review_rust_file() {
        let server = ArkaftMcpServer::new();
        let tools = server.create_tool_definitions().unwrap();
        let handler = ToolHandler::new(tools);
        
        // Test review_rust_file tool call
        let args = json!({
            "file_path": "test.rs",
            "file_content": "fn main() { println!(\"Hello, world!\"); }"
        });
        
        let result = handler.handle_tool_call("review_rust_file", args).await;
        assert!(result.is_ok());
        
        // Test result contains expected structure
        let response = result.unwrap();
        assert!(response.is_object());
    }

    #[tokio::test]
    async fn test_tool_handler_validate_architecture() {
        let server = ArkaftMcpServer::new();
        let tools = server.create_tool_definitions().unwrap();
        let handler = ToolHandler::new(tools);
        
        // Test validate_architecture tool call
        let args = json!({
            "description": "Microservices architecture with REST APIs"
        });
        
        let result = handler.handle_tool_call("validate_architecture", args).await;
        assert!(result.is_ok());
        
        // Test result contains expected structure
        let response = result.unwrap();
        assert!(response.is_object());
    }

    #[tokio::test]
    async fn test_tool_handler_get_best_practices() {
        let server = ArkaftMcpServer::new();
        let tools = server.create_tool_definitions().unwrap();
        let handler = ToolHandler::new(tools);
        
        // Test get_best_practices tool call
        let args = json!({
            "scenario": "API design"
        });
        
        let result = handler.handle_tool_call("get_best_practices", args).await;
        assert!(result.is_ok());
        
        // Test result contains expected structure
        let response = result.unwrap();
        assert!(response.is_object());
    }

    #[tokio::test]
    async fn test_tool_handler_unknown_tool() {
        let server = ArkaftMcpServer::new();
        let tools = server.create_tool_definitions().unwrap();
        let handler = ToolHandler::new(tools);
        
        // Test unknown tool call
        let args = json!({});
        
        let result = handler.handle_tool_call("unknown_tool", args).await;
        assert!(result.is_err());
        
        // Test error message
        let error = result.unwrap_err();
        assert!(error.to_string().contains("Unknown tool"));
    }

    #[tokio::test]
    async fn test_server_startup_and_shutdown() {
        let mut server = ArkaftMcpServer::new();
        
        // Test server startup
        let start_result = server.start().await;
        assert!(start_result.is_ok());
        
        // Test server shutdown
        let shutdown_result = server.shutdown().await;
        assert!(shutdown_result.is_ok());
        assert!(!server.initialized);
    }

    #[tokio::test]
    async fn test_server_info() {
        let server = ArkaftMcpServer::new();
        let (name, version) = server.info();
        
        assert!(!name.is_empty());
        assert!(!version.is_empty());
        assert_eq!(version, env!("CARGO_PKG_VERSION"));
    }

    #[tokio::test]
    async fn test_server_config() {
        let server = ArkaftMcpServer::new();
        let config = server.config();
        
        assert!(!config.server_name.is_empty());
        assert!(!config.adk_docs_version.is_empty());
        assert!(!config.log_level.is_empty());
    }
}