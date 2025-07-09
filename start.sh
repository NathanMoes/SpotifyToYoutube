#!/bin/bash

# Start all services with Docker Compose
echo "Starting Spotify to YouTube application..."
echo "This will start Neo4j database, backend API, and frontend web app"

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo "Error: Docker is not running. Please start Docker first."
    exit 1
fi

# Build and start all services
docker-compose up --build -d

echo ""
echo "Services are starting up..."
echo "- Neo4j Database: http://localhost:7474 (username: neo4j, password: s3cretP@ssw0rd)"
echo "- Backend API: http://localhost:8080"
echo "- Frontend App: http://localhost:3000"
echo ""
echo "Use 'docker-compose logs -f' to view logs"
echo "Use 'docker-compose down' to stop all services"
echo "Use 'docker-compose down -v' to stop and remove volumes"
