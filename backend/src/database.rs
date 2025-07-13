use neo4rs::{Graph, Query};
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseArtist {
    pub id: String,
    pub name: String,
    pub spotify_uri: String,
    pub external_urls: String,
}

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
    pub youtube_url: Option<String>, // For converted YouTube URL
    pub isrc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseAlbum {
    pub id: String,
    pub name: String,
    pub spotify_uri: String,
    pub album_type: String,
    pub release_date: String,
    pub total_tracks: u32,
}

#[derive(Clone)]
pub struct DatabaseManager {
    graph: Arc<Graph>,
}

impl DatabaseManager {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let uri = env::var("NEO4J_URI").unwrap_or_else(|_| "bolt://localhost:7687".to_string());
        let user = env::var("NEO4J_USER").unwrap_or_else(|_| "neo4j".to_string());
        let password = env::var("NEO4J_PASSWORD").unwrap_or_else(|_| "password".to_string());

        let graph = Graph::new(&uri, &user, &password).await
            .map_err(|e| format!("Failed to connect to Neo4j: {}", e))?;
        
        let db_manager = DatabaseManager {
            graph: Arc::new(graph),
        };

        // Initialize database schema
        db_manager.initialize_schema().await?;

        Ok(db_manager)
    }

    async fn initialize_schema(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Create constraints and indexes
        let queries = vec![
            // Unique constraints
            "CREATE CONSTRAINT artist_id IF NOT EXISTS FOR (a:Artist) REQUIRE a.id IS UNIQUE",
            "CREATE CONSTRAINT track_id IF NOT EXISTS FOR (t:Track) REQUIRE t.id IS UNIQUE", 
            "CREATE CONSTRAINT playlist_id IF NOT EXISTS FOR (p:Playlist) REQUIRE p.id IS UNIQUE",
            "CREATE CONSTRAINT album_id IF NOT EXISTS FOR (al:Album) REQUIRE al.id IS UNIQUE",
            
            // Indexes for performance
            "CREATE INDEX artist_name IF NOT EXISTS FOR (a:Artist) ON (a.name)",
            "CREATE INDEX track_name IF NOT EXISTS FOR (t:Track) ON (t.name)",
            "CREATE INDEX playlist_name IF NOT EXISTS FOR (p:Playlist) ON (p.name)",
            "CREATE INDEX track_isrc IF NOT EXISTS FOR (t:Track) ON (t.isrc)",
            "CREATE INDEX track_youtube_url IF NOT EXISTS FOR (t:Track) ON (t.youtube_url)",
        ];

        for query_str in queries {
            let query = Query::new(query_str.to_string());
            let _ = self.graph.execute(query).await; // Ignore errors for existing constraints
        }

        Ok(())
    }

    // Artist operations
    pub async fn create_or_update_artist(&self, artist: &DatabaseArtist) -> Result<(), Box<dyn std::error::Error>> {
        let query = Query::new(
            "MERGE (a:Artist {id: $id})
             SET a.name = $name,
                 a.spotify_uri = $spotify_uri,
                 a.external_urls = $external_urls,
                 a.updated_at = datetime()".to_string()
        )
        .param("id", artist.id.clone())
        .param("name", artist.name.clone())
        .param("spotify_uri", artist.spotify_uri.clone())
        .param("external_urls", artist.external_urls.clone());

        self.graph.execute(query).await.map_err(|e| format!("Failed to create/update artist: {}", e))?;
        Ok(())
    }

    // Track operations
    pub async fn create_or_update_track(&self, track: &DatabaseTrack) -> Result<(), Box<dyn std::error::Error>> {
        let query = Query::new(
            "MERGE (t:Track {id: $id})
             SET t.name = $name,
                 t.spotify_uri = $spotify_uri,
                 t.duration_ms = $duration_ms,
                 t.explicit = $explicit,
                 t.popularity = $popularity,
                 t.preview_url = $preview_url,
                 t.external_urls = $external_urls,
                 t.youtube_url = $youtube_url,
                 t.isrc = $isrc,
                 t.updated_at = datetime()".to_string()
        )
        .param("id", track.id.clone())
        .param("name", track.name.clone())
        .param("spotify_uri", track.spotify_uri.clone())
        .param("duration_ms", track.duration_ms as i64)
        .param("explicit", track.explicit)
        .param("popularity", track.popularity as i64)
        .param("preview_url", track.preview_url.clone())
        .param("external_urls", track.external_urls.clone())
        .param("youtube_url", track.youtube_url.clone())
        .param("isrc", track.isrc.clone());

        self.graph.execute(query).await.map_err(|e| format!("Failed to create/update track: {}", e))?;
        Ok(())
    }

    // Playlist operations
    pub async fn create_or_update_playlist(&self, playlist: &DatabasePlaylist) -> Result<(), Box<dyn std::error::Error>> {
        let query = Query::new(
            "MERGE (p:Playlist {id: $id})
             SET p.name = $name,
                 p.description = $description,
                 p.spotify_uri = $spotify_uri,
                 p.owner_id = $owner_id,
                 p.owner_display_name = $owner_display_name,
                 p.public = $public,
                 p.collaborative = $collaborative,
                 p.snapshot_id = $snapshot_id,
                 p.total_tracks = $total_tracks,
                 p.updated_at = datetime()".to_string()
        )
        .param("id", playlist.id.clone())
        .param("name", playlist.name.clone())
        .param("description", playlist.description.clone())
        .param("spotify_uri", playlist.spotify_uri.clone())
        .param("owner_id", playlist.owner_id.clone())
        .param("owner_display_name", playlist.owner_display_name.clone())
        .param("public", playlist.public)
        .param("collaborative", playlist.collaborative)
        .param("snapshot_id", playlist.snapshot_id.clone())
        .param("total_tracks", playlist.total_tracks as i64);

        self.graph.execute(query).await.map_err(|e| format!("Failed to create/update playlist: {}", e))?;
        Ok(())
    }

    // Album operations
    pub async fn create_or_update_album(&self, album: &DatabaseAlbum) -> Result<(), Box<dyn std::error::Error>> {
        let query = Query::new(
            "MERGE (al:Album {id: $id})
             SET al.name = $name,
                 al.spotify_uri = $spotify_uri,
                 al.album_type = $album_type,
                 al.release_date = $release_date,
                 al.total_tracks = $total_tracks,
                 al.updated_at = datetime()".to_string()
        )
        .param("id", album.id.clone())
        .param("name", album.name.clone())
        .param("spotify_uri", album.spotify_uri.clone())
        .param("album_type", album.album_type.clone())
        .param("release_date", album.release_date.clone())
        .param("total_tracks", album.total_tracks as i64);

        self.graph.execute(query).await.map_err(|e| format!("Failed to create/update album: {}", e))?;
        Ok(())
    }

    // Relationship operations
    pub async fn link_track_to_artist(&self, track_id: &str, artist_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let query = Query::new(
            "MATCH (t:Track {id: $track_id}), (a:Artist {id: $artist_id})
             MERGE (a)-[:PERFORMED]->(t)".to_string()
        )
        .param("track_id", track_id.to_string())
        .param("artist_id", artist_id.to_string());

        self.graph.execute(query).await.map_err(|e| format!("Failed to link track to artist: {}", e))?;
        Ok(())
    }

    pub async fn link_track_to_album(&self, track_id: &str, album_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let query = Query::new(
            "MATCH (t:Track {id: $track_id}), (al:Album {id: $album_id})
             MERGE (al)-[:CONTAINS]->(t)".to_string()
        )
        .param("track_id", track_id.to_string())
        .param("album_id", album_id.to_string());

        self.graph.execute(query).await.map_err(|e| format!("Failed to link track to album: {}", e))?;
        Ok(())
    }

    pub async fn link_playlist_to_track(&self, playlist_id: &str, track_id: &str, position: i64) -> Result<(), Box<dyn std::error::Error>> {
        let query = Query::new(
            "MATCH (p:Playlist {id: $playlist_id}), (t:Track {id: $track_id})
             MERGE (p)-[r:INCLUDES]->(t)
             SET r.position = $position,
                 r.added_at = datetime()".to_string()
        )
        .param("playlist_id", playlist_id.to_string())
        .param("track_id", track_id.to_string())
        .param("position", position);

        self.graph.execute(query).await.map_err(|e| format!("Failed to link playlist to track: {}", e))?;
        Ok(())
    }

    pub async fn link_album_to_artist(&self, album_id: &str, artist_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let query = Query::new(
            "MATCH (al:Album {id: $album_id}), (a:Artist {id: $artist_id})
             MERGE (a)-[:RELEASED]->(al)".to_string()
        )
        .param("album_id", album_id.to_string())
        .param("artist_id", artist_id.to_string());

        self.graph.execute(query).await.map_err(|e| format!("Failed to link album to artist: {}", e))?;
        Ok(())
    }

    // Simple operations for now - returning empty results to avoid compilation errors
    pub async fn find_track_by_isrc(&self, _isrc: &str) -> Result<Option<DatabaseTrack>, Box<dyn std::error::Error>> {
        // Simplified for now
        Ok(None)
    }

    pub async fn find_tracks_without_youtube_url(&self, _limit: i64) -> Result<Vec<DatabaseTrack>, Box<dyn std::error::Error>> {
        // Simplified for now
        Ok(Vec::new())
    }

    pub async fn update_track_youtube_url(&self, track_id: &str, youtube_url: &str) -> Result<(), Box<dyn std::error::Error>> {
        let query = Query::new(
            "MATCH (t:Track {id: $track_id})
             SET t.youtube_url = $youtube_url,
                 t.converted_at = datetime()".to_string()
        )
        .param("track_id", track_id.to_string())
        .param("youtube_url", youtube_url.to_string());

        self.graph.execute(query).await.map_err(|e| format!("Failed to update track YouTube URL: {}", e))?;
        Ok(())
    }

    pub async fn get_playlist_tracks(&self, _playlist_id: &str) -> Result<Vec<(DatabaseTrack, i64)>, Box<dyn std::error::Error>> {
        // Simplified for now
        Ok(Vec::new())
    }

    pub async fn search_tracks_by_name(&self, _name: &str, _limit: i64) -> Result<Vec<DatabaseTrack>, Box<dyn std::error::Error>> {
        // Simplified for now
        Ok(Vec::new())
    }
}
