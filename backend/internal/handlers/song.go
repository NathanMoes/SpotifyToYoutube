package handlers

import (
	"net/http"
	"strconv"

	"spotify-to-youtube-backend/internal/database"
	"spotify-to-youtube-backend/internal/models"

	"github.com/gin-gonic/gin"
)

type SongHandler struct {
	db *database.Neo4jDB
}

func NewSongHandler(db *database.Neo4jDB) *SongHandler {
	return &SongHandler{
		db: db,
	}
}

func (h *SongHandler) CreateSong(c *gin.Context) {
	var song models.Song
	if err := c.ShouldBindJSON(&song); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	if err := h.db.CreateSong(c.Request.Context(), &song); err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to create song"})
		return
	}

	c.JSON(http.StatusCreated, song)
}

func (h *SongHandler) GetSongs(c *gin.Context) {
	// TODO: Implement get all songs with pagination
	c.JSON(http.StatusOK, gin.H{"songs": []models.Song{}})
}

func (h *SongHandler) GetSong(c *gin.Context) {
	id := c.Param("id")
	
	song, err := h.db.GetSong(c.Request.Context(), id)
	if err != nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "Song not found"})
		return
	}

	c.JSON(http.StatusOK, song)
}

func (h *SongHandler) UpdateSong(c *gin.Context) {
	// TODO: Implement update song
	c.JSON(http.StatusOK, gin.H{"message": "Update song not yet implemented"})
}

func (h *SongHandler) DeleteSong(c *gin.Context) {
	// TODO: Implement delete song
	c.JSON(http.StatusOK, gin.H{"message": "Delete song not yet implemented"})
}

func (h *SongHandler) SearchSongs(c *gin.Context) {
	query := c.Query("q")
	if query == "" {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Search query is required"})
		return
	}

	limitStr := c.DefaultQuery("limit", "10")
	limit, err := strconv.Atoi(limitStr)
	if err != nil {
		limit = 10
	}

	songs, err := h.db.SearchSongs(c.Request.Context(), query, limit)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to search songs"})
		return
	}

	c.JSON(http.StatusOK, gin.H{"songs": songs})
}
