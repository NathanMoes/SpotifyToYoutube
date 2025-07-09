package services

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"net/url"
	"strings"

	"spotify-to-youtube-backend/internal/config"
	"spotify-to-youtube-backend/internal/models"

	"golang.org/x/oauth2"
)

type SpotifyService struct {
	config    config.SpotifyConfig
	oauthCfg  *oauth2.Config
	httpClient *http.Client
}

type SpotifyPlaylist struct {
	ID          string `json:"id"`
	Name        string `json:"name"`
	Description string `json:"description"`
	Tracks      struct {
		Items []SpotifyTrack `json:"items"`
	} `json:"tracks"`
}

type SpotifyTrack struct {
	Track struct {
		ID       string `json:"id"`
		Name     string `json:"name"`
		Duration int    `json:"duration_ms"`
		Album    struct {
			Name string `json:"name"`
		} `json:"album"`
		Artists []struct {
			Name string `json:"name"`
		} `json:"artists"`
		ExternalURLs struct {
			Spotify string `json:"spotify"`
		} `json:"external_urls"`
	} `json:"track"`
}

func NewSpotifyService(cfg config.SpotifyConfig) *SpotifyService {
	oauthCfg := &oauth2.Config{
		ClientID:     cfg.ClientID,
		ClientSecret: cfg.ClientSecret,
		RedirectURL:  cfg.RedirectURI,
		Scopes: []string{
			"playlist-read-private",
			"playlist-read-collaborative",
			"playlist-modify-public",
			"playlist-modify-private",
		},
		Endpoint: oauth2.Endpoint{
			AuthURL:  "https://accounts.spotify.com/authorize",
			TokenURL: "https://accounts.spotify.com/api/token",
		},
	}

	return &SpotifyService{
		config:     cfg,
		oauthCfg:   oauthCfg,
		httpClient: &http.Client{},
	}
}

func (s *SpotifyService) GetAuthURL(state string) string {
	return s.oauthCfg.AuthCodeURL(state)
}

func (s *SpotifyService) ExchangeCode(ctx context.Context, code string) (*oauth2.Token, error) {
	return s.oauthCfg.Exchange(ctx, code)
}

func (s *SpotifyService) GetPlaylists(ctx context.Context, token *oauth2.Token) ([]models.Playlist, error) {
	client := s.oauthCfg.Client(ctx, token)
	
	resp, err := client.Get("https://api.spotify.com/v1/me/playlists")
	if err != nil {
		return nil, fmt.Errorf("failed to get playlists: %w", err)
	}
	defer resp.Body.Close()

	var response struct {
		Items []SpotifyPlaylist `json:"items"`
	}

	if err := json.NewDecoder(resp.Body).Decode(&response); err != nil {
		return nil, fmt.Errorf("failed to decode response: %w", err)
	}

	playlists := make([]models.Playlist, len(response.Items))
	for i, item := range response.Items {
		playlists[i] = models.Playlist{
			ID:          item.ID,
			Name:        item.Name,
			Description: item.Description,
			Platform:    "spotify",
			ExternalID:  item.ID,
		}
	}

	return playlists, nil
}

func (s *SpotifyService) GetPlaylistTracks(ctx context.Context, token *oauth2.Token, playlistID string) ([]models.Song, error) {
	client := s.oauthCfg.Client(ctx, token)
	
	url := fmt.Sprintf("https://api.spotify.com/v1/playlists/%s/tracks", playlistID)
	resp, err := client.Get(url)
	if err != nil {
		return nil, fmt.Errorf("failed to get playlist tracks: %w", err)
	}
	defer resp.Body.Close()

	var response struct {
		Items []SpotifyTrack `json:"items"`
	}

	if err := json.NewDecoder(resp.Body).Decode(&response); err != nil {
		return nil, fmt.Errorf("failed to decode response: %w", err)
	}

	songs := make([]models.Song, len(response.Items))
	for i, item := range response.Items {
		track := item.Track
		artist := ""
		if len(track.Artists) > 0 {
			artist = track.Artists[0].Name
		}

		songs[i] = models.Song{
			ID:         track.ID,
			Title:      track.Name,
			Artist:     artist,
			Album:      track.Album.Name,
			Duration:   track.Duration / 1000, // Convert from ms to seconds
			SpotifyID:  track.ID,
			SpotifyURL: track.ExternalURLs.Spotify,
		}
	}

	return songs, nil
}

