use neo4rs::{Graph, Query, Result};
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
    pub async fn new() -> Result<Self> {
        let uri = env::var("NEO4J_URI").unwrap_or_else(|_| "bolt://localhost:7687".to_string());
        let user = env::var("NEO4J_USER").unwrap_or_else(|_| "neo4j".to_string());
        let password = env::var("NEO4J_PASSWORD").unwrap_or_else(|_| "password".to_string());

        let graph = Graph::new(&uri, &user, &password).await?;
        
        let db_manager = DatabaseManager {
            graph: Arc::new(graph),
        };

        // Initialize database schema
        db_manager.initialize_schema().await?;

        Ok(db_manager)
    }

    async fn initialize_schema(&self) -> Result<()> {
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
    pub async fn create_or_update_artist(&self, artist: &DatabaseArtist) -> Result<()> {
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

        self.graph.execute(query).await?;
        Ok(())
    }

    // Track operations
    pub async fn create_or_update_track(&self, track: &DatabaseTrack) -> Result<()> {
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

        self.graph.execute(query).await?;
        Ok(())
    }

    // Playlist operations
    pub async fn create_or_update_playlist(&self, playlist: &DatabasePlaylist) -> Result<()> {
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

        self.graph.execute(query).await?;
        Ok(())
    }

    // Album operations
    pub async fn create_or_update_album(&self, album: &DatabaseAlbum) -> Result<()> {
        let query = Query::new(
            "MERGE (al:Album {id: $id})
             SET al.name = $name,
                 al.spotify_uri = $spotify_uri,
                 al.album_type = $album_type,
                 al.release_date = $release_date,
                 al.total_tracks = $total_tracks,
                 al.updated_at = datetime()"
        )
        .param("id", &album.id)
        .param("name", &album.name)
        .param("spotify_uri", &album.spotify_uri)
        .param("album_type", &album.album_type)
        .param("release_date", &album.release_date)
        .param("total_tracks", album.total_tracks as i64);

        self.graph.execute(query).await?;
        Ok(())
    }

    // Relationship operations
    pub async fn link_track_to_artist(&self, track_id: &str, artist_id: &str) -> Result<()> {
        let query = Query::new(
            "MATCH (t:Track {id: $track_id}), (a:Artist {id: $artist_id})
             MERGE (a)-[:PERFORMED]->(t)"
        )
        .param("track_id", track_id)
        .param("artist_id", artist_id);

        self.graph.execute(query).await?;
        Ok(())
    }

    pub async fn link_track_to_album(&self, track_id: &str, album_id: &str) -> Result<()> {
        let query = Query::new(
            "MATCH (t:Track {id: $track_id}), (al:Album {id: $album_id})
             MERGE (al)-[:CONTAINS]->(t)"
        )
        .param("track_id", track_id)
        .param("album_id", album_id);

        self.graph.execute(query).await?;
        Ok(())
    }

    pub async fn link_playlist_to_track(&self, playlist_id: &str, track_id: &str, position: i64) -> Result<()> {
        let query = Query::new(
            "MATCH (p:Playlist {id: $playlist_id}), (t:Track {id: $track_id})
             MERGE (p)-[r:INCLUDES]->(t)
             SET r.position = $position,
                 r.added_at = datetime()"
        )
        .param("playlist_id", playlist_id)
        .param("track_id", track_id)
        .param("position", position);

        self.graph.execute(query).await?;
        Ok(())
    }

    pub async fn link_album_to_artist(&self, album_id: &str, artist_id: &str) -> Result<()> {
        let query = Query::new(
            "MATCH (al:Album {id: $album_id}), (a:Artist {id: $artist_id})
             MERGE (a)-[:RELEASED]->(al)"
        )
        .param("album_id", album_id)
        .param("artist_id", artist_id);

        self.graph.execute(query).await?;
        Ok(())
    }

    // Search operations
    pub async fn find_track_by_isrc(&self, isrc: &str) -> Result<Option<DatabaseTrack>> {
        let query = Query::new("MATCH (t:Track {isrc: $isrc}) RETURN t".to_string())
            .param("isrc", isrc.to_string());

        let mut result = self.graph.execute(query).await?;
        
        if let Some(row) = result.next().await? {
            match row.get::<neo4rs::Node>("t") {
                Ok(node) => {
                    let track = DatabaseTrack {
                        id: node.get::<String>("id").unwrap_or_default(),
                        name: node.get::<String>("name").unwrap_or_default(),
                        spotify_uri: node.get::<String>("spotify_uri").unwrap_or_default(),
                        duration_ms: node.get::<i64>("duration_ms").unwrap_or(0) as u32,
                        explicit: node.get::<bool>("explicit").unwrap_or(false),
                        popularity: node.get::<i64>("popularity").unwrap_or(0) as u32,
                        preview_url: node.get::<Option<String>>("preview_url").unwrap_or(None),
                        external_urls: node.get::<String>("external_urls").unwrap_or_default(),
                        youtube_url: node.get::<Option<String>>("youtube_url").unwrap_or(None),
                        isrc: node.get::<Option<String>>("isrc").unwrap_or(None),
                    };
                    Ok(Some(track))
                },
                Err(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    pub async fn find_tracks_without_youtube_url(&self, limit: i64) -> Result<Vec<DatabaseTrack>> {
        let query = Query::new(
            "MATCH (t:Track)
             WHERE t.youtube_url IS NULL
             RETURN t
             LIMIT $limit"
        )
        .param("limit", limit);

        let mut result = self.graph.execute(query).await?;
        let mut tracks = Vec::new();

        while let Some(row) = result.next().await? {
            let node = row.get::<neo4rs::Node>("t")?;
            let track = DatabaseTrack {
                id: node.get::<String>("id")?,
                name: node.get::<String>("name")?,
                spotify_uri: node.get::<String>("spotify_uri")?,
                duration_ms: node.get::<i64>("duration_ms")? as u32,
                explicit: node.get::<bool>("explicit")?,
                popularity: node.get::<i64>("popularity")? as u32,
                preview_url: node.get::<Option<String>>("preview_url")?,
                external_urls: node.get::<String>("external_urls")?,
                youtube_url: node.get::<Option<String>>("youtube_url")?,
                isrc: node.get::<Option<String>>("isrc")?,
            };
            tracks.push(track);
        }

        Ok(tracks)
    }

    pub async fn update_track_youtube_url(&self, track_id: &str, youtube_url: &str) -> Result<()> {
        let query = Query::new(
            "MATCH (t:Track {id: $track_id})
             SET t.youtube_url = $youtube_url,
                 t.converted_at = datetime()"
        )
        .param("track_id", track_id)
        .param("youtube_url", youtube_url);

        self.graph.execute(query).await?;
        Ok(())
    }

    pub async fn get_playlist_tracks(&self, playlist_id: &str) -> Result<Vec<(DatabaseTrack, i64)>> {
        let query = Query::new(
            "MATCH (p:Playlist {id: $playlist_id})-[r:INCLUDES]->(t:Track)
             RETURN t, r.position as position
             ORDER BY r.position"
        )
        .param("playlist_id", playlist_id);

        let mut result = self.graph.execute(query).await?;
        let mut tracks = Vec::new();

        while let Some(row) = result.next().await? {
            let node = row.get::<neo4rs::Node>("t")?;
            let position = row.get::<i64>("position")?;
            
            let track = DatabaseTrack {
                id: node.get::<String>("id")?,
                name: node.get::<String>("name")?,
                spotify_uri: node.get::<String>("spotify_uri")?,
                duration_ms: node.get::<i64>("duration_ms")? as u32,
                explicit: node.get::<bool>("explicit")?,
                popularity: node.get::<i64>("popularity")? as u32,
                preview_url: node.get::<Option<String>>("preview_url")?,
                external_urls: node.get::<String>("external_urls")?,
                youtube_url: node.get::<Option<String>>("youtube_url")?,
                isrc: node.get::<Option<String>>("isrc")?,
            };
            tracks.push((track, position));
        }

        Ok(tracks)
    }

    pub async fn search_tracks_by_name(&self, name: &str, limit: i64) -> Result<Vec<DatabaseTrack>> {
        let query = Query::new(
            "MATCH (t:Track)
             WHERE toLower(t.name) CONTAINS toLower($name)
             RETURN t
             LIMIT $limit"
        )
        .param("name", name)
        .param("limit", limit);

        let mut result = self.graph.execute(query).await?;
        let mut tracks = Vec::new();

        while let Some(row) = result.next().await? {
            let node = row.get::<neo4rs::Node>("t")?;
            let track = DatabaseTrack {
                id: node.get::<String>("id")?,
                name: node.get::<String>("name")?,
                spotify_uri: node.get::<String>("spotify_uri")?,
                duration_ms: node.get::<i64>("duration_ms")? as u32,
                explicit: node.get::<bool>("explicit")?,
                popularity: node.get::<i64>("popularity")? as u32,
                preview_url: node.get::<Option<String>>("preview_url")?,
                external_urls: node.get::<String>("external_urls")?,
                youtube_url: node.get::<Option<String>>("youtube_url")?,
                isrc: node.get::<Option<String>>("isrc")?,
            };
            tracks.push(track);
        }

        Ok(tracks)
    }
}
