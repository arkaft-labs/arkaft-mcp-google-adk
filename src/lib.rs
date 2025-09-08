//! Arkaft Google ADK MCP Server
//! 
//! A Model Context Protocol server that serves as an expert system for Google ADK
//! (Application Development Kit) documentation, providing comprehensive knowledge,
//! version awareness, best practices enforcement, and Rust code review capabilities.

pub mod server;
pub mod expert;
pub mod review;
pub mod utils;

pub use server::ArkaftMcpServer;