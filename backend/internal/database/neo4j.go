package database

import (
	"context"
	"fmt"
	"time"

	"spotify-to-youtube-backend/internal/config"
	"spotify-to-youtube-backend/internal/models"

	"github.com/neo4j/neo4j-go-driver/v5/neo4j"
)

type Neo4jDB struct {
	driver neo4j.DriverWithContext
}

func NewNeo4jDB(cfg config.Neo4jConfig) (*Neo4jDB, error) {
	driver, err := neo4j.NewDriverWithContext(
		cfg.URI,
		neo4j.BasicAuth(cfg.Username, cfg.Password, ""),
	)
	if err != nil {
		return nil, fmt.Errorf("failed to create Neo4j driver: %w", err)
	}

	// Test the connection
	ctx := context.Background()
	if err := driver.VerifyConnectivity(ctx); err != nil {
		return nil, fmt.Errorf("failed to verify Neo4j connectivity: %w", err)
	}

	db := &Neo4jDB{driver: driver}
	
	// Initialize schema
	if err := db.initializeSchema(ctx); err != nil {
		return nil, fmt.Errorf("failed to initialize schema: %w", err)
	}

	return db, nil
}

func (db *Neo4jDB) Close() error {
	return db.driver.Close(context.Background())
}

func (db *Neo4jDB) initializeSchema(ctx context.Context) error {
	session := db.driver.NewSession(ctx, neo4j.SessionConfig{})
	defer session.Close(ctx)

	// Create constraints and indexes
	queries := []string{
		"CREATE CONSTRAINT song_id IF NOT EXISTS FOR (s:Song) REQUIRE s.id IS UNIQUE",
		"CREATE CONSTRAINT playlist_id IF NOT EXISTS FOR (p:Playlist) REQUIRE p.id IS UNIQUE",
		"CREATE CONSTRAINT user_id IF NOT EXISTS FOR (u:User) REQUIRE u.id IS UNIQUE",
		"CREATE INDEX song_spotify_id IF NOT EXISTS FOR (s:Song) ON (s.spotify_id)",
		"CREATE INDEX song_youtube_id IF NOT EXISTS FOR (s:Song) ON (s.youtube_id)",
		"CREATE INDEX song_title_artist IF NOT EXISTS FOR (s:Song) ON (s.title, s.artist)",
	}

	for _, query := range queries {
		if _, err := session.Run(ctx, query, nil); err != nil {
			return fmt.Errorf("failed to execute query '%s': %w", query, err)
		}
	}

	return nil
}

// Song operations
func (db *Neo4jDB) CreateSong(ctx context.Context, song *models.Song) error {
	session := db.driver.NewSession(ctx, neo4j.SessionConfig{})
	defer session.Close(ctx)

	query := `
		CREATE (s:Song {
			id: $id,
			title: $title,
			artist: $artist,
			album: $album,
			duration: $duration,
			spotify_id: $spotify_id,
			youtube_id: $youtube_id,
			spotify_url: $spotify_url,
			youtube_url: $youtube_url,
			created_at: datetime(),
			updated_at: datetime()
		})
		RETURN s
	`

	params := map[string]interface{}{
		"id":          song.ID,
		"title":       song.Title,
		"artist":      song.Artist,
		"album":       song.Album,
		"duration":    song.Duration,
		"spotify_id":  song.SpotifyID,
		"youtube_id":  song.YouTubeID,
		"spotify_url": song.SpotifyURL,
		"youtube_url": song.YouTubeURL,
	}

	_, err := session.Run(ctx, query, params)
	return err
}

func (db *Neo4jDB) GetSong(ctx context.Context, id string) (*models.Song, error) {
	session := db.driver.NewSession(ctx, neo4j.SessionConfig{})
	defer session.Close(ctx)

	query := `
		MATCH (s:Song {id: $id})
		RETURN s.id, s.title, s.artist, s.album, s.duration, 
			   s.spotify_id, s.youtube_id, s.spotify_url, s.youtube_url,
			   s.created_at, s.updated_at
	`

	result, err := session.Run(ctx, query, map[string]interface{}{"id": id})
	if err != nil {
		return nil, err
	}

	record, err := result.Single(ctx)
	if err != nil {
		return nil, err
	}

	song := &models.Song{
		ID:         record.Values[0].(string),
		Title:      record.Values[1].(string),
		Artist:     record.Values[2].(string),
		Album:      record.Values[3].(string),
		Duration:   int(record.Values[4].(int64)),
		SpotifyID:  getStringValue(record.Values[5]),
		YouTubeID:  getStringValue(record.Values[6]),
		SpotifyURL: getStringValue(record.Values[7]),
		YouTubeURL: getStringValue(record.Values[8]),
		CreatedAt:  record.Values[9].(time.Time),
		UpdatedAt:  record.Values[10].(time.Time),
	}

	return song, nil
}

