// Explore rmcp API structure
use rmcp::model::*;
use rmcp::transport::*;

fn main() {
    println!("Exploring rmcp API");
    
    // Let's see what's available
    let capabilities = ServerCapabilities::default();
    println!("Default capabilities: {:?}", capabilities);
    
    // Check transport options
    let transport = stdio();
    println!("Transport created");
}