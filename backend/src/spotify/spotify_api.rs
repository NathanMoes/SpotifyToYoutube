use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::{sleep, Duration};

// Custom error type for better error handling
#[derive(Debug)]
pub enum SpotifyApiError {
    RequestError(reqwest::Error),
    AuthError(String),
    InvalidData(String),
}

impl std::fmt::Display for SpotifyApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpotifyApiError::RequestError(e) => write!(f, "Request error: {}", e),
            SpotifyApiError::AuthError(e) => write!(f, "Authentication error: {}", e),
            SpotifyApiError::InvalidData(e) => write!(f, "Invalid data: {}", e),
        }
    }
}

impl std::error::Error for SpotifyApiError {}

impl From<reqwest::Error> for SpotifyApiError {
    fn from(error: reqwest::Error) -> Self {
        SpotifyApiError::RequestError(error)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpotifyApi {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub code: String,
    pub expires_at: Option<u64>, // Unix timestamp when token expires
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpotifyAuthResponse {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
    pub expires_in: u64,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpotifyUserAuthResponse {
    pub code: String,
    pub state: String,
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenState {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpotifyExternalUrls {
    pub spotify: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpotifyImage {
    pub url: String,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpotifyUser {
    pub external_urls: SpotifyExternalUrls,
    pub href: String,
    pub id: String,
    #[serde(rename = "type")]
    pub user_type: String,
    pub uri: String,
    pub display_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpotifyArtist {
    pub external_urls: SpotifyExternalUrls,
    pub href: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub artist_type: String,
    pub uri: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpotifyRestrictions {
    pub reason: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpotifyAlbum {
    pub album_type: String,
    pub total_tracks: u32,
    pub available_markets: Vec<String>,
    pub external_urls: SpotifyExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<SpotifyImage>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: String,
    pub restrictions: Option<SpotifyRestrictions>,
    #[serde(rename = "type")]
    pub album_type_field: String,
    pub uri: String,
    pub artists: Vec<SpotifyArtist>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpotifyExternalIds {
    pub isrc: Option<String>,
    pub ean: Option<String>,
    pub upc: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpotifyTrack {
    pub album: SpotifyAlbum,
    pub artists: Vec<SpotifyArtist>,
    pub available_markets: Vec<String>,
    pub disc_number: u32,
    pub duration_ms: u32,
    pub explicit: bool,
    pub external_ids: SpotifyExternalIds,
    pub external_urls: SpotifyExternalUrls,
    pub href: String,
    pub id: String,
    pub is_playable: Option<bool>,
    pub linked_from: Option<serde_json::Value>,
    pub restrictions: Option<SpotifyRestrictions>,
    pub name: String,
    pub popularity: u32,
    pub preview_url: Option<String>,
    pub track_number: u32,
    #[serde(rename = "type")]
    pub track_type: String,
    pub uri: String,
    pub is_local: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpotifyPlaylistTrackItem {
    pub added_at: String,
    pub added_by: SpotifyUser,
    pub is_local: bool,
    pub track: SpotifyTrack,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpotifyPlaylistTracks {
    pub href: String,
    pub limit: u32,
    pub next: Option<String>,
    pub offset: u32,
    pub previous: Option<String>,
    pub total: u32,
    pub items: Vec<SpotifyPlaylistTrackItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpotifyPlaylist {
    pub collaborative: bool,
    pub description: Option<String>,
    pub external_urls: SpotifyExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<SpotifyImage>,
    pub name: String,
    pub owner: SpotifyUser,
    pub public: bool,
    pub snapshot_id: String,
    pub tracks: SpotifyPlaylistTracks,
    #[serde(rename = "type")]
    pub playlist_type: String,
    pub uri: String,
}

impl SpotifyApi {
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        SpotifyApi {
            client_id,
            client_secret,
            redirect_uri,
            access_token: None,
            refresh_token: None,
            code: String::new(),
            expires_at: None,
        }
    }

    /// Initialize with existing tokens (useful for restoring from storage)
    #[allow(dead_code)]
    pub fn with_tokens(
        client_id: String,
        client_secret: String,
        redirect_uri: String,
        access_token: String,
        refresh_token: String,
        expires_at: Option<u64>,
    ) -> Self {
        SpotifyApi {
            client_id,
            client_secret,
            redirect_uri,
            access_token: Some(access_token),
            refresh_token: Some(refresh_token),
            code: String::new(),
            expires_at,
        }
    }

    /// Initialize with just the authorization code to get initial tokens
    #[allow(dead_code)]
    pub async fn from_auth_code(
        client_id: String,
        client_secret: String,
        redirect_uri: String,
        code: String,
    ) -> Result<Self, SpotifyApiError> {
        let mut api = SpotifyApi::new(client_id, client_secret, redirect_uri);
        api.set_code(code);
        api.update_access_token().await?;
        Ok(api)
    }

    pub fn get_auth_url(&self) -> String {
        format!(
            "https://accounts.spotify.com/authorize?client_id={}&response_type=code&redirect_uri={}&scope=user-read-private user-read-email user-read-playback-state playlist-read-private playlist-read-collaborative",
            self.client_id, self.redirect_uri
        )
    }

    pub fn set_access_token(&mut self, token: String) {
        self.access_token = Some(token);
    }

    #[allow(dead_code)]
    pub fn get_access_token(&self) -> Option<&String> {
        self.access_token.as_ref()
    }

    pub fn set_refresh_token(&mut self, token: String) {
        self.refresh_token = Some(token);
    }

    pub fn get_refresh_token(&self) -> Option<&String> {
        self.refresh_token.as_ref()
    }

    #[allow(dead_code)]
    pub fn set_code(&mut self, code: String) {
        self.code = code;
    }

    #[allow(dead_code)]
    pub fn get_code(&self) -> &String {
        &self.code
    }

    pub fn set_expires_at(&mut self, expires_in: u64) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        self.expires_at = Some(now + expires_in);
    }

    pub fn is_token_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            // Check if token expires within the next 2 minutes (120 seconds)
            now >= expires_at - 120
        } else {
            true // If no expiration time is set, assume expired
        }
    }

    pub async fn ensure_valid_token(&mut self) -> Result<(), SpotifyApiError> {
        if self.access_token.is_none() || self.is_token_expired() {
            if self.refresh_token.is_some() {
                self.refresh_access_token().await?;
            } else {
                return Err(SpotifyApiError::AuthError(
                    "No refresh token available. Re-authorization required.".to_string(),
                ));
            }
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn update_access_token(
        &mut self,
    ) -> Result<SpotifyAuthResponse, SpotifyApiError> {
        if self.get_code().is_empty() {
            return Err(SpotifyApiError::AuthError(
                "Authorization code is required".to_string(),
            ));
        }
        
        let client = reqwest::Client::new();
        let response = client
            .post("https://accounts.spotify.com/api/token")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .form(&[
                ("grant_type", "authorization_code"),
                ("code", self.get_code()),
                ("redirect_uri", &self.redirect_uri),
            ])
            .send()
            .await?;

        if response.status().is_success() {
            let auth_response = response.json::<SpotifyAuthResponse>().await?;
            self.set_access_token(auth_response.access_token.clone());
            self.set_refresh_token(auth_response.refresh_token.clone());
            self.set_expires_at(auth_response.expires_in);
            Ok(auth_response)
        } else {
            let error_text = response.text().await?;
            Err(SpotifyApiError::AuthError(
                format!("Failed to update access token: {}", error_text),
            ))
        }
    }

    pub async fn refresh_access_token(&mut self) -> Result<SpotifyAuthResponse, SpotifyApiError> {
        let refresh_token = self.get_refresh_token().ok_or_else(|| {
            SpotifyApiError::AuthError("No refresh token available".to_string())
        })?;

        let client = reqwest::Client::new();
        let response = client
            .post("https://accounts.spotify.com/api/token")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .form(&[
                ("grant_type", "refresh_token"),
                ("refresh_token", refresh_token),
            ])
            .send()
            .await?;

        if response.status().is_success() {
            let auth_response = response.json::<SpotifyAuthResponse>().await?;
            self.set_access_token(auth_response.access_token.clone());
            
            // Update refresh token if a new one is provided
            if !auth_response.refresh_token.is_empty() {
                self.set_refresh_token(auth_response.refresh_token.clone());
            }
            
            self.set_expires_at(auth_response.expires_in);
            println!("Token refreshed successfully. Expires in {} seconds", auth_response.expires_in);
            Ok(auth_response)
        } else {
            let error_text = response.text().await?;
            Err(SpotifyApiError::AuthError(
                format!("Failed to refresh access token: {}", error_text),
            ))
        }
    }

    pub async fn start_token_refresh_loop(mut self) {
        loop {
            if let Some(expires_at) = self.expires_at {
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                let time_until_refresh = expires_at.saturating_sub(now + 120); // Refresh 2 minutes before expiry
                
                if time_until_refresh > 0 {
                    sleep(Duration::from_secs(time_until_refresh)).await;
                }
                
                if let Err(e) = self.refresh_access_token().await {
                    eprintln!("Failed to refresh token: {}", e);
                    // Sleep for 5 minutes before retrying
                    sleep(Duration::from_secs(300)).await;
                }
            } else {
                // If no expiration time is set, sleep for 55 minutes (assuming 1 hour token life)
                sleep(Duration::from_secs(3300)).await;
            }
        }
    }

    #[allow(dead_code)]
    pub async fn fetch_user_profile(&mut self) -> Result<serde_json::Value, SpotifyApiError> {
        self.ensure_valid_token().await?;
        
        if let Some(token) = &self.access_token {
            let client = reqwest::Client::new();
            let response = client
                .get("https://api.spotify.com/v1/me")
                .bearer_auth(token)
                .send()
                .await?;

            if response.status().is_success() {
                let user_profile = response.json::<serde_json::Value>().await?;
                Ok(user_profile)
            } else {
                Err(SpotifyApiError::AuthError(
                    "Unauthorized access".to_string(),
                ))
            }
        } else {
            Err(SpotifyApiError::InvalidData(
                "No access token available".to_string(),
            ))
        }
    }

    #[allow(dead_code)]
    pub async fn fetch_user_playlists(&mut self) -> Result<serde_json::Value, SpotifyApiError> {
        self.ensure_valid_token().await?;
        
        if let Some(token) = &self.access_token {
            let client = reqwest::Client::new();
            let response = client
                .get("https://api.spotify.com/v1/me/playlists")
                .bearer_auth(token)
                .send()
                .await?;

            if response.status().is_success() {
                let playlists = response.json::<serde_json::Value>().await?;
                Ok(playlists)
            } else {
                Err(SpotifyApiError::AuthError(
                    "Unauthorized access".to_string(),
                ))
            }
        } else {
            Err(SpotifyApiError::InvalidData(
                "No access token available".to_string(),
            ))
        }
    }

    #[allow(dead_code)]
    pub async fn fetch_playlist(
        &mut self,
        playlist_id: &str,
    ) -> Result<SpotifyPlaylist, SpotifyApiError> {
        self.ensure_valid_token().await?;
        
        if let Some(token) = &self.access_token {
            let client = reqwest::Client::new();
            let response = client
                .get(format!("https://api.spotify.com/v1/playlists/{}", playlist_id))
                .bearer_auth(token)
                .send()
                .await?;

            if response.status().is_success() {
                let playlist = response.json::<SpotifyPlaylist>().await?;
                Ok(playlist)
            } else {
                let status = response.status();
                let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                Err(SpotifyApiError::AuthError(
                    format!("The client is unauthorized due to authentication failure. Status: {}, Error: {}", status, error_text),
                ))
            }
        } else {
            Err(SpotifyApiError::InvalidData(
                "No access token available".to_string(),
            ))
        }
    }

    /// Get tracks from a playlist by fetching the full playlist data
    #[allow(dead_code)]
    pub async fn get_playlist_tracks(
        &mut self,
        playlist_id: &str,
    ) -> Result<SpotifyPlaylistTracks, SpotifyApiError> {
        let playlist = self.fetch_playlist(playlist_id).await?;
        Ok(playlist.tracks)
    }

    /// Fetch all tracks from a playlist with pagination support
    /// This method will make multiple API calls to get all tracks if the playlist has more than 100 tracks
    pub async fn fetch_all_playlist_tracks(
        &mut self,
        playlist_id: &str,
    ) -> Result<SpotifyPlaylist, SpotifyApiError> {
        self.ensure_valid_token().await?;
        
        if let Some(token) = &self.access_token {
            let client = reqwest::Client::new();
            
            // First, get the basic playlist info with the first batch of tracks
            let response = client
                .get(format!("https://api.spotify.com/v1/playlists/{}", playlist_id))
                .bearer_auth(token)
                .send()
                .await?;

            if !response.status().is_success() {
                let status = response.status();
                let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                return Err(SpotifyApiError::AuthError(
                    format!("The client is unauthorized due to authentication failure. Status: {}, Error: {}", status, error_text),
                ));
            }

            let mut playlist = response.json::<SpotifyPlaylist>().await?;
            
            // If there are more tracks to fetch, get them via pagination
            let mut next_url = playlist.tracks.next.clone();
            while let Some(url) = next_url {
                let response = client
                    .get(&url)
                    .bearer_auth(token)
                    .send()
                    .await?;

                if !response.status().is_success() {
                    let status = response.status();
                    let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                    return Err(SpotifyApiError::AuthError(
                        format!("Failed to fetch additional tracks. Status: {}, Error: {}", status, error_text),
                    ));
                }

                let additional_tracks = response.json::<SpotifyPlaylistTracks>().await?;
                
                // Append the new tracks to the existing ones
                playlist.tracks.items.extend(additional_tracks.items);
                
                // Update the next URL for the next iteration
                next_url = additional_tracks.next;
                
                // Add a small delay to avoid hitting rate limits
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
            
            // Update the tracks info to reflect the complete data
            playlist.tracks.limit = playlist.tracks.items.len() as u32;
            playlist.tracks.offset = 0;
            playlist.tracks.next = None;
            playlist.tracks.previous = None;
            
            Ok(playlist)
        } else {
            Err(SpotifyApiError::InvalidData(
                "No access token available".to_string(),
            ))
        }
    }

    /// Get current token state for persistence
    pub fn get_token_state(&self) -> Option<TokenState> {
        if let (Some(access_token), Some(refresh_token)) = (&self.access_token, &self.refresh_token) {
            Some(TokenState {
                access_token: access_token.clone(),
                refresh_token: refresh_token.clone(),
                expires_at: self.expires_at,
            })
        } else {
            None
        }
    }

    /// Restore token state from persistence
    pub fn restore_token_state(&mut self, token_state: TokenState) {
        self.access_token = Some(token_state.access_token);
        self.refresh_token = Some(token_state.refresh_token);
        self.expires_at = token_state.expires_at;
    }
}