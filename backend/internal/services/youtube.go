package services

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"net/url"

	"spotify-to-youtube-backend/internal/config"
	"spotify-to-youtube-backend/internal/models"

	"golang.org/x/oauth2"
	"golang.org/x/oauth2/google"
	"google.golang.org/api/youtube/v3"
)

type YouTubeService struct {
	config     config.YouTubeConfig
	oauthCfg   *oauth2.Config
	httpClient *http.Client
}

func NewYouTubeService(cfg config.YouTubeConfig) *YouTubeService {
	oauthCfg := &oauth2.Config{
		ClientID:     cfg.ClientID,
		ClientSecret: cfg.ClientSecret,
		RedirectURL:  cfg.RedirectURI,
		Scopes: []string{
			youtube.YoutubeScope,
			youtube.YoutubeReadonlyScope,
		},
		Endpoint: google.Endpoint,
	}

	return &YouTubeService{
		config:     cfg,
		oauthCfg:   oauthCfg,
		httpClient: &http.Client{},
	}
}

func (s *YouTubeService) GetAuthURL(state string) string {
	return s.oauthCfg.AuthCodeURL(state)
}

func (s *YouTubeService) ExchangeCode(ctx context.Context, code string) (*oauth2.Token, error) {
	return s.oauthCfg.Exchange(ctx, code)
}

func (s *YouTubeService) GetPlaylists(ctx context.Context, token *oauth2.Token) ([]models.Playlist, error) {
	client := s.oauthCfg.Client(ctx, token)
	service, err := youtube.New(client)
	if err != nil {
		return nil, fmt.Errorf("failed to create YouTube service: %w", err)
	}

	call := service.Playlists.List([]string{"snippet"}).Mine(true)
	response, err := call.Do()
	if err != nil {
		return nil, fmt.Errorf("failed to get playlists: %w", err)
	}

	playlists := make([]models.Playlist, len(response.Items))
	for i, item := range response.Items {
		playlists[i] = models.Playlist{
			ID:          item.Id,
			Name:        item.Snippet.Title,
			Description: item.Snippet.Description,
			Platform:    "youtube",
			ExternalID:  item.Id,
		}
	}

	return playlists, nil
}

func (s *YouTubeService) GetPlaylistTracks(ctx context.Context, token *oauth2.Token, playlistID string) ([]models.Song, error) {
	client := s.oauthCfg.Client(ctx, token)
	service, err := youtube.New(client)
	if err != nil {
		return nil, fmt.Errorf("failed to create YouTube service: %w", err)
	}

	call := service.PlaylistItems.List([]string{"snippet"}).PlaylistId(playlistID)
	response, err := call.Do()
	if err != nil {
		return nil, fmt.Errorf("failed to get playlist items: %w", err)
	}

	songs := make([]models.Song, len(response.Items))
	for i, item := range response.Items {
		videoID := item.Snippet.ResourceId.VideoId
		videoURL := fmt.Sprintf("https://www.youtube.com/watch?v=%s", videoID)

		songs[i] = models.Song{
			ID:         videoID,
			Title:      item.Snippet.Title,
			Artist:     item.Snippet.VideoOwnerChannelTitle,
			YouTubeID:  videoID,
			YouTubeURL: videoURL,
		}
	}

	return songs, nil
}

func (s *YouTubeService) SearchVideo(ctx context.Context, query string) (*models.Song, error) {
	searchURL := fmt.Sprintf(
		"https://www.googleapis.com/youtube/v3/search?part=snippet&q=%s&type=video&key=%s&maxResults=1",
		url.QueryEscape(query),
		s.config.APIKey,
	)

	resp, err := s.httpClient.Get(searchURL)
	if err != nil {
		return nil, fmt.Errorf("failed to search video: %w", err)
	}
	defer resp.Body.Close()

	var response struct {
		Items []struct {
			ID struct {
				VideoID string `json:"videoId"`
			} `json:"id"`
			Snippet struct {
				Title       string `json:"title"`
				ChannelTitle string `json:"channelTitle"`
			} `json:"snippet"`
		} `json:"items"`
	}

	if err := json.NewDecoder(resp.Body).Decode(&response); err != nil {
		return nil, fmt.Errorf("failed to decode response: %w", err)
	}

	if len(response.Items) == 0 {
		return nil, fmt.Errorf("no videos found for query: %s", query)
	}

	item := response.Items[0]
	videoURL := fmt.Sprintf("https://www.youtube.com/watch?v=%s", item.ID.VideoID)

	return &models.Song{
		ID:         item.ID.VideoID,
		Title:      item.Snippet.Title,
		Artist:     item.Snippet.ChannelTitle,
		YouTubeID:  item.ID.VideoID,
		YouTubeURL: videoURL,
	}, nil
}

func (s *YouTubeService) CreatePlaylist(ctx context.Context, token *oauth2.Token, name, description string) (*models.Playlist, error) {
	client := s.oauthCfg.Client(ctx, token)
	service, err := youtube.New(client)
	if err != nil {
		return nil, fmt.Errorf("failed to create YouTube service: %w", err)
	}

	playlist := &youtube.Playlist{
		Snippet: &youtube.PlaylistSnippet{
			Title:       name,
			Description: description,
		},
		Status: &youtube.PlaylistStatus{
			PrivacyStatus: "private",
		},
	}

	call := service.Playlists.Insert([]string{"snippet", "status"}, playlist)
	response, err := call.Do()
	if err != nil {
		return nil, fmt.Errorf("failed to create playlist: %w", err)
	}

	return &models.Playlist{
		ID:          response.Id,
		Name:        response.Snippet.Title,
		Description: response.Snippet.Description,
		Platform:    "youtube",
		ExternalID:  response.Id,
	}, nil
}

func (s *YouTubeService) AddVideoToPlaylist(ctx context.Context, token *oauth2.Token, playlistID, videoID string) error {
	client := s.oauthCfg.Client(ctx, token)
	service, err := youtube.New(client)
	if err != nil {
		return fmt.Errorf("failed to create YouTube service: %w", err)
	}

	playlistItem := &youtube.PlaylistItem{
		Snippet: &youtube.PlaylistItemSnippet{
			PlaylistId: playlistID,
			ResourceId: &youtube.ResourceId{
				Kind:    "youtube#video",
				VideoId: videoID,
			},
		},
	}

	call := service.PlaylistItems.Insert([]string{"snippet"}, playlistItem)
	_, err = call.Do()
	if err != nil {
		return fmt.Errorf("failed to add video to playlist: %w", err)
	}

	return nil
}