func (db *Neo4jDB) SearchSongs(ctx context.Context, query string, limit int) ([]*models.Song, error) {
	session := db.driver.NewSession(ctx, neo4j.SessionConfig{})
	defer session.Close(ctx)

	cypher := `
		MATCH (s:Song)
		WHERE s.title CONTAINS $query OR s.artist CONTAINS $query
		RETURN s.id, s.title, s.artist, s.album, s.duration, 
			   s.spotify_id, s.youtube_id, s.spotify_url, s.youtube_url,
			   s.created_at, s.updated_at
		LIMIT $limit
	`

	result, err := session.Run(ctx, cypher, map[string]interface{}{
		"query": query,
		"limit": limit,
	})
	if err != nil {
		return nil, err
	}

	var songs []*models.Song
	for result.Next(ctx) {
		record := result.Record()
		song := &models.Song{
			ID:         record.Values[0].(string),
			Title:      record.Values[1].(string),
			Artist:     record.Values[2].(string),
			Album:      record.Values[3].(string),
			Duration:   int(record.Values[4].(int64)),
			SpotifyID:  getStringValue(record.Values[5]),
			YouTubeID:  getStringValue(record.Values[6]),
			SpotifyURL: getStringValue(record.Values[7]),
			YouTubeURL: getStringValue(record.Values[8]),
			CreatedAt:  record.Values[9].(time.Time),
			UpdatedAt:  record.Values[10].(time.Time),
		}
		songs = append(songs, song)
	}

	return songs, nil
}

// Playlist operations
func (db *Neo4jDB) CreatePlaylist(ctx context.Context, playlist *models.Playlist) error {
	session := db.driver.NewSession(ctx, neo4j.SessionConfig{})
	defer session.Close(ctx)

	query := `
		CREATE (p:Playlist {
			id: $id,
			name: $name,
			description: $description,
			user_id: $user_id,
			platform: $platform,
			external_id: $external_id,
			created_at: datetime(),
			updated_at: datetime()
		})
		RETURN p
	`

	params := map[string]interface{}{
		"id":          playlist.ID,
		"name":        playlist.Name,
		"description": playlist.Description,
		"user_id":     playlist.UserID,
		"platform":    playlist.Platform,
		"external_id": playlist.ExternalID,
	}

	_, err := session.Run(ctx, query, params)
	return err
}

func (db *Neo4jDB) GetPlaylist(ctx context.Context, id string) (*models.Playlist, error) {
	session := db.driver.NewSession(ctx, neo4j.SessionConfig{})
	defer session.Close(ctx)

	query := `
		MATCH (p:Playlist {id: $id})
		OPTIONAL MATCH (p)-[:CONTAINS]->(s:Song)
		RETURN p.id, p.name, p.description, p.user_id, p.platform, p.external_id,
			   p.created_at, p.updated_at,
			   collect(s) as songs
	`

	result, err := session.Run(ctx, query, map[string]interface{}{"id": id})
	if err != nil {
		return nil, err
	}

	record, err := result.Single(ctx)
	if err != nil {
		return nil, err
	}

	playlist := &models.Playlist{
		ID:          record.Values[0].(string),
		Name:        record.Values[1].(string),
		Description: record.Values[2].(string),
		UserID:      record.Values[3].(string),
		Platform:    record.Values[4].(string),
		ExternalID:  record.Values[5].(string),
		CreatedAt:   record.Values[6].(time.Time),
		UpdatedAt:   record.Values[7].(time.Time),
		Songs:       []models.Song{}, // TODO: Parse songs from record.Values[8]
	}

	return playlist, nil
}

func (db *Neo4jDB) AddSongToPlaylist(ctx context.Context, playlistID, songID string) error {
	session := db.driver.NewSession(ctx, neo4j.SessionConfig{})
	defer session.Close(ctx)

	query := `
		MATCH (p:Playlist {id: $playlist_id})
		MATCH (s:Song {id: $song_id})
		MERGE (p)-[:CONTAINS]->(s)
	`

	_, err := session.Run(ctx, query, map[string]interface{}{
		"playlist_id": playlistID,
		"song_id":     songID,
	})
	return err
}

func getStringValue(value interface{}) string {
	if value == nil {
		return ""
	}
	if str, ok := value.(string); ok {
		return str
	}
	return ""
}
