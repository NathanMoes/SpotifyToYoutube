use std::env;
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tower_http::cors::CorsLayer;
use tracing::{info, error, warn, debug, instrument};

// Declare modules
mod spotify;
mod auth;
mod youtube;
mod database;
mod music_service;
mod conversion_service;
mod logging;

use spotify::app_integration::AppState;
use auth::AuthHandler;

// Request/Response structs
#[derive(Deserialize, Debug)]
struct ImportPlaylistRequest {
    url: String,
}

#[derive(Deserialize, Debug)]
struct AddTrackRequest {
    track_name: String,
    artist_name: String,
}

#[derive(Serialize)]
struct ImportPlaylistResponse {
    status: String,
    message: String,
    playlist_id: Option<String>,
    tracks_count: Option<usize>,
}

// Health check handler
#[instrument]
async fn health_check() -> (StatusCode, Json<Value>) {
    debug!("Health check requested");
    (StatusCode::OK, Json(json!({"status": "healthy", "service": "spotify-youtube-backend"})))
}

// API status handler
#[instrument(skip(_app_state))]
async fn api_status(State(_app_state): State<AppState>) -> (StatusCode, Json<Value>) {
    debug!("API status check requested");
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
#[instrument(skip(app_state), fields(playlist_id = %playlist_id))]
async fn store_playlist(
    State(app_state): State<AppState>,
    axum::extract::Path(playlist_id): axum::extract::Path<String>,
) -> (StatusCode, Json<Value>) {
    info!("Storing playlist in database");
    
    match app_state.store_playlist_in_database(&playlist_id).await {
        Ok(()) => {
            info!("Successfully stored playlist in database");
            (
                StatusCode::OK,
                Json(json!({
                    "status": "success",
                    "message": format!("Playlist {} stored in database", playlist_id)
                }))
            )
        },
        Err(e) => {
            error!(error = %e, "Failed to store playlist in database");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("Failed to store playlist: {}", e)
                }))
            )
        },
    }
}

// Get tracks that need YouTube conversion
#[instrument(skip(app_state))]
async fn get_tracks_for_conversion(
    State(app_state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> (StatusCode, Json<Value>) {
    let limit = params.get("limit")
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(10);

    info!("Getting tracks for conversion");

    match app_state.get_tracks_for_conversion(limit).await {
        Ok(tracks) => {
            info!(count = tracks.len(), "Successfully retrieved tracks for conversion");
            (
                StatusCode::OK,
                Json(json!({
                    "status": "success",
                    "tracks": tracks,
                    "count": tracks.len()
                }))
            )
        },
        Err(e) => {
            error!(error = %e, "Failed to get tracks for conversion");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("Failed to get tracks: {}", e)
                }))
            )
        },
    }
}

// Update track with YouTube URL
#[instrument(skip(app_state), fields(track_id = %track_id))]
async fn update_track_youtube_url(
    State(app_state): State<AppState>,
    axum::extract::Path(track_id): axum::extract::Path<String>,
    Json(payload): Json<Value>,
) -> (StatusCode, Json<Value>) {
    if let Some(youtube_url) = payload.get("youtube_url").and_then(|v| v.as_str()) {
        info!(youtube_url = %youtube_url, "Updating track with YouTube URL");
        
        match app_state.update_track_youtube_url(&track_id, youtube_url).await {
            Ok(()) => {
                info!("Successfully updated track with YouTube URL");
                (
                    StatusCode::OK,
                    Json(json!({
                        "status": "success",
                        "message": format!("Track {} updated with YouTube URL", track_id)
                    }))
                )
            },
            Err(e) => {
                error!(error = %e, "Failed to update track with YouTube URL");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "status": "error",
                        "message": format!("Failed to update track: {}", e)
                    }))
                )
            },
        }
    } else {
        warn!("Missing youtube_url in request body");
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
#[instrument(skip(app_state), fields(playlist_id = %playlist_id))]
async fn get_playlist_tracks_with_youtube(
    State(app_state): State<AppState>,
    axum::extract::Path(playlist_id): axum::extract::Path<String>,
) -> (StatusCode, Json<Value>) {
    info!("Getting playlist tracks with YouTube URLs");
    
    match app_state.get_playlist_tracks_with_youtube(&playlist_id).await {
        Ok(tracks) => {
            info!(count = tracks.len(), "Successfully retrieved playlist tracks with YouTube URLs");
            (
                StatusCode::OK,
                Json(json!({
                    "status": "success",
                    "playlist_id": playlist_id,
                    "tracks": tracks,
                    "count": tracks.len()
                }))
            )
        },
        Err(e) => {
            error!(error = %e, "Failed to get playlist tracks with YouTube URLs");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("Failed to get playlist tracks: {}", e)
                }))
            )
        },
    }
}

