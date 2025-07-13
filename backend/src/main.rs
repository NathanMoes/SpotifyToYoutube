use std::env;
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post, put},
    Router,
};
use serde_json::{json, Value};
use tower_http::cors::CorsLayer;

// Declare modules
mod spotify;
mod auth;
mod youtube;
mod database;
mod music_service;
mod conversion_service;

use spotify::app_integration::AppState;
use auth::AuthHandler;

// Health check handler
async fn health_check() -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json(json!({"status": "healthy", "service": "spotify-youtube-backend"})))
}

// API status handler
async fn api_status(State(_app_state): State<AppState>) -> (StatusCode, Json<Value>) {
    // You can add more detailed status checks here
    (StatusCode::OK, Json(json!({
        "status": "ok",
        "services": {
            "spotify": "connected",
            "youtube": "connected",
            "database": "connected"
        }
    })))
}

// Store playlist in database
async fn store_playlist(
    State(app_state): State<AppState>,
    axum::extract::Path(playlist_id): axum::extract::Path<String>,
) -> (StatusCode, Json<Value>) {
    match app_state.store_playlist_in_database(&playlist_id).await {
        Ok(()) => (
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "message": format!("Playlist {} stored in database", playlist_id)
            }))
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": format!("Failed to store playlist: {}", e)
            }))
        ),
    }
}

// Get tracks that need YouTube conversion
async fn get_tracks_for_conversion(
    State(app_state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> (StatusCode, Json<Value>) {
    let limit = params.get("limit")
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(10);

    match app_state.get_tracks_for_conversion(limit).await {
        Ok(tracks) => (
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "tracks": tracks,
                "count": tracks.len()
            }))
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": format!("Failed to get tracks: {}", e)
            }))
        ),
    }
}

// Update track with YouTube URL
async fn update_track_youtube_url(
    State(app_state): State<AppState>,
    axum::extract::Path(track_id): axum::extract::Path<String>,
    Json(payload): Json<Value>,
) -> (StatusCode, Json<Value>) {
    if let Some(youtube_url) = payload.get("youtube_url").and_then(|v| v.as_str()) {
        match app_state.update_track_youtube_url(&track_id, youtube_url).await {
            Ok(()) => (
                StatusCode::OK,
                Json(json!({
                    "status": "success",
                    "message": format!("Track {} updated with YouTube URL", track_id)
                }))
            ),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("Failed to update track: {}", e)
                }))
            ),
        }
    } else {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "status": "error",
                "message": "Missing youtube_url in request body"
            }))
        )
    }
}

// Get playlist tracks with YouTube URLs
async fn get_playlist_tracks_with_youtube(
    State(app_state): State<AppState>,
    axum::extract::Path(playlist_id): axum::extract::Path<String>,
) -> (StatusCode, Json<Value>) {
    match app_state.get_playlist_tracks_with_youtube(&playlist_id).await {
        Ok(tracks) => (
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "playlist_id": playlist_id,
                "tracks": tracks,
                "count": tracks.len()
            }))
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": format!("Failed to get playlist tracks: {}", e)
            }))
        ),
    }
}

// Search tracks in database
async fn search_tracks(
    State(app_state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> (StatusCode, Json<Value>) {
    if let Some(query) = params.get("q") {
        let limit = params.get("limit")
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap_or(10);

        match app_state.search_tracks_in_database(query, limit).await {
            Ok(tracks) => (
                StatusCode::OK,
                Json(json!({
                    "status": "success",
                    "query": query,
                    "tracks": tracks,
                    "count": tracks.len()
                }))
            ),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("Failed to search tracks: {}", e)
                }))
            ),
        }
    } else {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "status": "error",
                "message": "Missing 'q' query parameter"
            }))
        )
    }
}

// Get conversion statistics
async fn get_conversion_stats(State(app_state): State<AppState>) -> (StatusCode, Json<Value>) {
    match app_state.conversion_service.get_conversion_stats().await {
        Ok(stats) => (
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "statistics": {
                    "total_tracks": stats.total_tracks,
                    "converted_tracks": stats.converted_tracks,
                    "pending_conversion": stats.pending_conversion
                }
            }))
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": format!("Failed to get statistics: {}", e)
            }))
        ),
    }
}

