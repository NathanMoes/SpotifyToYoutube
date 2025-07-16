use crate::database::DatabaseTrack;
use crate::music_service::MusicDataService;
use crate::youtube::youtube_api::{YouTubeApi, YouTubeSearchParams};
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tracing::{info, warn, debug};

pub struct ConversionService {
    music_service: Arc<MusicDataService>,
    youtube_api: Option<YouTubeApi>,
}

impl ConversionService {
    pub fn new(music_service: Arc<MusicDataService>) -> Self {
        // Try to get YouTube API key from environment
        let youtube_api = std::env::var("YOUTUBE_API_KEY")
            .ok()
            .map(|api_key| YouTubeApi::new(api_key));

        if youtube_api.is_none() {
            warn!("YOUTUBE_API_KEY not found in environment variables. YouTube search will use mock URLs.");
        }

        ConversionService { 
            music_service,
            youtube_api,
        }
    }

    /// Start a background task to continuously convert tracks
    pub async fn start_background_conversion(&self) {
        info!("🔄 Background conversion service initialized (disabled for now)");
        // Background conversion disabled due to Send trait issues
        // Will be implemented in a future version
    }

    /// Manual conversion of a specific track
    pub async fn convert_track_manually(
        &self,
        track_id: &str,
        force: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        if force {
            // Force conversion - get the track directly from database regardless of YouTube URL status
            let track = self.music_service.get_track_by_id(track_id).await?
                .ok_or_else(|| format!("Track with ID {} not found", track_id))?;
            
            let youtube_url = self.search_youtube_for_track(&track).await?;
            self.music_service.update_track_youtube_url(track_id, &youtube_url).await?;
            
            Ok(youtube_url)
        } else {
            // Normal conversion - only convert tracks without YouTube URLs
            let tracks = self.music_service.get_tracks_for_conversion(1000).await?;
            
            let track = tracks.into_iter()
                .find(|t| t.id == track_id)
                .ok_or_else(|| format!("Track with ID {} not found or already has YouTube URL", track_id))?;

            let youtube_url = self.search_youtube_for_track(&track).await?;
            self.music_service.update_track_youtube_url(track_id, &youtube_url).await?;
            
            Ok(youtube_url)
        }
    }

    /// Get conversion statistics
    pub async fn get_conversion_stats(&self) -> Result<ConversionStats, Box<dyn std::error::Error>> {
        let (total_tracks, converted_tracks, pending_conversion) = self.music_service.get_conversion_stats().await?;
        Ok(ConversionStats {
            total_tracks,
            converted_tracks,
            pending_conversion,
        })
    }

    /// Process all tracks missing YouTube URLs on startup
    pub async fn process_missing_youtube_urls_on_startup(&self, batch_size: i64, max_tracks: Option<i64>) -> Result<(), Box<dyn std::error::Error>> {
        info!("🔄 Starting startup YouTube URL conversion process");
        
        let limit = max_tracks.unwrap_or(batch_size);
        
        // Get tracks that need YouTube URLs
        let tracks = self.music_service.get_tracks_for_conversion(limit).await?;
        
        if tracks.is_empty() {
            info!("✅ All tracks already have YouTube URLs");
            return Ok(());
        }

        info!("📋 Found {} tracks needing YouTube URLs", tracks.len());
        
        let mut processed = 0;
        let mut successful = 0;
        let mut failed = 0;

        for track in tracks {
            processed += 1;
            
            debug!(
                track_id = %track.id,
                track_name = %track.name,
                progress = format!("{}/{}", processed, limit),
                "Processing track for YouTube URL"
            );

            match self.find_and_update_youtube_url(&track).await {
                Ok(youtube_url) => {
                    successful += 1;
                    
                    // Check if it was actually converted or skipped
                    if track.youtube_url.is_some() && !track.youtube_url.as_ref().unwrap().is_empty() {
                        debug!(
                            track_name = %track.name,
                            existing_url = %track.youtube_url.as_ref().unwrap(),
                            "⏭️ Skipped track (already has YouTube URL)"
                        );
                    } else {
                        info!(
                            track_name = %track.name,
                            youtube_url = %youtube_url,
                            "✅ Successfully found and updated YouTube URL"
                        );
                    }
                }
                Err(e) => {
                    failed += 1;
                    warn!(
                        track_name = %track.name,
                        error = %e,
                        "❌ Failed to find YouTube URL"
                    );
                }
            }

            // Add a small delay to be respectful to the YouTube API
            if self.youtube_api.is_some() {
                sleep(Duration::from_millis(100)).await;
            }
        }

        info!(
            total_processed = processed,
            successful = successful,
            failed = failed,
            success_rate = format!("{:.1}%", (successful as f64 / processed as f64) * 100.0),
            "🏁 Completed startup YouTube URL conversion process"
        );

        Ok(())
    }

