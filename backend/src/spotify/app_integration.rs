use crate::spotify::spotify_api::{SpotifyApi, SpotifyPlaylistTracks, TokenState};
use crate::music_service::MusicDataService;
use crate::conversion_service::ConversionService;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;

// Example integration into your main application

#[derive(Clone)]
pub struct AppState {
    pub spotify_api: Arc<Mutex<SpotifyApi>>,
    pub music_service: Arc<MusicDataService>,
    pub conversion_service: Arc<ConversionService>,
}

impl AppState {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let client_id = env::var("SPOTIFY_CLIENT_ID")?;
        let client_secret = env::var("SPOTIFY_CLIENT_SECRET")?;
        let redirect_uri = env::var("SPOTIFY_REDIRECT_URI")?;

        println!("Client ID: {}", client_id);
        println!("Client Secret: {}", client_secret);
        println!("Redirect URI: {}", redirect_uri);

        // Try to load existing tokens first
        let mut spotify_api = SpotifyApi::new(client_id, client_secret, redirect_uri);
        
        // Try to load from file
        if let Ok(json) = std::fs::read_to_string("tokens.json") {
            if let Ok(token_state) = serde_json::from_str::<TokenState>(&json) {
                spotify_api.restore_token_state(token_state);
                println!("Loaded existing tokens");
            }
        }

        let spotify_api = Arc::new(Mutex::new(spotify_api));

        // Start background token refresh
        let refresh_api = spotify_api.clone();
        tokio::spawn(async move {
            let api = refresh_api.lock().await.clone();
            api.start_token_refresh_loop().await;
        });

        // Auto-save tokens periodically
        let save_api = spotify_api.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(300)); // Save every 5 minutes
            loop {
                interval.tick().await;
                let api = save_api.lock().await;
                if let Some(token_state) = api.get_token_state() {
                    if let Ok(json) = serde_json::to_string_pretty(&token_state) {
                        let _ = std::fs::write("tokens.json", json);
                    }
                }
            }
        });

        // Initialize music service
        let music_service = match MusicDataService::new().await {
            Ok(service) => Arc::new(service),
            Err(e) => {
                eprintln!("Warning: Failed to initialize music service: {:?}", e);
                eprintln!("Make sure Neo4j is running and accessible");
                return Err(format!("Database connection failed: {:?}", e).into());
            }
        };

        // Initialize conversion service
        let conversion_service = Arc::new(ConversionService::new(music_service.clone()));
        
        // Start background conversion
        conversion_service.start_background_conversion().await;
        println!("🔄 Background conversion service started");

        Ok(AppState { 
            spotify_api,
            music_service,
            conversion_service,
        })
    }

    #[allow(dead_code)]
    pub async fn get_user_playlists(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let mut api = self.spotify_api.lock().await;
        let playlists = api.fetch_user_playlists().await?;
        Ok(playlists)
    }

    pub async fn get_playlist_tracks(&self, playlist_id: &str) -> Result<SpotifyPlaylistTracks, Box<dyn std::error::Error>> {
        let mut api = self.spotify_api.lock().await;
        let tracks = api.fetch_playlist_tracks(playlist_id).await?;
        Ok(tracks)
    }

    pub async fn handle_auth_callback(&self, code: String) -> Result<(), Box<dyn std::error::Error>> {
        let mut api = self.spotify_api.lock().await;
        api.set_code(code);
        api.update_access_token().await?;
        
        // Save tokens immediately after successful auth
        if let Some(token_state) = api.get_token_state() {
            let json = serde_json::to_string_pretty(&token_state)?;
            std::fs::write("tokens.json", json)?;
        }
        
        Ok(())
    }

    pub async fn get_auth_url(&self) -> String {
        let api = self.spotify_api.lock().await;
        api.get_auth_url()
    }

    // Database integration methods
    pub async fn store_playlist_in_database(&self, playlist_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        // First get the playlist tracks from Spotify
        let tracks = self.get_playlist_tracks(playlist_id).await?;
        
        // Get playlist metadata (you might need to implement this in SpotifyApi)
        let playlist_info = self.get_playlist_info(playlist_id).await?;
        
        // Store in database
        self.music_service.store_playlist_data(
            playlist_id,
            &playlist_info["name"].as_str().unwrap_or("Unknown Playlist"),
            playlist_info["description"].as_str().map(|s| s.to_string()),
            &playlist_info["uri"].as_str().unwrap_or(""),
            &playlist_info["owner"]["id"].as_str().unwrap_or(""),
            &playlist_info["owner"]["display_name"].as_str().unwrap_or("Unknown User"),
            playlist_info["public"].as_bool().unwrap_or(false),
            playlist_info["collaborative"].as_bool().unwrap_or(false),
            &playlist_info["snapshot_id"].as_str().unwrap_or(""),
            &tracks,
        ).await?;

        println!("✅ Stored playlist {} in database with {} tracks", playlist_id, tracks.items.len());
        Ok(())
    }

    pub async fn get_playlist_info(&self, playlist_id: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let mut api = self.spotify_api.lock().await;
        let playlist = api.fetch_playlist(playlist_id).await?;
        let playlist_json = serde_json::to_value(playlist)?;
        Ok(playlist_json)
    }

    pub async fn get_tracks_for_conversion(&self, limit: i64) -> Result<Vec<crate::database::DatabaseTrack>, Box<dyn std::error::Error>> {
        let tracks = self.music_service.get_tracks_for_conversion(limit).await?;
        Ok(tracks)
    }

    pub async fn update_track_youtube_url(&self, track_id: &str, youtube_url: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.music_service.update_track_youtube_url(track_id, youtube_url).await?;
        Ok(())
    }

    pub async fn get_playlist_tracks_with_youtube(&self, playlist_id: &str) -> Result<Vec<(crate::database::DatabaseTrack, i64)>, Box<dyn std::error::Error>> {
        let tracks = self.music_service.get_playlist_tracks_with_youtube(playlist_id).await?;
        Ok(tracks)
    }

    pub async fn search_tracks_in_database(&self, name: &str, limit: i64) -> Result<Vec<crate::database::DatabaseTrack>, Box<dyn std::error::Error>> {
        let tracks = self.music_service.search_tracks(name, limit).await?;
        Ok(tracks)
    }
}
