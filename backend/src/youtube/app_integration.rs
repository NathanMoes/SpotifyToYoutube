use crate::youtube::youtube_api::{YouTubeApi, YouTubeSearchListResponse, YouTubeSearchParams};
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;

// YouTube integration into your main application

pub struct YouTubeAppState {
    pub youtube_api: Arc<Mutex<YouTubeApi>>,
}

impl YouTubeAppState {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let api_key = env::var("YOUTUBE_API_KEY")?;

        println!("YouTube API Key: {}", api_key);

        let youtube_api = YouTubeApi::new(api_key);
        let youtube_api = Arc::new(Mutex::new(youtube_api));

        Ok(YouTubeAppState { youtube_api })
    }

    pub async fn search_videos(&self, query: &str, max_results: Option<u32>) -> Result<YouTubeSearchListResponse, Box<dyn std::error::Error>> {
        let api = self.youtube_api.lock().await;
        let results = api.search_videos_simple(query, max_results).await?;
        Ok(results)
    }

    pub async fn search_videos_advanced(&self, params: YouTubeSearchParams) -> Result<YouTubeSearchListResponse, Box<dyn std::error::Error>> {
        let api = self.youtube_api.lock().await;
        let results = api.search_videos(params).await?;
        Ok(results)
    }
}
