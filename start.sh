#!/bin/bash

# Spotify to YouTube Converter - Startup Script

set -e

echo "🎵 Starting Spotify to YouTube Converter..."

# Check if .env file exists
if [ ! -f ".env" ]; then
    echo "❌ .env file not found!"
    echo "📋 Please copy .env.example to .env and fill in your API credentials:"
    echo "   cp .env.example .env"
    echo "   Then edit .env with your Spotify and YouTube API keys"
    exit 1
fi

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo "❌ Docker is not running. Please start Docker first."
    exit 1
fi

# Check if docker-compose is available
if ! command -v docker-compose > /dev/null 2>&1; then
    if ! command -v docker > /dev/null 2>&1 || ! docker compose version > /dev/null 2>&1; then
        echo "❌ Neither docker-compose nor 'docker compose' is available."
        echo "Please install Docker Compose or use a newer version of Docker."
        exit 1
    fi
    COMPOSE_CMD="docker compose"
else
    COMPOSE_CMD="docker-compose"
fi

echo "🔧 Building and starting services..."

# Build and start all services
$COMPOSE_CMD up --build -d

echo "⏳ Waiting for services to be ready..."

# Wait for services to be healthy
echo "🔍 Checking Neo4j..."
until $COMPOSE_CMD exec neo4j cypher-shell -u neo4j -p password123 "RETURN 1" > /dev/null 2>&1; do
    echo "   Neo4j not ready yet, waiting..."
    sleep 5
done
echo "✅ Neo4j is ready!"

echo "🔍 Checking Backend..."
until curl -f http://localhost:3000/health > /dev/null 2>&1; do
    echo "   Backend not ready yet, waiting..."
    sleep 5
done
echo "✅ Backend is ready!"

echo ""
echo "🎉 All services are running!"
echo ""
echo "📊 Neo4j Browser:  http://localhost:7474"
echo "🖥️  Frontend:       http://localhost"
echo "🔧 Backend API:    http://localhost:3000"
echo ""
echo "📱 To stop all services, run:"
echo "   $COMPOSE_CMD down"
echo ""
echo "📋 To view logs, run:"
echo "   $COMPOSE_CMD logs -f [service_name]"
echo ""
echo "Happy converting! 🎵 ➡️ 📺"
