use crate::database::{DatabaseManager, DatabaseArtist, DatabaseTrack, DatabasePlaylist, DatabaseAlbum};
use crate::spotify::spotify_api::{SpotifyTrack, SpotifyArtist, SpotifyPlaylistTracks, SpotifyAlbum};
use serde_json::to_string;

pub struct MusicDataService {
    db: DatabaseManager,
}

impl MusicDataService {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let db = DatabaseManager::new().await?;
        Ok(MusicDataService { db })
    }

    /// Store a complete playlist with all its tracks and relationships
    pub async fn store_playlist_data(
        &self,
        playlist_id: &str,
        playlist_name: &str,
        playlist_description: Option<String>,
        playlist_uri: &str,
        owner_id: &str,
        owner_display_name: &str,
        public: bool,
        collaborative: bool,
        snapshot_id: &str,
        tracks_data: &SpotifyPlaylistTracks,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Create playlist
        let playlist = DatabasePlaylist {
            id: playlist_id.to_string(),
            name: playlist_name.to_string(),
            description: playlist_description,
            spotify_uri: playlist_uri.to_string(),
            owner_id: owner_id.to_string(),
            owner_display_name: owner_display_name.to_string(),
            public,
            collaborative,
            snapshot_id: snapshot_id.to_string(),
            total_tracks: tracks_data.total,
        };

        self.db.create_or_update_playlist(&playlist).await?;

        // Store each track and its relationships
        for (position, track_item) in tracks_data.items.iter().enumerate() {
            self.store_track_with_relationships(&track_item.track, Some(playlist_id), position as i64).await?;
        }

        Ok(())
    }

    /// Store a track with all its relationships (artists, album)
    pub async fn store_track_with_relationships(
        &self,
        spotify_track: &SpotifyTrack,
        playlist_id: Option<&str>,
        position: i64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Store album
        let album = DatabaseAlbum {
            id: spotify_track.album.id.clone(),
            name: spotify_track.album.name.clone(),
            spotify_uri: spotify_track.album.uri.clone(),
            album_type: spotify_track.album.album_type.clone(),
            release_date: spotify_track.album.release_date.clone(),
            total_tracks: spotify_track.album.total_tracks,
        };
        self.db.create_or_update_album(&album).await?;

        // Store artists
        for artist in &spotify_track.artists {
            let db_artist = DatabaseArtist {
                id: artist.id.clone(),
                name: artist.name.clone(),
                spotify_uri: artist.uri.clone(),
                external_urls: to_string(&artist.external_urls).unwrap_or_default(),
            };
            self.db.create_or_update_artist(&db_artist).await?;
            
            // Link artist to album
            self.db.link_album_to_artist(&album.id, &artist.id).await?;
        }

        // Store track
        let track = DatabaseTrack {
            id: spotify_track.id.clone(),
            name: spotify_track.name.clone(),
            spotify_uri: spotify_track.uri.clone(),
            duration_ms: spotify_track.duration_ms,
            explicit: spotify_track.explicit,
            popularity: spotify_track.popularity,
            preview_url: spotify_track.preview_url.clone(),
            external_urls: to_string(&spotify_track.external_urls).unwrap_or_default(),
            youtube_url: None, // Will be filled later during conversion
            isrc: spotify_track.external_ids.isrc.clone(),
        };
        self.db.create_or_update_track(&track).await?;

        // Link track to album
        self.db.link_track_to_album(&track.id, &album.id).await?;

        // Link track to artists
        for artist in &spotify_track.artists {
            self.db.link_track_to_artist(&track.id, &artist.id).await?;
        }

        // Link track to playlist if provided
        if let Some(playlist_id) = playlist_id {
            self.db.link_playlist_to_track(playlist_id, &track.id, position).await?;
        }

        Ok(())
    }

    /// Find tracks that need YouTube URL conversion
    pub async fn get_tracks_for_conversion(&self, limit: i64) -> Result<Vec<DatabaseTrack>, Box<dyn std::error::Error>> {
        self.db.find_tracks_without_youtube_url(limit).await
    }

    /// Update a track with its YouTube URL after conversion
    pub async fn update_track_youtube_url(&self, track_id: &str, youtube_url: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.db.update_track_youtube_url(track_id, youtube_url).await
    }

    /// Get all tracks in a playlist with their YouTube URLs (if available)
    pub async fn get_playlist_tracks_with_youtube(&self, playlist_id: &str) -> Result<Vec<(DatabaseTrack, i64)>, Box<dyn std::error::Error>> {
        self.db.get_playlist_tracks(playlist_id).await
    }

    /// Search for tracks by name
    pub async fn search_tracks(&self, name: &str, limit: i64) -> Result<Vec<DatabaseTrack>, Box<dyn std::error::Error>> {
        self.db.search_tracks_by_name(name, limit).await
    }

    /// Check if a track already exists by ISRC (to avoid duplicate conversions)
    pub async fn find_existing_track_by_isrc(&self, isrc: &str) -> Result<Option<DatabaseTrack>, Box<dyn std::error::Error>> {
        self.db.find_track_by_isrc(isrc).await
    }

    /// Get database statistics
    pub async fn get_stats(&self) -> Result<MusicDatabaseStats, Box<dyn std::error::Error>> {
        // This would involve multiple queries to get counts
        // For now, returning a placeholder
        Ok(MusicDatabaseStats {
            total_tracks: 0,
            total_artists: 0,
            total_playlists: 0,
            total_albums: 0,
            tracks_with_youtube_urls: 0,
        })
    }
}

