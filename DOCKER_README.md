# Docker Setup for Spotify to YouTube

This setup provides a complete Docker environment that runs all components of the Spotify to YouTube application in containers.

## Services Included

1. **Neo4j Database** - Graph database for storing playlist and song data
2. **Backend API** - Go-based REST API server
3. **Frontend Web App** - React-based web application

## Quick Start

### Prerequisites
- Docker and Docker Compose installed
- At least 2GB of available RAM
- Ports 3000, 7474, 7687, and 8080 available

### Start All Services
```bash
./start.sh
```

Or manually:
```bash
docker-compose up --build -d
```

### Stop All Services
```bash
./stop.sh
```

Or manually:
```bash
docker-compose down
```

## Access Points

After starting the services, you can access:

- **Frontend Application**: http://localhost:3000
- **Backend API**: http://localhost:8080
- **Neo4j Browser**: http://localhost:7474
  - Username: `neo4j`
  - Password: `s3cretP@ssw0rd`

## Development Mode

For development with hot reloading:

```bash
docker-compose -f docker-compose.yml -f docker-compose.dev.yml up --build
```

This will:
- Mount source code as volumes for live editing
- Enable hot reloading for React frontend
- Enable debug mode for Go backend

## Environment Variables

The following environment variables are configured:

### Backend
- `NEO4J_URI`: bolt://neo4j:7687
- `NEO4J_USERNAME`: neo4j
- `NEO4J_PASSWORD`: s3cretP@ssw0rd
- `PORT`: 8080

### Frontend
- `REACT_APP_API_URL`: http://localhost:8080

## Health Checks

The setup includes health checks for:
- Neo4j database connection
- Backend API health endpoint
- Proper service startup order

## Volumes

The setup uses named volumes for persistent data:
- `neo4j_data`: Database data
- `neo4j_logs`: Database logs
- `neo4j_import`: Import directory
- `neo4j_plugins`: Plugin directory

## Troubleshooting

### View Logs
```bash
docker-compose logs -f
```

### View specific service logs
```bash
docker-compose logs -f [service-name]
# Example: docker-compose logs -f backend
```

### Rebuild specific service
```bash
docker-compose up --build [service-name]
# Example: docker-compose up --build frontend
```

### Reset database
```bash
docker-compose down -v
docker-compose up --build
```

### Check service status
```bash
docker-compose ps
```

## Configuration Files

- `docker-compose.yml`: Main composition file
- `docker-compose.dev.yml`: Development overrides
- `backend/Dockerfile`: Backend container definition
- `frontend/Dockerfile`: Frontend container definition

## Network

All services communicate through the `spotify-youtube-network` bridge network, allowing them to reach each other by service name.