// Search tracks in database
#[instrument(skip(app_state))]
async fn search_tracks(
    State(app_state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> (StatusCode, Json<Value>) {
    if let Some(query) = params.get("q") {
        let limit = params.get("limit")
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap_or(10);

        info!(query = %query, limit = %limit, "Searching tracks in database");

        match app_state.search_tracks_in_database(query, limit).await {
            Ok(tracks) => {
                info!(count = tracks.len(), "Successfully found tracks");
                (
                    StatusCode::OK,
                    Json(json!({
                        "status": "success",
                        "query": query,
                        "tracks": tracks,
                        "count": tracks.len()
                    }))
                )
            },
            Err(e) => {
                error!(error = %e, "Failed to search tracks in database");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "status": "error",
                        "message": format!("Failed to search tracks: {}", e)
                    }))
                )
            },
        }
    } else {
        warn!("Missing 'q' query parameter in search request");
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
#[instrument(skip(app_state))]
async fn get_conversion_stats(State(app_state): State<AppState>) -> (StatusCode, Json<Value>) {
    info!("Getting conversion statistics");
    
    match app_state.conversion_service.get_conversion_stats().await {
        Ok(stats) => {
            info!(
                total_tracks = stats.total_tracks,
                converted_tracks = stats.converted_tracks,
                pending_conversion = stats.pending_conversion,
                "Successfully retrieved conversion statistics"
            );
            (
                StatusCode::OK,
                Json(json!({
                    "status": "success",
                    "statistics": {
                        "total_tracks": stats.total_tracks,
                        "converted_tracks": stats.converted_tracks,
                        "pending_conversion": stats.pending_conversion
                    }
                }))
            )
        },
        Err(e) => {
            error!(error = %e, "Failed to get conversion statistics");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("Failed to get statistics: {}", e)
                }))
            )
        },
    }
}

// Manually trigger conversion for a specific track
#[instrument(skip(app_state), fields(track_id = %track_id))]
async fn convert_track_manually(
    State(app_state): State<AppState>,
    axum::extract::Path(track_id): axum::extract::Path<String>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> (StatusCode, Json<Value>) {
    let force = params.get("force").map(|v| v == "true").unwrap_or(false);
    
    info!("Manually converting track");
    
    match app_state.conversion_service.convert_track_manually(&track_id, force).await {
        Ok(youtube_url) => {
            info!(youtube_url = %youtube_url, "Successfully converted track manually");
            (
                StatusCode::OK,
                Json(json!({
                    "status": "success",
                    "track_id": track_id,
                    "youtube_url": youtube_url,
                    "message": "Track converted successfully"
                }))
            )
        },
        Err(e) => {
            error!(error = %e, "Failed to manually convert track");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("Failed to convert track: {}", e)
                }))
            )
        },
    }
}

// Import playlist by URL
#[instrument(skip(app_state))]
async fn import_playlist(
    State(app_state): State<AppState>,
    Json(payload): Json<ImportPlaylistRequest>,
) -> (StatusCode, Json<ImportPlaylistResponse>) {
    let ImportPlaylistRequest { url } = payload;

    info!("Importing playlist by URL");

    match app_state.import_playlist_by_url(&url).await {
        Ok((playlist_id, tracks_count)) => {
            info!(
                playlist_id = %playlist_id,
                tracks_count = tracks_count,
                "Successfully imported playlist"
            );
            (
                StatusCode::OK,
                Json(ImportPlaylistResponse {
                    status: "success".into(),
                    message: "Playlist imported successfully".into(),
                    playlist_id: Some(playlist_id),
                    tracks_count: Some(tracks_count),
                }),
            )
        },
        Err(e) => {
            error!(error = %e, "Failed to import playlist");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ImportPlaylistResponse {
                    status: "error".into(),
                    message: format!("Failed to import playlist: {}", e),
                    playlist_id: None,
                    tracks_count: None,
                }),
            )
        },
    }
}

