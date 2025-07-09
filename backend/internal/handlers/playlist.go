package handlers

import (
	"net/http"

	"spotify-to-youtube-backend/internal/models"
	"spotify-to-youtube-backend/internal/services"

	"github.com/gin-gonic/gin"
)

type PlaylistHandler struct {
	playlistService *services.PlaylistService
	spotifyService  *services.SpotifyService
	youtubeService  *services.YouTubeService
}

func NewPlaylistHandler(playlistService *services.PlaylistService, spotifyService *services.SpotifyService, youtubeService *services.YouTubeService) *PlaylistHandler {
	return &PlaylistHandler{
		playlistService: playlistService,
		spotifyService:  spotifyService,
		youtubeService:  youtubeService,
	}
}

func (h *PlaylistHandler) CreatePlaylist(c *gin.Context) {
	var playlist models.Playlist
	if err := c.ShouldBindJSON(&playlist); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	if err := h.playlistService.CreatePlaylist(c.Request.Context(), &playlist); err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to create playlist"})
		return
	}

	c.JSON(http.StatusCreated, playlist)
}

func (h *PlaylistHandler) GetPlaylists(c *gin.Context) {
	// TODO: Implement get all playlists
	c.JSON(http.StatusOK, gin.H{"playlists": []models.Playlist{}})
}

func (h *PlaylistHandler) GetPlaylist(c *gin.Context) {
	id := c.Param("id")
	
	playlist, err := h.playlistService.GetPlaylist(c.Request.Context(), id)
	if err != nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "Playlist not found"})
		return
	}

	c.JSON(http.StatusOK, playlist)
}

func (h *PlaylistHandler) UpdatePlaylist(c *gin.Context) {
	// TODO: Implement update playlist
	c.JSON(http.StatusOK, gin.H{"message": "Update playlist not yet implemented"})
}

func (h *PlaylistHandler) DeletePlaylist(c *gin.Context) {
	// TODO: Implement delete playlist
	c.JSON(http.StatusOK, gin.H{"message": "Delete playlist not yet implemented"})
}

func (h *PlaylistHandler) ConvertPlaylist(c *gin.Context) {
	id := c.Param("id")
	
	var conversionReq models.ConversionRequest
	if err := c.ShouldBindJSON(&conversionReq); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}
	
	conversionReq.PlaylistID = id
	
	result, err := h.playlistService.ConvertPlaylist(c.Request.Context(), &conversionReq, h.spotifyService, h.youtubeService)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to convert playlist"})
		return
	}

	c.JSON(http.StatusOK, result)
}

func (h *PlaylistHandler) SyncPlaylist(c *gin.Context) {
	id := c.Param("id")
	
	if err := h.playlistService.SyncPlaylist(c.Request.Context(), id); err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to sync playlist"})
		return
	}

	c.JSON(http.StatusOK, gin.H{"message": "Playlist synced successfully"})
}
