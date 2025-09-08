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
  
  // Comprehensive tests for adk_query functionality
    mod adk_query_tests {
        use crate::server::handlers::{handle_adk_query, AdkQueryParams};
        use serde_json::json;

        #[tokio::test]
        async fn test_adk_query_valid_parameters() {
            // Test with valid query parameter
            let params = json!({
                "query": "What is Google ADK?"
            });
            
            let result = handle_adk_query(params).await;
            assert!(result.is_ok());
            
            let response = result.unwrap();
            assert!(response.is_object());
            assert!(response["content"].is_array());
            assert!(response["content"][0]["type"] == "text");
            assert!(response["content"][0]["text"].is_string());
        }

        #[tokio::test]
        async fn test_adk_query_with_version() {
            // Test with query and version parameters
            let params = json!({
                "query": "ADK best practices",
                "version": "1.0.0"
            });
            
            let result = handle_adk_query(params).await;
            assert!(result.is_ok());
            
            let response = result.unwrap();
            assert!(response.is_object());
            assert!(response["content"].is_array());
            
            // Check that response contains version information
            let text = response["content"][0]["text"].as_str().unwrap();
            assert!(text.contains("1.0.0") || text.contains("version"));
        }

        #[tokio::test]
        async fn test_adk_query_empty_query() {
            // Test with empty query
            let params = json!({
                "query": ""
            });
            
            let result = handle_adk_query(params).await;
            assert!(result.is_err());
            
            let error = result.unwrap_err();
            assert!(error.to_string().contains("empty"));
        }

        #[tokio::test]
        async fn test_adk_query_whitespace_only_query() {
            // Test with whitespace-only query
            let params = json!({
                "query": "   \t\n   "
            });
            
            let result = handle_adk_query(params).await;
            assert!(result.is_err());
            
            let error = result.unwrap_err();
            assert!(error.to_string().contains("empty"));
        }

        #[tokio::test]
        async fn test_adk_query_missing_query_parameter() {
            // Test with missing query parameter
            let params = json!({
                "version": "1.0.0"
            });
            
            let result = handle_adk_query(params).await;
            assert!(result.is_err());
            
            let error = result.unwrap_err();
            assert!(error.to_string().contains("Invalid parameters"));
        }

        #[tokio::test]
        async fn test_adk_query_invalid_json() {
            // Test with invalid parameter structure
            let params = json!({
                "query": 123  // Should be string, not number
            });
            
            let result = handle_adk_query(params).await;
            assert!(result.is_err());
            
            let error = result.unwrap_err();
            assert!(error.to_string().contains("Invalid parameters"));
        }

        #[tokio::test]
        async fn test_adk_query_response_structure() {
            // Test response structure compliance
            let params = json!({
                "query": "Google ADK architecture patterns"
            });
            
            let result = handle_adk_query(params).await;
            assert!(result.is_ok());
            
            let response = result.unwrap();
            
            // Verify MCP response structure
            assert!(response.is_object());
            assert!(response.get("content").is_some());
            assert!(response["content"].is_array());
            assert!(!response["content"].as_array().unwrap().is_empty());
            
            let content_item = &response["content"][0];
            assert!(content_item.get("type").is_some());
            assert_eq!(content_item["type"], "text");
            assert!(content_item.get("text").is_some());
            assert!(content_item["text"].is_string());
        }

        #[tokio::test]
        async fn test_adk_query_official_documentation_references() {
            // Test that responses include official documentation references
            let params = json!({
                "query": "ADK quickstart guide"
            });
            
            let result = handle_adk_query(params).await;
            assert!(result.is_ok());
            
            let response = result.unwrap();
            let text = response["content"][0]["text"].as_str().unwrap();
            
            // Should contain official Google ADK documentation references
            assert!(text.contains("google.github.io/adk-docs") || 
                   text.contains("Official References") ||
                   text.contains("quickstart"));
        }

        #[tokio::test]
        async fn test_adk_query_version_specific_information() {
            // Test version-specific information retrieval
            let params_latest = json!({
                "query": "ADK features",
                "version": "latest"
            });
            
            let params_specific = json!({
                "query": "ADK features", 
                "version": "1.0.0"
            });
            
            let result_latest = handle_adk_query(params_latest).await;
            let result_specific = handle_adk_query(params_specific).await;
            
            assert!(result_latest.is_ok());
            assert!(result_specific.is_ok());
            
            // Both should succeed and contain version information
            let response_latest = result_latest.unwrap();
            let text_latest = response_latest["content"][0]["text"].as_str().unwrap();
            
            let response_specific = result_specific.unwrap();
            let text_specific = response_specific["content"][0]["text"].as_str().unwrap();
            
            assert!(text_latest.contains("latest") || text_latest.contains("version"));
            assert!(text_specific.contains("1.0.0") || text_specific.contains("version"));
        }

        #[tokio::test]
        async fn test_adk_query_parameter_validation() {
            // Test parameter validation with AdkQueryParams struct
            let valid_params = AdkQueryParams {
                query: "Valid query".to_string(),
                version: Some("1.0.0".to_string()),
            };
            
            assert!(!valid_params.query.is_empty());
            assert!(valid_params.version.is_some());
            
            let params_json = serde_json::to_value(&valid_params).unwrap();
            let result = handle_adk_query(params_json).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_adk_query_concept_search() {
            // Test searching for specific ADK concepts
            let concept_queries = vec![
                "application development kit",
                "best practices",
                "architecture patterns",
                "ADK setup"
            ];
            
            for query in concept_queries {
                let params = json!({
                    "query": query
                });
                
                let result = handle_adk_query(params).await;
                assert!(result.is_ok(), "Failed for query: {}", query);
                
                let response = result.unwrap();
                let text = response["content"][0]["text"].as_str().unwrap();
                assert!(!text.is_empty(), "Empty response for query: {}", query);
            }
        }

        #[tokio::test]
        async fn test_adk_query_implementation_guidance() {
            // Test that queries return implementation guidance
            let params = json!({
                "query": "How to implement ADK patterns?"
            });
            
            let result = handle_adk_query(params).await;
            assert!(result.is_ok());
            
            let response = result.unwrap();
            let text = response["content"][0]["text"].as_str().unwrap();
            
            // Should contain guidance-related keywords
            assert!(text.to_lowercase().contains("implementation") ||
                   text.to_lowercase().contains("guidance") ||
                   text.to_lowercase().contains("pattern") ||
                   text.to_lowercase().contains("practice"));
        }
    }