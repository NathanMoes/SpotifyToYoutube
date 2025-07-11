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

    pub fn get_access_token(&self) -> Option<&String> {
        self.access_token.as_ref()
    }

    pub fn set_refresh_token(&mut self, token: String) {
        self.refresh_token = Some(token);
    }

    pub fn get_refresh_token(&self) -> Option<&String> {
        self.refresh_token.as_ref()
    }

    pub fn set_code(&mut self, code: String) {
        self.code = code;
    }

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

    pub async fn fetch_playlist_tracks(
        &mut self,
        playlist_id: &str,
    ) -> Result<serde_json::Value, SpotifyApiError> {
        self.ensure_valid_token().await?;
        
        if let Some(token) = &self.access_token {
            let client = reqwest::Client::new();
            let response = client
                .get(format!("https://api.spotify.com/v1/playlists/{}/tracks", playlist_id))
                .bearer_auth(token)
                .send()
                .await?;

            if response.status().is_success() {
                let tracks = response.json::<serde_json::Value>().await?;
                Ok(tracks)
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