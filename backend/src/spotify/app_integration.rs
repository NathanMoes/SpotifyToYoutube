use crate::spotify::spotify_api::{SpotifyApi, SpotifyPlaylistTracks, TokenState};
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;

// Example integration into your main application

pub struct AppState {
    pub spotify_api: Arc<Mutex<SpotifyApi>>,
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

        Ok(AppState { spotify_api })
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
}
