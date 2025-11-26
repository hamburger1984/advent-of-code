#!/bin/bash

# Go TDD Watch Mode
# Automatically runs tests when files change

echo "🔍 Go TDD Watch Mode"
echo "Watching for changes... (Ctrl+C to stop)"
echo ""

# Check if fswatch is available
if command -v fswatch &> /dev/null; then
    # Use fswatch for file watching (macOS/BSD)
    fswatch -o . | while read f; do
        clear
        echo "🔄 Running tests..."
        echo ""
        go test -v ./...
        echo ""
        echo "✨ Waiting for changes..."
    done
elif command -v inotifywait &> /dev/null; then
    # Use inotify for Linux
    while true; do
        clear
        echo "🔄 Running tests..."
        echo ""
        go test -v ./...
        echo ""
        echo "✨ Waiting for changes..."
        inotifywait -qre modify,create,delete .
    done
else
    # Fallback: simple polling
    echo "⚠️  fswatch not found. Using simple polling (slower)."
    echo "   Install fswatch for better performance: brew install fswatch"
    echo ""

    while true; do
        clear
        echo "🔄 Running tests..."
        echo ""
        go test -v ./...
        echo ""
        echo "✨ Waiting for changes..."
        sleep 2
    done
fi
