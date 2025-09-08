# Arkaft Google ADK MCP Server

A Model Context Protocol (MCP) server built in Rust that serves as an expert system for Google ADK (Application Development Kit) documentation. The server provides comprehensive knowledge of Google ADK documentation, ensures access to the latest versions, promotes best practices, and offers file translation capabilities with architectural guidance.

## Overview

This MCP acts as an intelligent assistant for developers working with Google ADK, offering:

- **Documentation Expertise**: Comprehensive knowledge of Google ADK documentation with version awareness and official references
- **Best Practices Enforcement**: Promotes and enforces official Google ADK best practices and architectural patterns
- **Rust Code Review**: Analysis of .rs files for translation needs, ADK compliance, and architectural improvements
- **Architecture Validation**: Validation of proposed implementations against established ADK guidelines

## Features

### MCP Tools

- `adk_query` - Query Google ADK documentation and concepts with current version awareness
- `review_rust_file` - Review Rust files for translation needs, ADK compliance, and architectural improvements  
- `validate_architecture` - Validate architectural patterns against official Google ADK best practices
- `get_best_practices` - Get official Google ADK best practices for specific scenarios

### Documentation References

The server provides accurate information based on official Google ADK documentation:

- [Google ADK Quickstart Guide](https://google.github.io/adk-docs/get-started/quickstart/) and related official sources
- Always references the most current version of Google ADK documentation
- Clarifies version-specific features and compatibility
- Implementation examples using current ADK version syntax and patterns

## Installation

### Prerequisites

- Rust (latest stable version)
- Cargo package manager

### Building from Source

```bash
cd arkaft-mcp-google-adk
cargo build --release
```

### Running the Server

```bash
# Run in development mode
cargo run

# Run the release build
./target/release/arkaft-mcp-google-adk
```

## Usage

### With MCP-Compatible Tools

Configure your MCP client to connect to this server. The server implements the standard MCP protocol and can be used with any compatible IDE or development tool.

### Example Tool Calls

```json
{
  "method": "tools/call",
  "params": {
    "name": "adk_query",
    "arguments": {
      "query": "How do I implement authentication in Google ADK?",
      "version": "latest"
    }
  }
}
```

## Development Status

🚧 **Currently in Development**

This project is currently being implemented based on the requirements and design specifications. The current state includes:

- ✅ **Task 1 Complete**: Project structure and core MCP server foundation
  - ✅ Standard Rust project structure with proper module organization
  - ✅ Cargo.toml with rmcp SDK (official Rust MCP implementation) and required dependencies
  - ✅ Complete module structure (server, expert, review, utils) with foundational code
  - ✅ Comprehensive error handling infrastructure with custom error types
  - ✅ Logging and tracing infrastructure configured with environment support
  - ✅ Environment variable configuration system with ServerConfig
  - ✅ MCP tool schemas defined for all four required tools
  - ✅ Handler stubs prepared for implementation
  - ✅ Documentation Expert and Code Review Engine foundations established

- ✅ **Task 2 Complete**: Core MCP server with protocol handling
  - ✅ MCP server initialization and connection management implemented
  - ✅ Basic MCP protocol message handling with proper async operations
  - ✅ Tokio runtime integration for async server operations
  - ✅ Server startup and shutdown procedures with graceful handling
  - ✅ Comprehensive unit tests for MCP protocol compliance (12 test cases)
  - ✅ Tool handler implementation with proper error handling
  - ✅ All four MCP tools (adk_query, review_rust_file, validate_architecture, get_best_practices) with complete schemas
  - ✅ Server capabilities configuration and tool registration
  - ✅ Foundation ready for MCP client connections via stdio transport

- ✅ **Task 3 Complete**: Documentation Expert System foundation
  - ✅ Comprehensive ADK knowledge base with version-aware information retrieval
  - ✅ DocumentationExpert with query processing, concept search, and best practices lookup
  - ✅ Official documentation reference generation with categorized URLs
  - ✅ Version management system with aliases and environment configuration
  - ✅ Response formatting with proper references and version information
  - ✅ Comprehensive unit tests for query functionality (12 test cases)

- ⏳ **Next**: Code review engine implementation (Task 4)
- ⏳ Best practices enforcement system (Task 5)

See [Implementation Tasks](.kiro/specs/arkaft-google-adk-mcp/tasks.md) for detailed progress tracking.

## Project Structure

```
arkaft-mcp-google-adk/
├── Cargo.toml              # Project configuration with rmcp SDK and dependencies
├── src/
│   ├── main.rs             # Entry point with async server startup and logging
│   ├── lib.rs              # Library exports and module declarations
│   ├── server/
│   │   ├── mod.rs          # ArkaftMcpServer with tool registration and initialization
│   │   └── handlers.rs     # MCP tool request handlers (stubs ready for implementation)
│   ├── expert/
│   │   ├── mod.rs          # DocumentationExpert with version-aware query foundation
│   │   ├── adk_knowledge.rs # ADK knowledge base (ready for implementation)
│   │   └── documentation.rs # Documentation utilities (ready for implementation)
│   ├── review/
│   │   ├── mod.rs          # CodeReviewEngine with analysis structures defined
│   │   ├── analyzer.rs     # Rust code analysis (ready for implementation)
│   │   └── suggestions.rs  # Improvement suggestions (ready for implementation)
│   └── utils/
│       ├── mod.rs          # Utility functions with logging and config initialization
│       └── error.rs        # Comprehensive ArkaftMcpError types and handling
├── examples/
│   └── test_rmcp.rs        # Testing utilities for rmcp exploration
├── target/                 # Build artifacts (generated)
└── README.md               # This documentation
```

## Contributing

This project follows standard Rust development practices:

1. Use `cargo fmt` for code formatting
2. Run `cargo clippy` for linting
3. Ensure `cargo test` passes for all changes
4. Follow the architectural patterns defined in the design document

## Requirements

For detailed requirements and acceptance criteria, see the [Requirements Document](.kiro/specs/arkaft-google-adk-mcp/requirements.md).

For technical design and architecture details, see the [Design Document](.kiro/specs/arkaft-google-adk-mcp/design.md).

## License

[License information to be added]

## Support

[Support information to be added]