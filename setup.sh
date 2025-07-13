#!/bin/bash

# Spotify to YouTube Converter - Complete Setup Script

set -e

echo "🎵 Spotify to YouTube Converter Setup"
echo "======================================"

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check prerequisites
echo "🔍 Checking prerequisites..."

if ! command_exists docker; then
    echo "❌ Docker is not installed. Please install Docker first:"
    echo "   https://docs.docker.com/get-docker/"
    exit 1
fi

if ! command_exists docker-compose && ! (command_exists docker && docker compose version > /dev/null 2>&1); then
    echo "❌ Docker Compose is not available. Please install Docker Compose:"
    echo "   https://docs.docker.com/compose/install/"
    exit 1
fi

echo "✅ Docker and Docker Compose are available"

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo "❌ Docker is not running. Please start Docker first."
    exit 1
fi

echo "✅ Docker is running"

# Check if .env file exists
if [ ! -f ".env" ]; then
    echo ""
    echo "📋 Setting up environment file..."
    if [ -f ".env.example" ]; then
        cp .env.example .env
        echo "✅ Copied .env.example to .env"
        echo ""
        echo "⚠️  IMPORTANT: You need to edit .env file with your API credentials!"
        echo ""
        echo "Required credentials:"
        echo "1. Spotify API credentials from: https://developer.spotify.com/"
        echo "   - SPOTIFY_CLIENT_ID"
        echo "   - SPOTIFY_CLIENT_SECRET"
        echo ""
        echo "2. YouTube Data API key from: https://console.cloud.google.com/"
        echo "   - YOUTUBE_API_KEY"
        echo ""
        echo "Please edit .env file now and then run this script again."
        exit 1
    else
        echo "❌ .env.example file not found. Please create .env file manually."
        exit 1
    fi
fi

# Check if required environment variables are set
echo "🔍 Checking environment variables..."

if ! grep -q "^SPOTIFY_CLIENT_ID=.*[^=]" .env; then
    echo "❌ SPOTIFY_CLIENT_ID is not set in .env file"
    echo "Please edit .env and add your Spotify Client ID"
    exit 1
fi

if ! grep -q "^SPOTIFY_CLIENT_SECRET=.*[^=]" .env; then
    echo "❌ SPOTIFY_CLIENT_SECRET is not set in .env file"
    echo "Please edit .env and add your Spotify Client Secret"
    exit 1
fi

if ! grep -q "^YOUTUBE_API_KEY=.*[^=]" .env; then
    echo "❌ YOUTUBE_API_KEY is not set in .env file"
    echo "Please edit .env and add your YouTube API Key"
    exit 1
fi

echo "✅ Environment variables are configured"

# Determine compose command
if command_exists docker-compose; then
    COMPOSE_CMD="docker-compose"
else
    COMPOSE_CMD="docker compose"
fi

echo ""
echo "🔧 Building and starting services..."
echo "This may take a few minutes on first run..."

# Build and start services
$COMPOSE_CMD up --build -d

echo ""
echo "⏳ Waiting for services to be ready..."

# Wait for Neo4j
echo "🔍 Waiting for Neo4j to be ready..."
for i in {1..30}; do
    if $COMPOSE_CMD exec -T neo4j cypher-shell -u neo4j -p password123 "RETURN 1" > /dev/null 2>&1; then
        echo "✅ Neo4j is ready!"
        break
    fi
    if [ $i -eq 30 ]; then
        echo "❌ Neo4j failed to start within 5 minutes"
        echo "Check logs with: $COMPOSE_CMD logs neo4j"
        exit 1
    fi
    echo "   Attempt $i/30: Neo4j not ready yet, waiting..."
    sleep 10
done

# Wait for Backend
echo "🔍 Waiting for Backend to be ready..."
for i in {1..20}; do
    if curl -f http://localhost:3000/health > /dev/null 2>&1; then
        echo "✅ Backend is ready!"
        break
    fi
    if [ $i -eq 20 ]; then
        echo "❌ Backend failed to start within 5 minutes"
        echo "Check logs with: $COMPOSE_CMD logs backend"
        exit 1
    fi
    echo "   Attempt $i/20: Backend not ready yet, waiting..."
    sleep 15
done

echo ""
echo "🎉 Setup complete! All services are running!"
echo ""
echo "🌐 Access your application:"
echo "   Frontend:      http://localhost"
echo "   Backend API:   http://localhost:3000"
echo "   Neo4j Browser: http://localhost:7474"
echo "                  (username: neo4j, password: password123)"
echo ""
echo "📋 Useful commands:"
echo "   View logs:     $COMPOSE_CMD logs -f [service_name]"
echo "   Stop services: $COMPOSE_CMD down"
echo "   Restart:       $COMPOSE_CMD restart [service_name]"
echo ""
echo "📖 For more information, check the README.md file"
echo ""
echo "Happy converting! 🎵 ➡️ 📺"
