package config

import (
	"os"
)

type Config struct {
	Neo4j   Neo4jConfig
	Spotify SpotifyConfig
	YouTube YouTubeConfig
	Server  ServerConfig
}

type Neo4jConfig struct {
	URI      string
	Username string
	Password string
}

type SpotifyConfig struct {
	ClientID     string
	ClientSecret string
	RedirectURI  string
}

type YouTubeConfig struct {
	ClientID     string
	ClientSecret string
	RedirectURI  string
	APIKey       string
}

type ServerConfig struct {
	Port string
}

func Load() *Config {
	return &Config{
		Neo4j: Neo4jConfig{
			URI:      getEnv("NEO4J_URI", "bolt://localhost:7687"),
			Username: getEnv("NEO4J_USERNAME", "neo4j"),
			Password: getEnv("NEO4J_PASSWORD", "password"),
		},
		Spotify: SpotifyConfig{
			ClientID:     getEnv("SPOTIFY_CLIENT_ID", ""),
			ClientSecret: getEnv("SPOTIFY_CLIENT_SECRET", ""),
			RedirectURI:  getEnv("SPOTIFY_REDIRECT_URI", "http://localhost:8080/callback/spotify"),
		},
		YouTube: YouTubeConfig{
			ClientID:     getEnv("YOUTUBE_CLIENT_ID", ""),
			ClientSecret: getEnv("YOUTUBE_CLIENT_SECRET", ""),
			RedirectURI:  getEnv("YOUTUBE_REDIRECT_URI", "http://localhost:8080/callback/youtube"),
			APIKey:       getEnv("YOUTUBE_API_KEY", ""),
		},
		Server: ServerConfig{
			Port: getEnv("PORT", "8080"),
		},
	}
}

func getEnv(key, defaultValue string) string {
	if value := os.Getenv(key); value != "" {
		return value
	}
	return defaultValue
}
