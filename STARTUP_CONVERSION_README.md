# Startup YouTube URL Conversion

This feature automatically searches for YouTube URLs for tracks missing them when the backend starts up.

## How it Works

1. **On Startup**: After database initialization, the backend automatically searches for tracks without YouTube URLs
2. **Batch Processing**: Processes tracks in configurable batches (default: up to 100 tracks)
3. **Smart Search**: Uses either the YouTube Data API (if configured) or generates mock URLs for testing
4. **Respectful Rate Limiting**: Adds small delays between API calls to respect YouTube's rate limits
5. **Progress Logging**: Provides detailed logging of the conversion process

## Configuration

### YouTube API Key (Optional but Recommended)

Set the `YOUTUBE_API_KEY` environment variable to use real YouTube searches:

```bash
export YOUTUBE_API_KEY="your_youtube_data_api_key_here"
```

Without this key, the system will generate deterministic mock YouTube URLs for testing purposes.

### Environment Variables

Add to your `.env` file:

```env
# Required for database
NEO4J_URI=bolt://localhost:7687
NEO4J_USER=neo4j
NEO4J_PASSWORD=your_password

# Optional - for real YouTube searches
YOUTUBE_API_KEY=your_youtube_data_api_key_here

# Other existing variables...
```

## Getting a YouTube API Key

1. Go to the [Google Cloud Console](https://console.cloud.google.com/)
2. Create a new project or select an existing one
3. Enable the YouTube Data API v3
4. Create credentials (API Key)
5. Copy the API key to your environment variables

## Startup Process Flow

```
Backend Startup
    ↓
Database Connection
    ↓
Spotify Authentication
    ↓
Playlist Import (existing playlist)
    ↓
🚀 YouTube URL Conversion Process
    ↓
    ├─ Find tracks without YouTube URLs
    ├─ Search YouTube for each track
    ├─ Update database with found URLs
    └─ Report conversion statistics
    ↓
Web Server Start
```

## Conversion Process Details

### Real YouTube API Search
- Searches using track name and attempts to find the best match
- Filters for video type content
- Prefers medium-duration videos (songs vs long videos)
- Takes the most relevant result
- Logs track name, YouTube title, and channel for verification

### Mock Mode (No API Key)
- Generates deterministic URLs based on track name hash
- Useful for development and testing
- URLs follow format: `https://www.youtube.com/watch?v=[hash]`

## Logs and Monitoring

Look for these log messages during startup:

```
🚀 Starting automatic YouTube URL conversion for missing tracks...
📋 Found X tracks needing YouTube URLs
✅ Successfully found and updated YouTube URL
🏁 Completed startup YouTube URL conversion process
```

## Customization

You can modify the conversion parameters in `main.rs`:

```rust
// Process up to 100 tracks in batches of 50
app_state.conversion_service.process_missing_youtube_urls_on_startup(50, Some(100)).await
```

## Manual Conversion

You can also trigger manual conversions via the API:

```bash
# Convert a specific track
POST /api/tracks/{track_id}/convert

# Get conversion statistics
GET /api/conversion/stats
```

## Troubleshooting

### No YouTube URLs Found
- Check that `YOUTUBE_API_KEY` is set correctly
- Verify the YouTube Data API is enabled in Google Cloud Console
- Check API quotas and limits

### Rate Limiting
- The system includes built-in delays (100ms between requests)
- Monitor YouTube API quotas in Google Cloud Console
- Consider reducing batch sizes for large databases

### Database Issues
- Ensure Neo4j is running and accessible
- Check database connection logs
- Verify track data exists in the database

## Performance Considerations

- **Startup Time**: Conversion process will extend startup time
- **API Quotas**: YouTube Data API has daily quotas
- **Batch Size**: Adjust based on your track count and API limits
- **Network**: Requires internet connection for YouTube searches
