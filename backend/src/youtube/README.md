# YouTube Data API v3 Setup Guide

This guide will help you set up YouTube Data API v3 with API key authentication for your application.

## Overview

This implementation uses the YouTube Data API v3 with API key authentication, which is perfect for:

- Searching for videos, channels, and playlists
- Accessing public YouTube data
- No user authentication required

For user-specific operations (like accessing private playlists), OAuth 2.0 would be required, but for search functionality, an API key is sufficient and much simpler.

## Prerequisites

1. A Google account
2. Access to the Google Cloud Console

## Step 1: Create a Google Cloud Project

1. Go to the [Google Cloud Console](https://console.cloud.google.com/)
2. Click on the project dropdown at the top of the page
3. Click "New Project"
4. Enter a project name (e.g., "SpotifyToYoutube")
5. Click "Create"

## Step 2: Enable YouTube Data API v3

1. In the Google Cloud Console, go to "APIs & Services" > "Library"
2. Search for "YouTube Data API v3"
3. Click on the YouTube Data API v3 result
4. Click "Enable"

## Step 3: Create an API Key

1. Go to "APIs & Services" > "Credentials"
2. Click "Create Credentials" > "API key"
3. Your API key will be created and displayed
4. (Optional) Click "Restrict Key" to add restrictions:
   - **Application restrictions**: Choose "HTTP referrers" for web apps or "IP addresses" for server apps
   - **API restrictions**: Select "YouTube Data API v3" to restrict the key to only this API

## Step 4: Configure Environment Variables

1. Copy your API key
2. Update your `.env` file:

```env
YOUTUBE_API_KEY=your_actual_api_key_here
```

## API Features Implemented

The current implementation provides:

### Search Functionality

- **`search_videos_simple(query, max_results)`** - Simple video search
- **`search_videos_advanced(params)`** - Advanced search with full parameter control

### Search Parameters Supported

- `query` - Search term
- `max_results` - Number of results (1-50, default: 25)
- `order` - Sort order (relevance, date, rating, title, viewCount)
- `published_after` / `published_before` - Date filters
- `region_code` - Geographic region
- `relevance_language` - Language preference
- `safe_search` - Content filtering
- `video_duration` - Duration filters (short, medium, long)
- `video_definition` - Quality filters (HD, SD)
- `video_caption` - Caption availability
- `page_token` - Pagination

### Response Data

Each search result includes:

- Video ID, title, description
- Channel information
- Thumbnail URLs (multiple sizes)
- Publication date
- Live broadcast status

## Example Usage

```rust
// Simple search
let results = youtube_state.search_videos("rust programming", Some(10)).await?;

// Advanced search
let params = YouTubeSearchParams {
    query: "music tutorial".to_string(),
    max_results: Some(25),
    order: Some("viewCount".to_string()),
    video_duration: Some("medium".to_string()),
    safe_search: Some("strict".to_string()),
    ..Default::default()
};
let results = youtube_state.search_videos_advanced(params).await?;
```

## Security Notes

1. Keep your API key secure and never commit it to version control
2. Consider using API key restrictions in Google Cloud Console
3. Monitor your API usage in the Google Cloud Console

## API Quotas

YouTube Data API v3 has quotas:

- Default quota: 10,000 units per day
- Search operations cost 100 units per request
- Monitor your usage in the Google Cloud Console under "APIs & Services" > "Quotas"

## Troubleshooting

- **"API key not valid" error**: Check that your API key is correct and the YouTube Data API v3 is enabled
- **"Quota exceeded" error**: You've hit your daily quota limit
- **"Access forbidden" error**: Your API key may have restrictions that prevent access

## Testing

After setting up the API key:

1. Run your Rust application
2. The app will automatically test YouTube search functionality
3. Search results will be displayed in the console
4. No browser authorization needed!
