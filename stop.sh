#!/bin/bash

# Stop all services
echo "Stopping all services..."
docker-compose down

echo "All services stopped."
echo ""
echo "To remove volumes as well (this will delete database data), run:"
echo "docker-compose down -v"