// Manually trigger conversion for a specific track
async fn convert_track_manually(
    State(app_state): State<AppState>,
    axum::extract::Path(track_id): axum::extract::Path<String>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> (StatusCode, Json<Value>) {
    let force = params.get("force").map(|v| v == "true").unwrap_or(false);
    
    match app_state.conversion_service.convert_track_manually(&track_id, force).await {
        Ok(youtube_url) => (
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "track_id": track_id,
                "youtube_url": youtube_url,
                "message": "Track converted successfully"
            }))
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": format!("Failed to convert track: {}", e)
            }))
        ),
    }
}

async fn start_web_server(app_state: AppState, port: String) -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/status", get(api_status))
        .route("/api/playlists/:playlist_id/store", post(store_playlist))
        .route("/api/tracks/for-conversion", get(get_tracks_for_conversion))
        .route("/api/tracks/:track_id/youtube-url", put(update_track_youtube_url))
        .route("/api/playlists/:playlist_id/tracks", get(get_playlist_tracks_with_youtube))
        .route("/api/tracks/search", get(search_tracks))
        .route("/api/conversion/stats", get(get_conversion_stats))
        .route("/api/tracks/:track_id/convert", post(convert_track_manually))
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    let addr = format!("0.0.0.0:{}", port);
    println!("🚀 Server listening on {}", addr);
    println!("📖 API Documentation:");
    println!("   GET  /health - Health check");
    println!("   GET  /api/status - Service status");
    println!("   POST /api/playlists/:id/store - Store playlist in database");
    println!("   GET  /api/tracks/for-conversion - Get tracks needing conversion");
    println!("   PUT  /api/tracks/:id/youtube-url - Update track YouTube URL");
    println!("   GET  /api/playlists/:id/tracks - Get playlist tracks with YouTube URLs");
    println!("   GET  /api/tracks/search?q=query - Search tracks");
    println!("   GET  /api/conversion/stats - Get conversion statistics");
    println!("   POST /api/tracks/:id/convert - Manually convert track");
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

#[tokio::main]
async fn main() {
    // Initialize environment variables from .env file
    dotenv::dotenv().ok();
    env_logger::init();


    let _youtube_state = youtube::app_integration::YouTubeAppState::new().await;

    // Initialize the AppState
    match AppState::new().await {
        Ok(app_state) => {
            println!("AppState initialized successfully!");

            // Handle authentication
            if let Err(e) = AuthHandler::ensure_authenticated(&app_state).await {
                eprintln!("❌ Authentication failed: {:?}", e);
                return;
            }

            // Now try to get playlist tracks and store in database
            let port = env::var("PORT").unwrap_or_else(|_| "3000".into());
            println!("🚀 Starting server on port {}", port);
            println!("📀 Fetching playlist tracks...");
            
            let playlist_id = "441K4rF3u0qfg9m4X1WSQJ";
            match app_state.get_playlist_tracks(playlist_id).await {
                Ok(tracks) => {
                    println!("✅ Successfully fetched {} tracks", tracks.items.len());
                    for (i, item) in tracks.items.iter().take(5).enumerate() {
                        println!("{}. {} - {}", 
                            i + 1, 
                            item.track.name, 
                            item.track.artists.iter().map(|a| a.name.as_str()).collect::<Vec<_>>().join(", ")
                        );
                    }

                    // Store playlist in database
                    println!("💾 Storing playlist in database...");
                    match app_state.store_playlist_in_database(playlist_id).await {
                        Ok(()) => {
                            println!("✅ Playlist stored in database successfully!");
                            
                            // Show some tracks that need YouTube conversion
                            match app_state.get_tracks_for_conversion(5).await {
                                Ok(db_tracks) => {
                                    if !db_tracks.is_empty() {
                                        println!("🔍 Tracks needing YouTube conversion:");
                                        for track in db_tracks.iter().take(3) {
                                            println!("  - {} (ID: {})", track.name, track.id);
                                        }
                                    } else {
                                        println!("✅ All tracks already have YouTube URLs!");
                                    }
                                },
                                Err(e) => println!("⚠️ Could not check conversion status: {}", e),
                            }
                        },
                        Err(e) => {
                            eprintln!("❌ Failed to store playlist in database: {:?}", e);
                            eprintln!("Make sure Neo4j is running and the connection details are correct");
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ Failed to fetch playlist tracks: {:?}", e);
                }
            }

            // Start the web server
            if let Err(e) = start_web_server(app_state, port).await {
                eprintln!("❌ Failed to start web server: {:?}", e);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to initialize app state: {:?}", e);
        }
    }
}