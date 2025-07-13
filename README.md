# Spotify to YouTube Converter

A full-stack application that converts Spotify playlists to YouTube playlists, built with Rust (backend), Yew (frontend), and Neo4j (database).

## 🚀 Quick Start with Docker

### Prerequisites
- Docker and Docker Compose installed on your system
- Spotify Developer Account (for API keys)
- YouTube Data API key

### 1. Setup Environment Variables

Copy the example environment file and fill in your API credentials:

```bash
cp .env.example .env
```

Edit `.env` file and add your credentials:

```bash
# Spotify API Credentials (get from https://developer.spotify.com/)
SPOTIFY_CLIENT_ID=your_spotify_client_id_here
SPOTIFY_CLIENT_SECRET=your_spotify_client_secret_here
SPOTIFY_REDIRECT_URI=http://localhost:3000/callback

# YouTube API Credentials (get from https://console.cloud.google.com/)
YOUTUBE_API_KEY=your_youtube_api_key_here
```

### 2. Start the Application

Run the startup script:

```bash
./start.sh
```

Or use Docker Compose directly:

```bash
docker-compose up --build -d
```

### 3. Access the Application

- **Frontend**: http://localhost (main application)
- **Backend API**: http://localhost:3000 (API endpoints)
- **Neo4j Browser**: http://localhost:7474 (database management)
  - Username: `neo4j`
  - Password: `password123`

### 4. Stop the Application

```bash
./stop.sh
```

Or use Docker Compose:

```bash
docker-compose down
```

## 🏗️ Architecture

The application consists of three main services:

1. **Frontend** (Yew + WebAssembly)
   - Served by Nginx on port 80
   - Built from Rust using wasm-pack
   - Proxy configuration for API calls to backend

2. **Backend** (Rust + Axum)
   - REST API server on port 3000
   - Handles Spotify and YouTube API integration
   - Manages authentication and data processing

3. **Database** (Neo4j)
   - Graph database for storing relationships between songs, artists, and playlists
   - APOC plugins enabled for advanced graph operations
   - Web interface on port 7474

## 🛠️ Development

### Manual Build (without Docker)

If you want to build and run the services manually:

#### Backend
```bash
cd backend
cargo run
```

#### Frontend
```bash
cd frontend
wasm-pack build --target web --out-dir pkg --no-typescript
# Serve the files using a local HTTP server
```

### Environment Variables

Make sure to set the environment variables in `.env` file:

- `YOUTUBE_API_KEY` - Your YouTube Data API key
- `SPOTIFY_CLIENT_ID` - Your Spotify app client ID
- `SPOTIFY_CLIENT_SECRET` - Your Spotify app client secret
- `SPOTIFY_REDIRECT_URI` - OAuth redirect URI (http://localhost:3000/callback)
- `OPENSSL_DIR` - SSL directory path (usually auto-detected)

### API Documentation

Once the backend is running, you can access API documentation at:
- Swagger UI: http://localhost:3000/swagger-ui/
- ReDoc: http://localhost:3000/redoc/

## 📋 Features

- Convert Spotify playlists to YouTube playlists
- Search and match songs across platforms
- Store song relationships in Neo4j graph database
- Modern web interface built with Rust and WebAssembly
- RESTful API for integration with other services

## 🔧 Troubleshooting

### Common Issues

1. **Services not starting**: Check that Docker is running and ports 80, 3000, 7474, and 7687 are available
2. **API authentication errors**: Verify your Spotify and YouTube API credentials in the `.env` file
3. **Database connection issues**: Wait for Neo4j to fully start up (check logs with `docker-compose logs neo4j`)

### Viewing Logs

```bash
# View all service logs
docker-compose logs -f

# View specific service logs
docker-compose logs -f backend
docker-compose logs -f frontend
docker-compose logs -f neo4j
```

### Rebuilding Services

If you make changes to the code:

```bash
docker-compose up --build
```

## 📄 License

This project is open source. Please check the license file for details.