    /// Find and update YouTube URL for a single track
    async fn find_and_update_youtube_url(&self, track: &DatabaseTrack) -> Result<String, Box<dyn std::error::Error>> {
        // Check if track already has a YouTube URL
        if let Some(existing_url) = &track.youtube_url {
            if !existing_url.is_empty() {
                debug!(
                    track_id = %track.id,
                    track_name = %track.name,
                    existing_url = %existing_url,
                    "🔄 Track already has YouTube URL, skipping"
                );
                return Ok(existing_url.clone());
            }
        }
        
        let youtube_url = self.search_youtube_for_track(track).await?;
        
        // Update the database with the YouTube URL
        self.music_service.update_track_youtube_url(&track.id, &youtube_url).await?;
        
        Ok(youtube_url)
    }

    /// Search YouTube for a track using the real API or mock implementation
    async fn search_youtube_for_track(&self, track: &DatabaseTrack) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(youtube_api) = &self.youtube_api {
            // Use real YouTube API
            self.search_youtube_real_api(youtube_api, track).await
        } else {
            // Use mock implementation
            self.search_youtube_mock(track).await
        }
    }

    /// Search YouTube using the real YouTube Data API
    async fn search_youtube_real_api(&self, youtube_api: &YouTubeApi, track: &DatabaseTrack) -> Result<String, Box<dyn std::error::Error>> {
        // Get artist information for better search results
        let artists = self.music_service.get_track_artists(&track.id).await
            .unwrap_or_else(|e| {
                warn!(track_id = %track.id, error = %e, "Failed to get track artists");
                Vec::new()
            });

        // Create search query with track name and artist(s)
        let query = if !artists.is_empty() {
            let artist_names: Vec<&str> = artists.iter().map(|a| a.name.as_str()).collect();
            let primary_artist = artist_names.first().unwrap_or(&"");
            
            // Use track name + primary artist for best results
            format!("{} {}", track.name, primary_artist)
        } else {
            // Fallback: try to extract artist from Spotify URI or use track name only
            if !track.spotify_uri.is_empty() {
                // If we have Spotify URI, we can try to use it for context
                format!("{} official", track.name)
            } else {
                track.name.clone()
            }
        };

        debug!(
            track_id = %track.id,
            track_name = %track.name,
            search_query = %query,
            artist_count = artists.len(),
            "🔍 Searching YouTube with optimized query"
        );

        let search_params = YouTubeSearchParams {
            query,
            max_results: Some(5), // Get more results for better matching
            order: Some("relevance".to_string()),
            video_duration: Some("medium".to_string()), // Prefer songs over long videos
            ..Default::default()
        };

        let search_response = youtube_api.search_videos(search_params).await
            .map_err(|e| format!("YouTube API search failed: {}", e))?;

        if search_response.items.is_empty() {
            return Err("No YouTube videos found for this track".into());
        }

        // Find the best match by analyzing titles
        let best_match = self.find_best_youtube_match(&search_response.items, track, &artists)?;
        
        if let Some(video_id) = &best_match.id.video_id {
            let youtube_url = format!("https://www.youtube.com/watch?v={}", video_id);
            
            debug!(
                track_name = %track.name,
                youtube_title = %best_match.snippet.title,
                youtube_channel = %best_match.snippet.channel_title,
                youtube_url = %youtube_url,
                artist_names = ?artists.iter().map(|a| &a.name).collect::<Vec<_>>(),
                "🎯 Found optimized YouTube match"
            );
            
            Ok(youtube_url)
        } else {
            Err("YouTube search result did not contain a video ID".into())
        }
    }

    /// Find the best YouTube match from search results by analyzing titles and channels
    fn find_best_youtube_match<'a>(
        &self,
        search_items: &'a [crate::youtube::youtube_api::YouTubeSearchResult],
        track: &DatabaseTrack,
        artists: &[crate::database::DatabaseArtist],
    ) -> Result<&'a crate::youtube::youtube_api::YouTubeSearchResult, Box<dyn std::error::Error>> {
        let track_name_lower = track.name.to_lowercase();
        let artist_names_lower: Vec<String> = artists.iter()
            .map(|a| a.name.to_lowercase())
            .collect();

        let mut best_score = 0;
        let mut best_index = 0;

        for (index, item) in search_items.iter().enumerate() {
            let title_lower = item.snippet.title.to_lowercase();
            let channel_lower = item.snippet.channel_title.to_lowercase();
            
            let mut score = 0;

            // Score based on track name match in title
            if title_lower.contains(&track_name_lower) {
                score += 100;
            }

            // Score based on artist name match in title or channel
            for artist_name in &artist_names_lower {
                if title_lower.contains(artist_name) {
                    score += 80;
                }
                if channel_lower.contains(artist_name) {
                    score += 60; // Channel name match is good but not as strong as title
                }
            }

            // Prefer official channels and verified accounts
            if channel_lower.contains("official") {
                score += 20;
            }
            if channel_lower.contains("vevo") {
                score += 15;
            }

            // Slight preference for first results (they're already relevance-sorted)
            score += (search_items.len() - index) * 2;

            debug!(
                youtube_title = %item.snippet.title,
                youtube_channel = %item.snippet.channel_title,
                match_score = score,
                "Evaluating YouTube search result"
            );

            if score > best_score {
                best_score = score;
                best_index = index;
            }
        }

        if best_score == 0 {
            warn!("No good matches found, using first result");
        }

        Ok(&search_items[best_index])
    }

    /// Mock YouTube search (fallback when no API key is available)
    async fn search_youtube_mock(&self, track: &DatabaseTrack) -> Result<String, Box<dyn std::error::Error>> {
        // Get artist information for more realistic mock URLs
        let artists = self.music_service.get_track_artists(&track.id).await
            .unwrap_or_else(|e| {
                warn!(track_id = %track.id, error = %e, "Failed to get track artists for mock");
                Vec::new()
            });

        // Create a deterministic but varied mock URL based on track data and artists
        let search_query = if !artists.is_empty() {
            format!("{} {}", track.name, artists[0].name)
        } else {
            track.name.clone()
        };
        
        let mock_video_id = format!("{:x}", md5::compute(search_query.as_bytes()));
        let youtube_url = format!("https://www.youtube.com/watch?v={}", &mock_video_id[0..11]);
        
        // Simulate API delay
        sleep(Duration::from_millis(200)).await;
        
        debug!(
            track_name = %track.name,
            artist_names = ?artists.iter().map(|a| &a.name).collect::<Vec<_>>(),
            mock_youtube_url = %youtube_url,
            "🎭 Generated enhanced mock YouTube URL"
        );
        
        Ok(youtube_url)
    }
}

#[derive(Debug, Clone)]
pub struct ConversionStats {
    pub total_tracks: u64,
    pub converted_tracks: u64,
    pub pending_conversion: u64,
}
