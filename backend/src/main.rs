use std::env;

// Declare the spotify module
mod spotify;

// Import the AppState from the spotify module
use spotify::app_integration::AppState;

#[tokio::main]
async fn main() {
    // Initialize the AppState
    let app_state = AppState::new()
        .await
        .expect("Failed to initialize app state");
    
    println!("AppState initialized successfully!");
    
    let port = env::var("PORT").unwrap_or_else(|_| "3000".into());
    println!("Server would listen on port {}", port);
}
