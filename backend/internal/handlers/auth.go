package handlers

import (
	"net/http"

	"spotify-to-youtube-backend/internal/services"

	"github.com/gin-gonic/gin"
)

type AuthHandler struct {
	spotifyService *services.SpotifyService
	youtubeService *services.YouTubeService
}

func NewAuthHandler(spotifyService *services.SpotifyService, youtubeService *services.YouTubeService) *AuthHandler {
	return &AuthHandler{
		spotifyService: spotifyService,
		youtubeService: youtubeService,
	}
}

func (h *AuthHandler) SpotifyAuth(c *gin.Context) {
	state := "spotify-auth-state" // In production, use a random state
	authURL := h.spotifyService.GetAuthURL(state)
	
	c.JSON(http.StatusOK, gin.H{
		"auth_url": authURL,
	})
}

func (h *AuthHandler) YouTubeAuth(c *gin.Context) {
	state := "youtube-auth-state" // In production, use a random state
	authURL := h.youtubeService.GetAuthURL(state)
	
	c.JSON(http.StatusOK, gin.H{
		"auth_url": authURL,
	})
}

func (h *AuthHandler) SpotifyCallback(c *gin.Context) {
	code := c.Query("code")
	if code == "" {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Missing authorization code"})
		return
	}

	token, err := h.spotifyService.ExchangeCode(c.Request.Context(), code)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to exchange code for token"})
		return
	}

	// In a real application, you would store this token securely
	// and associate it with the user's session
	c.JSON(http.StatusOK, gin.H{
		"message":      "Successfully authenticated with Spotify",
		"access_token": token.AccessToken,
		"expires_in":   token.Expiry,
	})
}

func (h *AuthHandler) YouTubeCallback(c *gin.Context) {
	code := c.Query("code")
	if code == "" {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Missing authorization code"})
		return
	}

	token, err := h.youtubeService.ExchangeCode(c.Request.Context(), code)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to exchange code for token"})
		return
	}

	// In a real application, you would store this token securely
	// and associate it with the user's session
	c.JSON(http.StatusOK, gin.H{
		"message":      "Successfully authenticated with YouTube",
		"access_token": token.AccessToken,
		"expires_in":   token.Expiry,
	})
}
