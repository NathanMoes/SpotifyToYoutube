package utils

import (
	"crypto/rand"
	"encoding/hex"
	"fmt"
	"strings"
	"time"
)

// GenerateID generates a unique ID
func GenerateID() string {
	bytes := make([]byte, 16)
	rand.Read(bytes)
	return hex.EncodeToString(bytes)
}

// GenerateState generates a random state string for OAuth
func GenerateState() string {
	bytes := make([]byte, 32)
	rand.Read(bytes)
	return hex.EncodeToString(bytes)
}

// NormalizeSearchQuery normalizes a search query for better matching
func NormalizeSearchQuery(query string) string {
	// Remove extra whitespace
	query = strings.TrimSpace(query)
	
	// Convert to lowercase
	query = strings.ToLower(query)
	
	// Remove common words that might interfere with search
	commonWords := []string{"official", "video", "music", "lyric", "lyrics", "hd", "hq"}
	for _, word := range commonWords {
		query = strings.ReplaceAll(query, word, "")
	}
	
	// Remove extra whitespace again
	query = strings.TrimSpace(query)
	
	return query
}

// FormatDuration formats duration in seconds to a readable string
func FormatDuration(seconds int) string {
	duration := time.Duration(seconds) * time.Second
	hours := int(duration.Hours())
	minutes := int(duration.Minutes()) % 60
	secs := int(duration.Seconds()) % 60
	
	if hours > 0 {
		return fmt.Sprintf("%d:%02d:%02d", hours, minutes, secs)
	}
	return fmt.Sprintf("%d:%02d", minutes, secs)
}

// BuildSpotifySearchQuery builds a search query for Spotify API
func BuildSpotifySearchQuery(title, artist string) string {
	return fmt.Sprintf("track:\"%s\" artist:\"%s\"", title, artist)
}

// BuildYouTubeSearchQuery builds a search query for YouTube API
func BuildYouTubeSearchQuery(title, artist string) string {
	return fmt.Sprintf("%s %s", title, artist)
}
