#!/usr/bin/env bash
set -euo pipefail

if [ $# -ne 1 ]; then
    echo "Usage: $0 <day-directory>"
    echo "Example: $0 day-1"
    exit 1
fi

DAY_DIR=$1

if [ ! -d "$DAY_DIR" ]; then
    echo "Error: Directory $DAY_DIR does not exist"
    exit 1
fi

cd "$DAY_DIR"

echo "🔍 Watching for changes in $DAY_DIR..."
echo "Press Ctrl+C to stop"
echo ""

# Check if fswatch is available
if command -v fswatch &> /dev/null; then
    # Use fswatch for efficient file watching
    fswatch -o src/ | while read f; do
        clear
        echo "🔄 Running tests..."
        echo ""
        ./gradlew test --quiet || true
        echo ""
        echo "---"
        echo "Waiting for changes..."
    done
else
    echo "⚠️  fswatch not found. Using polling mode (install fswatch for better performance)"
    echo "   macOS: brew install fswatch"
    echo "   Linux: apt-get install fswatch / yum install fswatch"
    echo ""

    # Fallback to polling
    LAST_HASH=""
    while true; do
        # Calculate hash of all source files
        CURRENT_HASH=$(find src/ -type f -exec md5 -q {} \; 2>/dev/null | md5 -q || echo "")

        if [ "$CURRENT_HASH" != "$LAST_HASH" ] && [ -n "$LAST_HASH" ]; then
            clear
            echo "🔄 Running tests..."
            echo ""
            ./gradlew test --quiet || true
            echo ""
            echo "---"
            echo "Waiting for changes..."
        fi

        LAST_HASH="$CURRENT_HASH"
        sleep 2
    done
fi
