use serde::{Deserialize, Serialize};
use gloo_net::http::Request;

#[derive(Serialize)]
pub struct ImportPlaylistRequest {
    pub url: String,
}

#[derive(Serialize)]
pub struct AddTrackRequest {
    pub track_name: String,
    pub artist_name: String,
}

#[derive(Deserialize)]
pub struct ImportPlaylistResponse {
    pub status: String,
    pub message: String,
    pub playlist_id: Option<String>,
    pub tracks_count: Option<usize>,
}

#[derive(Deserialize)]
pub struct AddTrackResponse {
    pub status: String,
    pub message: String,
    pub track_id: Option<String>,
}

pub struct ApiService {
    base_url: String,
}

impl ApiService {
    pub fn new() -> Self {
        // In production, you might want to get this from environment variables
        let base_url = "http://localhost:3000".to_string();
        Self { base_url }
    }

    pub async fn import_playlist(&self, url: String) -> Result<ImportPlaylistResponse, String> {
        let request_body = ImportPlaylistRequest { url };
        
        let url_endpoint = format!("{}/api/playlists/import", self.base_url);
        
        let response = Request::post(&url_endpoint)
            .json(&request_body)
            .map_err(|e| format!("Failed to create request: {:?}", e))?
            .send()
            .await
            .map_err(|e| format!("Request failed: {:?}", e))?;

        if !response.ok() {
            return Err(format!("HTTP error: {}", response.status()));
        }

        response.json()
            .await
            .map_err(|e| format!("Failed to parse response: {:?}", e))
    }

    pub async fn add_track(&self, track_name: String, artist_name: String) -> Result<AddTrackResponse, String> {
        let request_body = AddTrackRequest { track_name, artist_name };
        
        let url_endpoint = format!("{}/api/tracks/add", self.base_url);
        
        let response = Request::post(&url_endpoint)
            .json(&request_body)
            .map_err(|e| format!("Failed to create request: {:?}", e))?
            .send()
            .await
            .map_err(|e| format!("Request failed: {:?}", e))?;

        if !response.ok() {
            return Err(format!("HTTP error: {}", response.status()));
        }

        response.json()
            .await
            .map_err(|e| format!("Failed to parse response: {:?}", e))
    }
}
