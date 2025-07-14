use crate::spotify::spotify_api::{SpotifyApi, SpotifyPlaylist, TokenState};
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

    pub async fn get_playlist_tracks(&self, playlist_id: &str) -> Result<SpotifyPlaylist, Box<dyn std::error::Error>> {
        let mut api = self.spotify_api.lock().await;
        let tracks = api.fetch_playlist(playlist_id).await?;
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
        // Get the full playlist data from Spotify
        let playlist = self.get_playlist_tracks(playlist_id).await?;
        
        // Store in database using the playlist data directly
        self.music_service.store_playlist_data(
            playlist_id,
            &playlist.name,
            playlist.description.clone(),
            &playlist.uri,
            &playlist.owner.id,
            playlist.owner.display_name.as_deref().unwrap_or("Unknown User"),
            playlist.public,
            playlist.collaborative,
            &playlist.snapshot_id,
            &playlist.tracks,
        ).await?;

        println!("✅ Stored playlist {} in database with {} tracks", playlist_id, playlist.tracks.items.len());
        Ok(())
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

    /// Get all playlists from the database
    pub async fn get_all_playlists(&self, limit: i64, offset: i64) -> Result<Vec<crate::database::DatabasePlaylist>, Box<dyn std::error::Error>> {
        self.music_service.get_all_playlists(limit, offset).await
    }

    /// Get a specific playlist from the database
    pub async fn get_playlist(&self, playlist_id: &str) -> Result<crate::database::DatabasePlaylist, Box<dyn std::error::Error>> {
        self.music_service.get_playlist(playlist_id).await
    }

    // Extract playlist ID from Spotify URL
    pub fn extract_playlist_id_from_url(url: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Handle various Spotify URL formats:
        // https://open.spotify.com/playlist/4rF3u0qfg9m4X1WSQJ
        // https://open.spotify.com/playlist/4rF3u0qfg9m4X1WSQJ?si=...
        // spotify:playlist:4rF3u0qfg9m4X1WSQJ
        
        if let Some(captures) = regex::Regex::new(r"playlist[/:]([\w\d]+)")?.captures(url) {
            if let Some(playlist_id) = captures.get(1) {
                return Ok(playlist_id.as_str().to_string());
            }
        }
        
        Err("Invalid Spotify playlist URL".into())
    }

    pub async fn import_playlist_by_url(&self, url: &str) -> Result<(String, usize), Box<dyn std::error::Error>> {
        let playlist_id = Self::extract_playlist_id_from_url(url)?;
        
        // Store the playlist in the database
        self.store_playlist_in_database(&playlist_id).await?;
        
        // Get tracks count for response
        let playlist = self.get_playlist_tracks(&playlist_id).await?;
        
        Ok((playlist_id, playlist.tracks.items.len()))
    }

    pub async fn add_track(&self, track_name: &str, artist_name: &str) -> Result<String, Box<dyn std::error::Error>> {
        // This would search for the track on Spotify and add it to a default playlist or user's collection
        // For now, let's create a simple implementation that adds it to the database
        let track_id = format!("manual_{}", uuid::Uuid::new_v4());
        
        // You would implement this method in music_service to add individual tracks
        self.music_service.add_manual_track(&track_id, track_name, artist_name).await?;
        
        Ok(track_id)
    }
}
