use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use std::io::{self, Write};
use tokio::sync::Mutex;
use tokio::time::timeout;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::Deserialize;
use crate::spotify::app_integration::AppState;

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

// Function to handle manual authorization code entry
async fn handle_manual_auth_code(app_state: &AppState) -> Result<(), Box<dyn std::error::Error>> {
    println!();
    println!("📝 Manual Authorization Code Entry");
    println!("After authorizing in your browser, you'll be redirected to a URL like:");
    println!("http://localhost:8080/callback?code=YOUR_CODE_HERE");
    println!();
    print!("Please paste the authorization code from the URL: ");
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let code = input.trim().to_string();
    
    if code.is_empty() {
        return Err("No authorization code provided".into());
    }
    
    println!("✅ Processing authorization code...");
    
    // Handle the authorization callback
    match app_state.handle_auth_callback(code).await {
        Ok(_) => {
            println!("🎉 Authorization successful! Tokens have been saved.");
            Ok(())
        }
        Err(e) => {
            Err(format!("Authorization failed: {:?}", e).into())
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

/// Public interface for handling Spotify OAuth authentication
pub struct AuthHandler;

impl AuthHandler {
    /// Ensures the app state has valid authentication tokens
    /// Returns Ok(()) if authentication is successful or already valid
    pub async fn ensure_authenticated(app_state: &AppState) -> Result<(), Box<dyn std::error::Error>> {
        // Check if we have valid tokens
        let needs_auth = {
            let api = app_state.spotify_api.lock().await;
            api.get_access_token().is_none() || api.is_token_expired()
        };

        if needs_auth {
            println!("No valid tokens found. Starting authorization flow...");
            
            // Get authorization URL
            let auth_url = app_state.get_auth_url().await;
            println!("🔗 Authorization URL: {}", auth_url);
            
            // Try automated flow first
            println!("🌐 Attempting automated authorization...");
            if let Err(e) = open_browser(&auth_url).await {
                println!("❌ Could not open browser automatically: {}", e);
            }
            
            println!("📋 Please open this URL in your browser:");
            println!("{}", auth_url);
            println!();
            
            // Use a specific port for the OAuth callback
            let oauth_port = 8080;
            
            println!("⏳ Waiting for authorization callback...");
            println!("💡 If the callback doesn't work, you can paste the code manually below.");
            
            // Start the OAuth server and wait for callback with shorter timeout
            match tokio::time::timeout(Duration::from_secs(60), start_oauth_server(oauth_port)).await {
                Ok(Ok(code)) => {
                    println!("✅ Received authorization code automatically, processing...");
                    
                    // Handle the authorization callback
                    match app_state.handle_auth_callback(code).await {
                        Ok(_) => {
                            println!("🎉 Authorization successful! Tokens have been saved.");
                        }
                        Err(e) => {
                            return Err(format!("Authorization failed: {:?}", e).into());
                        }
                    }
                }
                Ok(Err(e)) => {
                    eprintln!("❌ OAuth server error: {}", e);
                    println!("📝 Falling back to manual code entry...");
                    
                    handle_manual_auth_code(app_state).await?;
                }
                Err(_) => {
                    println!("⏰ Callback timeout - falling back to manual code entry...");
                    
                    handle_manual_auth_code(app_state).await?;
                }
            }
        } else {
            println!("✅ Valid tokens found, skipping authorization.");
        }

        Ok(())
    }
}
