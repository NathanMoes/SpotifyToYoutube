#!/bin/bash

# Spotify to YouTube Converter - Stop Script

set -e

echo "🛑 Stopping Spotify to YouTube Converter..."

# Check if docker-compose is available
if ! command -v docker-compose > /dev/null 2>&1; then
    if ! command -v docker > /dev/null 2>&1 || ! docker compose version > /dev/null 2>&1; then
        echo "❌ Neither docker-compose nor 'docker compose' is available."
        exit 1
    fi
    COMPOSE_CMD="docker compose"
else
    COMPOSE_CMD="docker-compose"
fi

# Stop all services
$COMPOSE_CMD down

echo "✅ All services stopped!"
echo ""
echo "💡 To remove all data volumes as well, run:"
echo "   $COMPOSE_CMD down -v"