// Add individual track
#[instrument(skip(app_state))]
async fn add_track(
    State(app_state): State<AppState>,
    Json(payload): Json<AddTrackRequest>,
) -> (StatusCode, Json<Value>) {
    let AddTrackRequest { track_name, artist_name } = payload;

    info!("Adding individual track");

    match app_state.add_track(&track_name, &artist_name).await {
        Ok(track_id) => {
            info!(track_id = %track_id, "Successfully added track");
            (
                StatusCode::OK,
                Json(json!({
                    "status": "success",
                    "message": format!("Track '{}' by {} added successfully", track_name, artist_name),
                    "track_id": track_id
                }))
            )
        },
        Err(e) => {
            error!(error = %e, "Failed to add track");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("Failed to add track: {}", e)
                }))
            )
        },
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
        .route("/api/playlists/import", post(import_playlist))
        .route("/api/tracks/add", post(add_track))
        .layer(logging::create_trace_layer())
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    let addr = format!("0.0.0.0:{}", port);
    info!(address = %addr, "Starting HTTP server");
    
    info!("📖 API Documentation:");
    info!("   GET  /health - Health check");
    info!("   GET  /api/status - Service status");
    info!("   POST /api/playlists/:id/store - Store playlist in database");
    info!("   GET  /api/tracks/for-conversion - Get tracks needing conversion");
    info!("   PUT  /api/tracks/:id/youtube-url - Update track YouTube URL");
    info!("   GET  /api/playlists/:id/tracks - Get playlist tracks with YouTube URLs");
    info!("   GET  /api/tracks/search?q=query - Search tracks");
    info!("   GET  /api/conversion/stats - Get conversion statistics");
    info!("   POST /api/tracks/:id/convert - Manually convert track");
    info!("   POST /api/playlists/import - Import playlist by URL");
    info!("   POST /api/tracks/add - Add individual track");
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("🚀 Server listening on {}", addr);
    axum::serve(listener, app).await?;
    
    Ok(())
}

#[tokio::main]
async fn main() {
    // Initialize environment variables from .env file
    dotenv::dotenv().ok();
    
    // Initialize comprehensive logging
    if let Err(e) = logging::init_logging() {
        eprintln!("Failed to initialize logging: {}", e);
        std::process::exit(1);
    }

    let port = env::var("PORT").unwrap_or_else(|_| "3000".into());
    logging::log_startup_info(&port);

    let _youtube_state = youtube::app_integration::YouTubeAppState::new().await;

    // Initialize the AppState
    match AppState::new().await {
        Ok(app_state) => {
            info!("AppState initialized successfully");

            // Handle authentication
            if let Err(e) = AuthHandler::ensure_authenticated(&app_state).await {
                error!(error = %e, "Authentication failed");
                return;
            }
            info!("Authentication successful");

            // Now try to get playlist tracks and store in database
            info!("Fetching playlist tracks...");
            
            let playlist_id = "441K4rF3u0qfg9m4X1WSQJ";
            match app_state.get_playlist_tracks(playlist_id).await {
                Ok(tracks) => {
                    info!(track_count = tracks.items.len(), "Successfully fetched tracks");
                    for (i, item) in tracks.items.iter().take(5).enumerate() {
                        debug!(
                            position = i + 1,
                            track_name = %item.track.name,
                            artists = %item.track.artists.iter().map(|a| a.name.as_str()).collect::<Vec<_>>().join(", "),
                            "Track details"
                        );
                    }

                    // Store playlist in database
                    info!("Storing playlist in database...");
                    match app_state.store_playlist_in_database(playlist_id).await {
                        Ok(()) => {
                            info!("Playlist stored in database successfully");
                            
                            // Show some tracks that need YouTube conversion
                            match app_state.get_tracks_for_conversion(5).await {
                                Ok(db_tracks) => {
                                    if !db_tracks.is_empty() {
                                        info!(conversion_needed_count = db_tracks.len(), "Tracks needing YouTube conversion");
                                        for track in db_tracks.iter().take(3) {
                                            debug!(track_id = %track.id, track_name = %track.name, "Track needs conversion");
                                        }
                                    } else {
                                        info!("All tracks already have YouTube URLs");
                                    }
                                },
                                Err(e) => warn!(error = %e, "Could not check conversion status"),
                            }
                        },
                        Err(e) => {
                            error!(error = %e, "Failed to store playlist in database. Make sure Neo4j is running and connection details are correct");
                        }
                    }
                }
                Err(e) => {
                    error!(error = %e, "Failed to fetch playlist tracks");
                }
            }

            // Start the web server
            if let Err(e) = start_web_server(app_state, port).await {
                error!(error = %e, "Failed to start web server");
            }
        }
        Err(e) => {
            error!(error = %e, "Failed to initialize app state");
        }
    }
    
    logging::log_shutdown();
}