func (s *SpotifyService) SearchTrack(ctx context.Context, token *oauth2.Token, query string) (*models.Song, error) {
	client := s.oauthCfg.Client(ctx, token)
	
	searchURL := fmt.Sprintf("https://api.spotify.com/v1/search?q=%s&type=track&limit=1", 
		url.QueryEscape(query))
	
	resp, err := client.Get(searchURL)
	if err != nil {
		return nil, fmt.Errorf("failed to search track: %w", err)
	}
	defer resp.Body.Close()

	var response struct {
		Tracks struct {
			Items []struct {
				ID       string `json:"id"`
				Name     string `json:"name"`
				Duration int    `json:"duration_ms"`
				Album    struct {
					Name string `json:"name"`
				} `json:"album"`
				Artists []struct {
					Name string `json:"name"`
				} `json:"artists"`
				ExternalURLs struct {
					Spotify string `json:"spotify"`
				} `json:"external_urls"`
			} `json:"items"`
		} `json:"tracks"`
	}

	if err := json.NewDecoder(resp.Body).Decode(&response); err != nil {
		return nil, fmt.Errorf("failed to decode response: %w", err)
	}

	if len(response.Tracks.Items) == 0 {
		return nil, fmt.Errorf("no tracks found for query: %s", query)
	}

	track := response.Tracks.Items[0]
	artist := ""
	if len(track.Artists) > 0 {
		artist = track.Artists[0].Name
	}

	return &models.Song{
		ID:         track.ID,
		Title:      track.Name,
		Artist:     artist,
		Album:      track.Album.Name,
		Duration:   track.Duration / 1000,
		SpotifyID:  track.ID,
		SpotifyURL: track.ExternalURLs.Spotify,
	}, nil
}

func (s *SpotifyService) CreatePlaylist(ctx context.Context, token *oauth2.Token, name, description string) (*models.Playlist, error) {
	client := s.oauthCfg.Client(ctx, token)
	
	// First get user ID
	userResp, err := client.Get("https://api.spotify.com/v1/me")
	if err != nil {
		return nil, fmt.Errorf("failed to get user info: %w", err)
	}
	defer userResp.Body.Close()

	var user struct {
		ID string `json:"id"`
	}
	if err := json.NewDecoder(userResp.Body).Decode(&user); err != nil {
		return nil, fmt.Errorf("failed to decode user response: %w", err)
	}

	// Create playlist
	createURL := fmt.Sprintf("https://api.spotify.com/v1/users/%s/playlists", user.ID)
	payload := map[string]interface{}{
		"name":        name,
		"description": description,
		"public":      false,
	}
	
	payloadBytes, _ := json.Marshal(payload)
	req, err := http.NewRequestWithContext(ctx, "POST", createURL, strings.NewReader(string(payloadBytes)))
	if err != nil {
		return nil, fmt.Errorf("failed to create request: %w", err)
	}
	
	req.Header.Set("Content-Type", "application/json")
	
	resp, err := client.Do(req)
	if err != nil {
		return nil, fmt.Errorf("failed to create playlist: %w", err)
	}
	defer resp.Body.Close()

	var playlist SpotifyPlaylist
	if err := json.NewDecoder(resp.Body).Decode(&playlist); err != nil {
		return nil, fmt.Errorf("failed to decode playlist response: %w", err)
	}

	return &models.Playlist{
		ID:          playlist.ID,
		Name:        playlist.Name,
		Description: playlist.Description,
		Platform:    "spotify",
		ExternalID:  playlist.ID,
		UserID:      user.ID,
	}, nil
}
