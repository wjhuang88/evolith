#!/bin/bash

# Development startup script for Evolith

set -e

echo "🚀 Starting Evolith Development Environment..."

# Check if .env exists, if not copy from .env.example
if [ ! -f .env ]; then
    echo "📝 Creating .env from .env.example..."
    cp .env.example .env
fi

# Load environment variables
export $(grep -v '^#' .env | xargs)

# Start Docker services (optional)
read -p "Start Docker services (PostgreSQL, Redis, MinIO)? [y/N] " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "🐳 Starting Docker services..."
    docker-compose up -d postgres redis minio
    echo "⏳ Waiting for services to be ready..."
    sleep 5
fi

# Start backend
echo "🦀 Starting Rust backend..."
cd backend
cargo run &
BACKEND_PID=$!
cd ..

# Wait for backend to start
echo "⏳ Waiting for backend to start..."
sleep 3

# Start frontend
echo "⚡ Starting Next.js frontend..."
cd frontend
npm run dev &
FRONTEND_PID=$!
cd ..

echo ""
echo "✅ Development environment started!"
echo ""
echo "📍 Services:"
echo "   - Frontend: http://localhost:3000"
echo "   - Backend:  http://localhost:8080"
echo "   - API Docs: http://localhost:8080/docs (when implemented)"
echo ""
echo "Press Ctrl+C to stop all services..."

# Trap Ctrl+C to kill both processes
trap "echo 'Stopping services...'; kill $BACKEND_PID $FRONTEND_PID 2>/dev/null; exit 0" INT TERM

# Wait for both processes
wait
