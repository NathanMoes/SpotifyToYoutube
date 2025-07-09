package models

import (
	"time"
)

type Song struct {
	ID          string    `json:"id"`
	Title       string    `json:"title"`
	Artist      string    `json:"artist"`
	Album       string    `json:"album"`
	Duration    int       `json:"duration"` // in seconds
	SpotifyID   string    `json:"spotify_id,omitempty"`
	YouTubeID   string    `json:"youtube_id,omitempty"`
	SpotifyURL  string    `json:"spotify_url,omitempty"`
	YouTubeURL  string    `json:"youtube_url,omitempty"`
	CreatedAt   time.Time `json:"created_at"`
	UpdatedAt   time.Time `json:"updated_at"`
}

type Playlist struct {
	ID          string    `json:"id"`
	Name        string    `json:"name"`
	Description string    `json:"description"`
	UserID      string    `json:"user_id"`
	Platform    string    `json:"platform"` // "spotify" or "youtube"
	ExternalID  string    `json:"external_id"`
	Songs       []Song    `json:"songs"`
	CreatedAt   time.Time `json:"created_at"`
	UpdatedAt   time.Time `json:"updated_at"`
}

type User struct {
	ID                string    `json:"id"`
	SpotifyID         string    `json:"spotify_id,omitempty"`
	YouTubeID         string    `json:"youtube_id,omitempty"`
	SpotifyToken      string    `json:"spotify_token,omitempty"`
	YouTubeToken      string    `json:"youtube_token,omitempty"`
	SpotifyRefresh    string    `json:"spotify_refresh,omitempty"`
	YouTubeRefresh    string    `json:"youtube_refresh,omitempty"`
	CreatedAt         time.Time `json:"created_at"`
	UpdatedAt         time.Time `json:"updated_at"`
}

type ConversionRequest struct {
	PlaylistID     string `json:"playlist_id"`
	SourcePlatform string `json:"source_platform"`
	TargetPlatform string `json:"target_platform"`
}

type ConversionResult struct {
	Success       bool     `json:"success"`
	ConvertedSongs int     `json:"converted_songs"`
	FailedSongs   []string `json:"failed_songs"`
	NewPlaylistID string   `json:"new_playlist_id"`
	Message       string   `json:"message"`
}
