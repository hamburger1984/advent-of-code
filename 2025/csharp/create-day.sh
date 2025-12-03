#!/bin/bash

FETCH_SCRIPT="../.aoc-fetch.sh"
CACHE_DIR="${HOME}/.aoc-cache/2025"

if [ -z "$1" ]; then
    echo "Usage: ./create-day.sh <day-number> [--fetch]"
    echo "Example: ./create-day.sh 1"
    echo "Example: ./create-day.sh 1 --fetch  # Auto-fetch input and task"
    exit 1
fi

DAY=$1
FETCH_DATA=${2:-""}
DAY_DIR="day-${DAY}"

if [ -d "$DAY_DIR" ]; then
    echo "Error: $DAY_DIR already exists"
    exit 1
fi

echo "Creating $DAY_DIR..."
cp -r template "$DAY_DIR"

# Rename project file
mv "$DAY_DIR/Day.csproj" "$DAY_DIR/Day${DAY}.csproj"

# Add to solution
dotnet sln add "$DAY_DIR/Day${DAY}.csproj"

# Fetch input and task if requested
if [ "$FETCH_DATA" = "--fetch" ]; then
    # Check if already cached (from another language)
    INPUT_CACHE="${CACHE_DIR}/day${DAY}_input.txt"
    TASK_CACHE="${CACHE_DIR}/day${DAY}_task_part1.html"

    if [ -f "$INPUT_CACHE" ]; then
        echo "✓ Using cached input (shared across all languages)"
        cat "$INPUT_CACHE" > "$DAY_DIR/input.txt"
    elif [ -f "$FETCH_SCRIPT" ]; then
        echo "Fetching input from Advent of Code..."
        if INPUT=$("$FETCH_SCRIPT" "$DAY" input 2>/dev/null); then
            echo "$INPUT" > "$DAY_DIR/input.txt"
            echo "✓ Input saved to $DAY_DIR/input.txt"
        else
            echo "⚠ Failed to fetch input: $INPUT"
        fi
    else
        echo "⚠ Fetch script not found at $FETCH_SCRIPT"
    fi

    if [ -f "$TASK_CACHE" ]; then
        echo "✓ Using cached task (shared across all languages)"
        # Use the fetch script to properly format the task
        "$FETCH_SCRIPT" "$DAY" task > "$DAY_DIR/task.txt" 2>/dev/null || \
        (sed 's/<[^>]*>//g' "$TASK_CACHE" | \
        sed 's/&lt;/</g; s/&gt;/>/g; s/&amp;/\&/g; s/&quot;/"/g; s/&#39;/'"'"'/g' | \
        sed 's/^[[:space:]]*//; s/[[:space:]]*$//' | \
        grep -v '^$' > "$DAY_DIR/task.txt" || true)
    elif [ -f "$FETCH_SCRIPT" ]; then
        echo "Fetching task from Advent of Code..."
        if TASK=$("$FETCH_SCRIPT" "$DAY" task 2>/dev/null); then
            echo "$TASK" > "$DAY_DIR/task.txt"
            echo "✓ Task saved to $DAY_DIR/task.txt"
        else
            echo "⚠ Failed to fetch task"
        fi
    else
        echo "⚠ Fetch script not found at $FETCH_SCRIPT"
    fi

    # Generate tests from examples if task was fetched
    if [ -f "$DAY_DIR/task.txt" ]; then
        PARSE_SCRIPT="../.aoc-parse-examples.sh"
        GEN_SCRIPT="../.aoc-generate-tests.sh"

        if [ -f "$PARSE_SCRIPT" ] && [ -f "$GEN_SCRIPT" ]; then
            echo "Generating tests from examples..."
            EXAMPLES=$("$PARSE_SCRIPT" "$DAY_DIR/task.txt" all 2>/dev/null)
            if [ $? -eq 0 ] && [ -n "$EXAMPLES" ]; then
                "$GEN_SCRIPT" csharp "$DAY_DIR" "$EXAMPLES" 2>/dev/null && echo "✓ Tests generated with example data"
            fi
        fi
    fi
fi

echo ""
echo "Done! $DAY_DIR created and added to solution"
[ -z "$FETCH_DATA" ] && echo "  Edit $DAY_DIR/input.txt and $DAY_DIR/task.txt"
echo ""
echo "To run:"
echo "  cd $DAY_DIR && dotnet run"
echo ""
echo "To run with release optimizations:"
echo "  cd $DAY_DIR && dotnet run -c Release"
