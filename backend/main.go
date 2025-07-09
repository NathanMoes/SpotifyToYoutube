package main

import (
	"log"
	"os"

	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
	"github.com/joho/godotenv"

	"spotify-to-youtube-backend/internal/config"
	"spotify-to-youtube-backend/internal/database"
	"spotify-to-youtube-backend/internal/handlers"
	"spotify-to-youtube-backend/internal/middleware"
	"spotify-to-youtube-backend/internal/services"
)

func main() {
	// Load environment variables
	if err := godotenv.Load(); err != nil {
		log.Println("No .env file found")
	}

	// Initialize configuration
	cfg := config.Load()

	// Initialize database
	db, err := database.NewNeo4jDB(cfg.Neo4j)
	if err != nil {
		log.Fatalf("Failed to connect to Neo4j: %v", err)
	}
	defer db.Close()

	// Initialize services
	spotifyService := services.NewSpotifyService(cfg.Spotify)
	youtubeService := services.NewYouTubeService(cfg.YouTube)
	playlistService := services.NewPlaylistService(db)

	// Initialize handlers
	authHandler := handlers.NewAuthHandler(spotifyService, youtubeService)
	playlistHandler := handlers.NewPlaylistHandler(playlistService, spotifyService, youtubeService)
	songHandler := handlers.NewSongHandler(db)

	// Initialize Gin router
	r := gin.Default()

	// Configure CORS
	config := cors.DefaultConfig()
	config.AllowOrigins = []string{"http://localhost:3000"}
	config.AllowMethods = []string{"GET", "POST", "PUT", "DELETE", "OPTIONS"}
	config.AllowHeaders = []string{"*"}
	config.AllowCredentials = true
	r.Use(cors.New(config))

	// Middleware
	r.Use(middleware.Logger())
	r.Use(middleware.ErrorHandler())

	// Routes
	setupRoutes(r, authHandler, playlistHandler, songHandler)

	// Start server
	port := os.Getenv("PORT")
	if port == "" {
		port = "8080"
	}

	log.Printf("Server starting on port %s", port)
	if err := r.Run(":" + port); err != nil {
		log.Fatalf("Failed to start server: %v", err)
	}
}

func setupRoutes(r *gin.Engine, authHandler *handlers.AuthHandler, playlistHandler *handlers.PlaylistHandler, songHandler *handlers.SongHandler) {
	api := r.Group("/api/v1")

	// Health check
	api.GET("/health", func(c *gin.Context) {
		c.JSON(200, gin.H{"status": "ok"})
	})

	// Authentication routes
	auth := api.Group("/auth")
	{
		auth.GET("/spotify", authHandler.SpotifyAuth)
		auth.GET("/youtube", authHandler.YouTubeAuth)
		auth.GET("/callback/spotify", authHandler.SpotifyCallback)
		auth.GET("/callback/youtube", authHandler.YouTubeCallback)
	}

	// Playlist routes
	playlists := api.Group("/playlists")
	{
		playlists.POST("/", playlistHandler.CreatePlaylist)
		playlists.GET("/", playlistHandler.GetPlaylists)
		playlists.GET("/:id", playlistHandler.GetPlaylist)
		playlists.PUT("/:id", playlistHandler.UpdatePlaylist)
		playlists.DELETE("/:id", playlistHandler.DeletePlaylist)
		playlists.POST("/:id/convert", playlistHandler.ConvertPlaylist)
		playlists.POST("/:id/sync", playlistHandler.SyncPlaylist)
	}

	// Song routes
	songs := api.Group("/songs")
	{
		songs.POST("/", songHandler.CreateSong)
		songs.GET("/", songHandler.GetSongs)
		songs.GET("/:id", songHandler.GetSong)
		songs.PUT("/:id", songHandler.UpdateSong)
		songs.DELETE("/:id", songHandler.DeleteSong)
		songs.POST("/search", songHandler.SearchSongs)
	}
}
