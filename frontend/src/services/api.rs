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

// Database track structure matching backend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseTrack {
    pub id: String,
    pub name: String,
    pub spotify_uri: String,
    pub duration_ms: u32,
    pub explicit: bool,
    pub popularity: u32,
    pub preview_url: Option<String>,
    pub external_urls: String,
    pub youtube_url: Option<String>,
    pub isrc: Option<String>,
}

// Database playlist structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DatabasePlaylist {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub spotify_uri: String,
    pub owner_id: String,
    pub owner_display_name: String,
    pub public: bool,
    pub collaborative: bool,
    pub snapshot_id: String,
    pub total_tracks: u32,
}

#[derive(Deserialize)]
pub struct TracksForConversionResponse {
    pub status: String,
    pub tracks: Vec<DatabaseTrack>,
    pub count: usize,
}

#[derive(Deserialize)]
pub struct SearchTracksResponse {
    pub status: String,
    pub query: String,
    pub tracks: Vec<DatabaseTrack>,
    pub count: usize,
}

#[derive(Deserialize)]
pub struct PlaylistsResponse {
    pub status: String,
    pub playlists: Vec<DatabasePlaylist>,
    pub count: usize,
}

#[derive(Deserialize)]
pub struct PlaylistTracksResponse {
    pub status: String,
    pub playlist: DatabasePlaylist,
    pub tracks: Vec<DatabaseTrack>,
    pub count: usize,
}

#[derive(Deserialize)]
pub struct ConversionStatsResponse {
    pub status: String,
    pub total_tracks: u32,
    pub tracks_with_youtube: u32,
    pub tracks_without_youtube: u32,
    pub conversion_percentage: f32,
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

    pub async fn get_tracks_for_conversion(&self, limit: Option<i64>, offset: Option<i64>) -> Result<TracksForConversionResponse, String> {
        let limit_param = limit.unwrap_or(50);
        let offset_param = offset.unwrap_or(0);
        let url_endpoint = format!("{}/api/tracks/for-conversion?limit={}&offset={}", self.base_url, limit_param, offset_param);
        
        let response = Request::get(&url_endpoint)
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

    pub async fn search_tracks(&self, query: &str, limit: Option<i64>) -> Result<SearchTracksResponse, String> {
        let limit_param = limit.unwrap_or(20);
        let url_endpoint = format!("{}/api/tracks/search?q={}&limit={}", self.base_url, 
            urlencoding::encode(query), limit_param);
        
        let response = Request::get(&url_endpoint)
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

    pub async fn get_playlists(&self, limit: Option<i64>, offset: Option<i64>) -> Result<PlaylistsResponse, String> {
        let limit_param = limit.unwrap_or(50);
        let offset_param = offset.unwrap_or(0);
        let url_endpoint = format!("{}/api/playlists?limit={}&offset={}", self.base_url, limit_param, offset_param);
        
        let response = Request::get(&url_endpoint)
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

    pub async fn get_playlist_tracks(&self, playlist_id: &str, limit: Option<i64>, offset: Option<i64>) -> Result<PlaylistTracksResponse, String> {
        let limit_param = limit.unwrap_or(50);
        let offset_param = offset.unwrap_or(0);
        let url_endpoint = format!("{}/api/playlists/{}/tracks?limit={}&offset={}", self.base_url, playlist_id, limit_param, offset_param);
        
        let response = Request::get(&url_endpoint)
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

    pub async fn get_conversion_stats(&self) -> Result<ConversionStatsResponse, String> {
        let url_endpoint = format!("{}/api/conversion/stats", self.base_url);
        
        let response = Request::get(&url_endpoint)
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

    pub async fn update_track_youtube_url(&self, track_id: &str, youtube_url: &str) -> Result<(), String> {
        let url_endpoint = format!("{}/api/tracks/{}/youtube-url", self.base_url, track_id);
        
        let payload = serde_json::json!({
            "youtube_url": youtube_url
        });
        
        let response = Request::put(&url_endpoint)
            .json(&payload)
            .map_err(|e| format!("Failed to create request: {:?}", e))?
            .send()
            .await
            .map_err(|e| format!("Request failed: {:?}", e))?;

        if !response.ok() {
            return Err(format!("HTTP error: {}", response.status()));
        }

        Ok(())
    }
}
