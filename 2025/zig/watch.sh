#!/bin/bash

# Zig TDD Watch Mode
# Automatically runs tests when files change

if [ -z "$1" ]; then
    echo "Usage: ./watch.sh <day-number>"
    echo "Example: ./watch.sh 1"
    exit 1
fi

DAY=$1
DAY_DIR="day-${DAY}"

if [ ! -d "$DAY_DIR" ]; then
    echo "Error: $DAY_DIR not found"
    exit 1
fi

echo "🔍 Zig TDD Watch Mode for $DAY_DIR"
echo "Watching for changes... (Ctrl+C to stop)"
echo ""

# Check if fswatch is available
if command -v fswatch &> /dev/null; then
    # Use fswatch for file watching (macOS/BSD)
    fswatch -o "$DAY_DIR/src" | while read f; do
        clear
        echo "🔄 Running tests for $DAY_DIR..."
        echo ""
        zig build test-${DAY_DIR}
        echo ""
        echo "✨ Waiting for changes..."
    done
elif command -v inotifywait &> /dev/null; then
    # Use inotify for Linux
    while true; do
        clear
        echo "🔄 Running tests for $DAY_DIR..."
        echo ""
        zig build test-${DAY_DIR}
        echo ""
        echo "✨ Waiting for changes..."
        inotifywait -qre modify,create,delete "$DAY_DIR/src"
    done
else
    # Fallback: simple polling
    echo "⚠️  fswatch not found. Using simple polling (slower)."
    echo "   Install fswatch for better performance: brew install fswatch"
    echo ""

    while true; do
        clear
        echo "🔄 Running tests for $DAY_DIR..."
        echo ""
        zig build test-${DAY_DIR}
        echo ""
        echo "✨ Waiting for changes..."
        sleep 2
    done
fi
