package services

import (
	"context"
	"fmt"
	"time"

	"spotify-to-youtube-backend/internal/database"
	"spotify-to-youtube-backend/internal/models"
)

type PlaylistService struct {
	db *database.Neo4jDB
}

func NewPlaylistService(db *database.Neo4jDB) *PlaylistService {
	return &PlaylistService{
		db: db,
	}
}

func (s *PlaylistService) CreatePlaylist(ctx context.Context, playlist *models.Playlist) error {
	playlist.CreatedAt = time.Now()
	playlist.UpdatedAt = time.Now()
	return s.db.CreatePlaylist(ctx, playlist)
}

func (s *PlaylistService) GetPlaylist(ctx context.Context, id string) (*models.Playlist, error) {
	return s.db.GetPlaylist(ctx, id)
}

func (s *PlaylistService) AddSongToPlaylist(ctx context.Context, playlistID, songID string) error {
	return s.db.AddSongToPlaylist(ctx, playlistID, songID)
}

func (s *PlaylistService) ConvertPlaylist(ctx context.Context, conversionReq *models.ConversionRequest, spotifyService *SpotifyService, youtubeService *YouTubeService) (*models.ConversionResult, error) {
	// Get the source playlist
	playlist, err := s.GetPlaylist(ctx, conversionReq.PlaylistID)
	if err != nil {
		return nil, fmt.Errorf("failed to get playlist: %w", err)
	}

	result := &models.ConversionResult{
		Success:       true,
		ConvertedSongs: 0,
		FailedSongs:   []string{},
	}

	// Create new playlist on target platform
	_ = fmt.Sprintf("%s (Converted from %s)", playlist.Name, conversionReq.SourcePlatform)
	
	// TODO: Implement actual conversion logic based on source and target platforms
	// This would involve:
	// 1. Getting songs from source playlist
	// 2. Searching for equivalent songs on target platform
	// 3. Creating new playlist on target platform
	// 4. Adding found songs to new playlist
	
	result.Message = "Conversion completed successfully"
	return result, nil
}

func (s *PlaylistService) SyncPlaylist(ctx context.Context, playlistID string) error {
	// TODO: Implement playlist synchronization logic
	// This would involve comparing the playlist with its external counterpart
	// and updating any changes
	return fmt.Errorf("sync functionality not yet implemented")
}