#[derive(Debug, Clone)]
pub struct MusicDatabaseStats {
    pub total_tracks: u64,
    pub total_artists: u64,
    pub total_playlists: u64,
    pub total_albums: u64,
    pub tracks_with_youtube_urls: u64,
}

/// Conversion utilities
impl MusicDataService {
    /// Convert Spotify track to database track format
    pub fn spotify_track_to_db_track(spotify_track: &SpotifyTrack) -> DatabaseTrack {
        DatabaseTrack {
            id: spotify_track.id.clone(),
            name: spotify_track.name.clone(),
            spotify_uri: spotify_track.uri.clone(),
            duration_ms: spotify_track.duration_ms,
            explicit: spotify_track.explicit,
            popularity: spotify_track.popularity,
            preview_url: spotify_track.preview_url.clone(),
            external_urls: to_string(&spotify_track.external_urls).unwrap_or_default(),
            youtube_url: None,
            isrc: spotify_track.external_ids.isrc.clone(),
        }
    }

    /// Convert Spotify artist to database artist format
    pub fn spotify_artist_to_db_artist(spotify_artist: &SpotifyArtist) -> DatabaseArtist {
        DatabaseArtist {
            id: spotify_artist.id.clone(),
            name: spotify_artist.name.clone(),
            spotify_uri: spotify_artist.uri.clone(),
            external_urls: to_string(&spotify_artist.external_urls).unwrap_or_default(),
        }
    }

    /// Convert Spotify album to database album format
    pub fn spotify_album_to_db_album(spotify_album: &SpotifyAlbum) -> DatabaseAlbum {
        DatabaseAlbum {
            id: spotify_album.id.clone(),
            name: spotify_album.name.clone(),
            spotify_uri: spotify_album.uri.clone(),
            album_type: spotify_album.album_type.clone(),
            release_date: spotify_album.release_date.clone(),
            total_tracks: spotify_album.total_tracks,
        }
    }

    /// Add a manually entered track (not from Spotify)
    pub async fn add_manual_track(
        &self, 
        track_id: &str, 
        track_name: &str, 
        artist_name: &str
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Create a manual artist entry
        let artist_id = format!("manual_artist_{}", uuid::Uuid::new_v4());
        let artist = DatabaseArtist {
            id: artist_id.clone(),
            name: artist_name.to_string(),
            spotify_uri: "".to_string(), // Empty for manual entries
            external_urls: "{}".to_string(),
        };
        
        self.db.create_or_update_artist(&artist).await?;

        // Create a manual track entry
        let track = DatabaseTrack {
            id: track_id.to_string(),
            name: track_name.to_string(),
            spotify_uri: "".to_string(), // Empty for manual entries
            duration_ms: 0, // Unknown for manual entries
            explicit: false,
            popularity: 0, // Unknown for manual entries
            preview_url: None,
            external_urls: "{}".to_string(),
            youtube_url: None,
            isrc: None,
        };

        self.db.create_or_update_track(&track).await?;

        // Create the relationship between track and artist
        self.db.link_track_to_artist(track_id, &artist_id).await?;

        Ok(())
    }
}
