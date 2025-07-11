use std::env;

// Declare modules
mod spotify;
mod auth;

use spotify::app_integration::AppState;
use auth::AuthHandler;

#[tokio::main]
async fn main() {
    // Initialize environment variables from .env file
    dotenv::dotenv().ok();
    env_logger::init();

    // Initialize the AppState
    match AppState::new().await {
        Ok(app_state) => {
            println!("AppState initialized successfully!");

            // Handle authentication
            if let Err(e) = AuthHandler::ensure_authenticated(&app_state).await {
                eprintln!("❌ Authentication failed: {:?}", e);
                return;
            }

            // Now try to get playlist tracks
            let port = env::var("PORT").unwrap_or_else(|_| "3000".into());
            println!("🚀 Server would listen on port {}", port);
            println!("📀 Fetching playlist tracks...");
            match app_state.get_playlist_tracks("441K4rF3u0qfg9m4X1WSQJ").await {
                Ok(tracks) => {
                    println!("✅ Successfully fetched {} tracks", tracks.items.len());
                    for (i, item) in tracks.items.iter().take(5).enumerate() {
                        println!("{}. {} - {}", 
                            i + 1, 
                            item.track.name, 
                            item.track.artists.iter().map(|a| a.name.as_str()).collect::<Vec<_>>().join(", ")
                        );
                    }
                }
                Err(e) => {
                    eprintln!("❌ Failed to fetch playlist tracks: {:?}", e);
                }
            }

            // TODO: Start your actual server here
            // For example:
            // start_web_server(&app_state, port).await;
        }
        Err(e) => {
            eprintln!("❌ Failed to initialize app state: {:?}", e);
        }
    }
}