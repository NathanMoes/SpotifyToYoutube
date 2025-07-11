use serde::{Deserialize, Serialize};

// Custom error type for better error handling
#[derive(Debug)]
pub enum YouTubeApiError {
    RequestError(reqwest::Error),
    AuthError(String),
    InvalidData(String),
}

impl std::fmt::Display for YouTubeApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            YouTubeApiError::RequestError(e) => write!(f, "Request error: {}", e),
            YouTubeApiError::AuthError(e) => write!(f, "Authentication error: {}", e),
            YouTubeApiError::InvalidData(e) => write!(f, "Invalid data: {}", e),
        }
    }
}

impl std::error::Error for YouTubeApiError {}

impl From<reqwest::Error> for YouTubeApiError {
    fn from(error: reqwest::Error) -> Self {
        YouTubeApiError::RequestError(error)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct YouTubeApi {
    pub api_key: String,
}

// YouTube Search API data structures
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct YouTubeSearchListResponse {
    pub kind: String,
    pub etag: String,
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
    #[serde(rename = "prevPageToken")]
    pub prev_page_token: Option<String>,
    #[serde(rename = "regionCode")]
    pub region_code: Option<String>,
    #[serde(rename = "pageInfo")]
    pub page_info: YouTubePageInfo,
    pub items: Vec<YouTubeSearchResult>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct YouTubePageInfo {
    #[serde(rename = "totalResults")]
    pub total_results: u32,
    #[serde(rename = "resultsPerPage")]
    pub results_per_page: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct YouTubeSearchResult {
    pub kind: String,
    pub etag: String,
    pub id: YouTubeSearchId,
    pub snippet: YouTubeSearchSnippet,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct YouTubeSearchId {
    pub kind: String,
    #[serde(rename = "videoId")]
    pub video_id: Option<String>,
    #[serde(rename = "channelId")]
    pub channel_id: Option<String>,
    #[serde(rename = "playlistId")]
    pub playlist_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct YouTubeSearchSnippet {
    #[serde(rename = "publishedAt")]
    pub published_at: String,
    #[serde(rename = "channelId")]
    pub channel_id: String,
    pub title: String,
    pub description: String,
    pub thumbnails: YouTubeThumbnails,
    #[serde(rename = "channelTitle")]
    pub channel_title: String,
    #[serde(rename = "liveBroadcastContent")]
    pub live_broadcast_content: Option<String>,
    #[serde(rename = "publishTime")]
    pub publish_time: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct YouTubeThumbnails {
    pub default: Option<YouTubeThumbnail>,
    pub medium: Option<YouTubeThumbnail>,
    pub high: Option<YouTubeThumbnail>,
    pub standard: Option<YouTubeThumbnail>,
    pub maxres: Option<YouTubeThumbnail>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct YouTubeThumbnail {
    pub url: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct YouTubeSearchParams {
    pub query: String,
    pub max_results: Option<u32>,
    pub order: Option<String>,
    pub published_after: Option<String>,
    pub published_before: Option<String>,
    pub region_code: Option<String>,
    pub relevance_language: Option<String>,
    pub safe_search: Option<String>,
    pub video_type: Option<String>,
    pub video_duration: Option<String>,
    pub video_definition: Option<String>,
    pub video_caption: Option<String>,
    pub page_token: Option<String>,
}

impl Default for YouTubeSearchParams {
    fn default() -> Self {
        Self {
            query: String::new(),
            max_results: Some(25),
            order: Some("relevance".to_string()),
            published_after: None,
            published_before: None,
            region_code: None,
            relevance_language: None,
            safe_search: Some("moderate".to_string()),
            video_type: None,
            video_duration: None,
            video_definition: None,
            video_caption: None,
            page_token: None,
        }
    }
}

impl YouTubeApi {
    pub fn new(api_key: String) -> Self {
        YouTubeApi { api_key }
    }

    pub async fn search_videos(&self, params: YouTubeSearchParams) -> Result<YouTubeSearchListResponse, YouTubeApiError> {
        let client = reqwest::Client::new();
        
        let mut url = format!(
            "https://www.googleapis.com/youtube/v3/search?part=snippet&key={}&q={}",
            urlencoding::encode(&self.api_key),
            urlencoding::encode(&params.query)
        );

        // Add optional parameters
        if let Some(max_results) = params.max_results {
            url.push_str(&format!("&maxResults={}", max_results));
        }

        if let Some(order) = &params.order {
            url.push_str(&format!("&order={}", urlencoding::encode(order)));
        }

        if let Some(published_after) = &params.published_after {
            url.push_str(&format!("&publishedAfter={}", urlencoding::encode(published_after)));
        }

        if let Some(published_before) = &params.published_before {
            url.push_str(&format!("&publishedBefore={}", urlencoding::encode(published_before)));
        }

        if let Some(region_code) = &params.region_code {
            url.push_str(&format!("&regionCode={}", urlencoding::encode(region_code)));
        }

        if let Some(relevance_language) = &params.relevance_language {
            url.push_str(&format!("&relevanceLanguage={}", urlencoding::encode(relevance_language)));
        }

        if let Some(safe_search) = &params.safe_search {
            url.push_str(&format!("&safeSearch={}", urlencoding::encode(safe_search)));
        }

        if let Some(video_type) = &params.video_type {
            url.push_str(&format!("&videoType={}", urlencoding::encode(video_type)));
        }

        if let Some(video_duration) = &params.video_duration {
            url.push_str(&format!("&videoDuration={}", urlencoding::encode(video_duration)));
        }

        if let Some(video_definition) = &params.video_definition {
            url.push_str(&format!("&videoDefinition={}", urlencoding::encode(video_definition)));
        }

        if let Some(video_caption) = &params.video_caption {
            url.push_str(&format!("&videoCaption={}", urlencoding::encode(video_caption)));
        }

        if let Some(page_token) = &params.page_token {
            url.push_str(&format!("&pageToken={}", urlencoding::encode(page_token)));
        }

        // Restrict to video type only for better search results
        url.push_str("&type=video");

        println!("🔍 YouTube API URL: {}", url);

        let response = client.get(&url).send().await?;

        if response.status().is_success() {
            let search_response: YouTubeSearchListResponse = response.json().await?;
            Ok(search_response)
        } else {
            let error_text = response.text().await?;
            Err(YouTubeApiError::InvalidData(format!("Search failed: {}", error_text)))
        }
    }

    pub async fn search_videos_simple(&self, query: &str, max_results: Option<u32>) -> Result<YouTubeSearchListResponse, YouTubeApiError> {
        let params = YouTubeSearchParams {
            query: query.to_string(),
            max_results: max_results.or(Some(10)),
            ..Default::default()
        };

        self.search_videos(params).await
    }
}