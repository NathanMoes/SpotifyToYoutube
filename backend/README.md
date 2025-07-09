# Spotify to YouTube Backend

A Go backend service for converting playlists between Spotify and YouTube using Neo4j as the database.

## Features

- OAuth2 authentication with Spotify and YouTube
- Playlist conversion between platforms
- Song matching and search capabilities
- Neo4j graph database for storing relationships
- RESTful API endpoints
- Docker support

## Prerequisites

- Go 1.21 or higher
- Docker and Docker Compose
- Spotify API credentials
- YouTube API credentials

## Setup

1. **Clone the repository and navigate to the backend directory**

2. **Set up environment variables**
   ```bash
   cp .env.example .env
   ```
   
   Edit `.env` with your API credentials:
   - `SPOTIFY_CLIENT_ID` and `SPOTIFY_CLIENT_SECRET` from [Spotify Developer Dashboard](https://developer.spotify.com/)
   - `YOUTUBE_CLIENT_ID`, `YOUTUBE_CLIENT_SECRET`, and `YOUTUBE_API_KEY` from [Google Cloud Console](https://console.cloud.google.com/)

3. **Start Neo4j database**
   ```bash
   docker-compose up -d neo4j
   ```

4. **Install Go dependencies**
   ```bash
   go mod tidy
   ```

5. **Run the application**
   ```bash
   go run main.go
   ```

## API Endpoints

### Authentication
- `GET /api/v1/auth/spotify` - Get Spotify OAuth URL
- `GET /api/v1/auth/youtube` - Get YouTube OAuth URL
- `GET /api/v1/auth/callback/spotify` - Spotify OAuth callback
- `GET /api/v1/auth/callback/youtube` - YouTube OAuth callback

### Playlists
- `POST /api/v1/playlists` - Create a new playlist
- `GET /api/v1/playlists` - Get all playlists
- `GET /api/v1/playlists/:id` - Get playlist by ID
- `PUT /api/v1/playlists/:id` - Update playlist
- `DELETE /api/v1/playlists/:id` - Delete playlist
- `POST /api/v1/playlists/:id/convert` - Convert playlist to another platform
- `POST /api/v1/playlists/:id/sync` - Sync playlist with external platform

### Songs
- `POST /api/v1/songs` - Create a new song
- `GET /api/v1/songs` - Get all songs
- `GET /api/v1/songs/:id` - Get song by ID
- `PUT /api/v1/songs/:id` - Update song
- `DELETE /api/v1/songs/:id` - Delete song
- `POST /api/v1/songs/search` - Search songs

## Development

### Project Structure
```
backend/
├── internal/
│   ├── config/          # Configuration management
│   ├── database/        # Neo4j database operations
│   ├── handlers/        # HTTP request handlers
│   ├── middleware/      # HTTP middleware
│   ├── models/          # Data models
│   └── services/        # Business logic services
├── main.go             # Application entry point
├── Dockerfile          # Docker configuration
├── go.mod              # Go module file
└── .env.example        # Environment variables template
```

### Running Tests
```bash
go test ./...
```

### Building Docker Image
```bash
docker build -t spotify-youtube-backend .
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Commit your changes
6. Push to the branch
7. Create a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.
