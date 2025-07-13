# Database Setup Guide

This application uses Neo4j as its graph database to store relationships between songs, artists, albums, and playlists.

## Prerequisites

1. **Neo4j Database**: You need a running Neo4j instance
2. **Environment Variables**: Properly configured database connection

## Logging and Monitoring

The backend includes comprehensive logging for monitoring database operations and performance:

### Configuration

Set logging levels in your `.env` file:

```bash
# Log levels: trace, debug, info, warn, error
RUST_LOG=info,spotify_to_youtube_backend=debug

# JSON format for production
LOG_FORMAT=json
```

### Features

- **Database Query Tracking**: All Neo4j operations are logged with timing
- **Error Context**: Detailed error information with query context
- **Performance Monitoring**: Slow database operations are flagged
- **Request Tracing**: Each API request has a unique ID for tracing through database operations

### Example Logs

```bash
# Database operations
INFO  Storing playlist in database playlist_id="441K4rF3u0qfg9m4X1WSQJ"
DEBUG Executing Neo4j query: MERGE (p:Playlist {id: $playlist_id})...
INFO  Successfully stored playlist in database

# Performance monitoring
WARN  Slow database operation detected query_time_ms="1234"

# Error tracking
ERROR Failed to store playlist error="Connection timeout" playlist_id="123"
```

For detailed logging documentation, see [LOGGING_README.md](LOGGING_README.md).

## Neo4j Installation

### Option 1: Docker (Recommended)
```bash
docker run \
    --name neo4j \
    -p7474:7474 -p7687:7687 \
    -d \
    -v $HOME/neo4j/data:/data \
    -v $HOME/neo4j/logs:/logs \
    -v $HOME/neo4j/import:/var/lib/neo4j/import \
    -v $HOME/neo4j/plugins:/plugins \
    --env NEO4J_AUTH=neo4j/password \
    neo4j:latest
```

### Option 2: Local Installation
1. Download Neo4j Desktop from https://neo4j.com/download/
2. Create a new database project
3. Set password to match your `.env` file
4. Start the database

## Database Schema

The application creates the following node types and relationships:

### Node Types:
- **Artist**: Represents music artists
  - Properties: `id`, `name`, `spotify_uri`, `external_urls`
- **Track**: Represents individual songs
  - Properties: `id`, `name`, `spotify_uri`, `duration_ms`, `explicit`, `popularity`, `preview_url`, `external_urls`, `youtube_url`, `isrc`
- **Album**: Represents music albums
  - Properties: `id`, `name`, `spotify_uri`, `album_type`, `release_date`, `total_tracks`
- **Playlist**: Represents Spotify playlists
  - Properties: `id`, `name`, `description`, `spotify_uri`, `owner_id`, `owner_display_name`, `public`, `collaborative`, `snapshot_id`, `total_tracks`

### Relationships:
- `(Artist)-[:PERFORMED]->(Track)`: Artist performed/created a track
- `(Album)-[:CONTAINS]->(Track)`: Album contains a track
- `(Playlist)-[:INCLUDES]->(Track)`: Playlist includes a track (with position property)
- `(Artist)-[:RELEASED]->(Album)`: Artist released an album

## Environment Configuration

Copy `.env.example` to `.env` and update the values:

```bash
cp .env.example .env
```

Update the Neo4j configuration in `.env`:
```
NEO4J_URI=bolt://localhost:7687
NEO4J_USER=neo4j
NEO4J_PASSWORD=your_password_here
```

## API Endpoints

Once the backend is running, you can use these endpoints:

### Store Playlist
```bash
curl -X POST http://localhost:3000/api/playlists/{playlist_id}/store
```

### Get Tracks for Conversion
```bash
curl "http://localhost:3000/api/tracks/for-conversion?limit=10"
```

### Update Track with YouTube URL
```bash
curl -X PUT http://localhost:3000/api/tracks/{track_id}/youtube-url \
  -H "Content-Type: application/json" \
  -d '{"youtube_url": "https://youtube.com/watch?v=..."}'
```

### Get Playlist Tracks with YouTube URLs
```bash
curl http://localhost:3000/api/playlists/{playlist_id}/tracks
```

### Search Tracks
```bash
curl "http://localhost:3000/api/tracks/search?q=song%20name&limit=5"
```

## Database Queries

You can run these Cypher queries in Neo4j Browser (http://localhost:7474):

### View all relationships:
```cypher
MATCH (n)-[r]->(m) RETURN n, r, m LIMIT 25
```

### Find tracks without YouTube URLs:
```cypher
MATCH (t:Track) 
WHERE t.youtube_url IS NULL 
RETURN t.name, t.id
LIMIT 10
```

### Find popular tracks by artist:
```cypher
MATCH (a:Artist)-[:PERFORMED]->(t:Track) 
WHERE a.name CONTAINS "Artist Name"
RETURN t.name, t.popularity 
ORDER BY t.popularity DESC
```

### Get playlist statistics:
```cypher
MATCH (p:Playlist)-[:INCLUDES]->(t:Track)
RETURN p.name, count(t) as track_count
ORDER BY track_count DESC
```

## Troubleshooting

1. **Connection Failed**: Ensure Neo4j is running and accessible
2. **Authentication Error**: Check username/password in `.env`
3. **Port Issues**: Make sure ports 7474 and 7687 are available
4. **Schema Issues**: The application automatically creates constraints and indexes

## Data Flow

1. **Spotify Data Ingestion**: When you store a playlist, the system:
   - Fetches playlist metadata from Spotify
   - Fetches all tracks and their details
   - Creates nodes for artists, albums, tracks, and playlists
   - Creates relationships between them

2. **YouTube URL Storage**: As tracks are converted:
   - The `youtube_url` property is updated on track nodes
   - You can query for tracks that need conversion

3. **Search and Retrieval**: The graph structure allows for:
   - Fast lookups by ISRC or track ID
   - Complex queries across relationships
   - Efficient playlist reconstruction
