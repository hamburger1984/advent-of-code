#!/bin/bash

# C# TDD Watch Mode
# Uses dotnet watch test (built-in)

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

echo "🔍 C# Watch Mode for $DAY_DIR"
echo ""

cd "$DAY_DIR"

# C# doesn't have built-in tests in the template yet
# For now, just run the program on changes
echo "⚠️  Test framework not yet configured for C#"
echo "Running program on file changes..."
echo ""

dotnet watch run
