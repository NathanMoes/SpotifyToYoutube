use std::env;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::timeout;

// Declare the spotify module
mod spotify;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::Deserialize;

// Import the AppState from the spotify module
use spotify::app_integration::AppState;

#[derive(Deserialize)]
struct AuthParams {
    code: String,
}

// Shared state for handling OAuth callback
struct OAuthState {
    code: Arc<Mutex<Option<String>>>,
    completed: Arc<AtomicBool>,
}

impl OAuthState {
    fn new() -> Self {
        Self {
            code: Arc::new(Mutex::new(None)),
            completed: Arc::new(AtomicBool::new(false)),
        }
    }
}

// Handler for processing the callback with the authorization code
async fn handle_auth_callback(
    oauth_state: web::Data<OAuthState>,
    params: web::Query<AuthParams>,
) -> impl Responder {
    println!("Received authorization code: {}", params.code);
    
    // Store the code
    let mut code_guard = oauth_state.code.lock().await;
    *code_guard = Some(params.code.clone());
    
    // Mark as completed
    oauth_state.completed.store(true, Ordering::Relaxed);
    
    HttpResponse::Ok().body("✅ Authorization successful! You can close this window and return to the application.")
}

// Function to start temporary OAuth server
async fn start_oauth_server(port: u16) -> Result<String, Box<dyn std::error::Error>> {
    let oauth_state = web::Data::new(OAuthState::new());
    let oauth_state_clone = oauth_state.clone();
    
    println!("Starting temporary OAuth server on port {}", port);
    
    // Start the server in a background task
    let server_handle = tokio::spawn(async move {
        HttpServer::new(move || {
            App::new()
                .app_data(oauth_state.clone())
                .route("/callback", web::get().to(handle_auth_callback))
        })
        .bind(format!("127.0.0.1:{}", port))
        .unwrap()
        .run()
        .await
    });
    
    // Wait for the authorization code with timeout
    let timeout_duration = Duration::from_secs(300); // 5 minutes timeout
    let result = timeout(timeout_duration, async {
        loop {
            if oauth_state_clone.completed.load(Ordering::Relaxed) {
                let code_guard = oauth_state_clone.code.lock().await;
                if let Some(code) = code_guard.clone() {
                    return Ok(code);
                }
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }).await;
    
    // Stop the server
    server_handle.abort();
    
    match result {
        Ok(code_result) => code_result,
        Err(_) => Err("OAuth timeout - no authorization code received within 5 minutes".into()),
    }
}



#[tokio::main]
async fn main() {
    // Initialize environment variables from .env file
    dotenv::dotenv().ok();
    env_logger::init();

    // Initialize the AppState
    match AppState::new().await {
        Ok(app_state) => {
            println!("AppState initialized successfully!");

            // Check if we have valid tokens
            let needs_auth = {
                let api = app_state.spotify_api.lock().await;
                api.get_access_token().is_none() || api.is_token_expired()
            };

            if needs_auth {
                println!("No valid tokens found. Starting automated authorization flow...");
                
                // Use a specific port for the OAuth callback
                let oauth_port = 8080;
                
                // Get authorization URL
                let auth_url = app_state.get_auth_url().await;
                println!("🔗 Authorization URL: {}", auth_url);
                
                // Try to automatically open the browser
                println!("🌐 Attempting to open browser automatically...");
                if let Err(e) = open_browser(&auth_url).await {
                    println!("❌ Could not open browser automatically: {}", e);
                    println!("📋 Please manually open this URL in your browser:");
                    println!("{}", auth_url);
                }
                
                println!("⏳ Waiting for authorization callback...");
                
                // Start the OAuth server and wait for callback
                match start_oauth_server(oauth_port).await {
                    Ok(code) => {
                        println!("✅ Received authorization code, processing...");
                        
                        // Handle the authorization callback
                        match app_state.handle_auth_callback(code).await {
                            Ok(_) => {
                                println!("🎉 Authorization successful! Tokens have been saved.");
                            }
                            Err(e) => {
                                eprintln!("❌ Authorization failed: {:?}", e);
                                return;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ OAuth server error: {}", e);
                        return;
                    }
                }
            } else {
                println!("✅ Valid tokens found, skipping authorization.");
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
        }
        Err(e) => {
            eprintln!("❌ Failed to initialize app state: {:?}", e);
        }
    }
}

// Function to automatically open the browser
async fn open_browser(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(&["/C", "start", url])
            .output()?;
    }
    
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(url)
            .output()?;
    }
    
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(url)
            .output()?;
    }
    
    Ok(())
}