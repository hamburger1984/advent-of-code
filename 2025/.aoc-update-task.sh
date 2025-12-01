#!/bin/bash

# Helper script to update task.txt with Part 2 after solving Part 1
# Usage: ./.aoc-update-task.sh <day> <language>

FETCH_SCRIPT="$(dirname "$0")/.aoc-fetch.sh"
CACHE_DIR="${HOME}/.aoc-cache/2025"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

if [ $# -lt 2 ]; then
    echo "Usage: $0 <day> <language>"
    echo ""
    echo "Examples:"
    echo "  $0 5 zig      # Update zig/day-5/task.txt with Part 2"
    echo "  $0 5 csharp   # Update csharp/day-5/task.txt with Part 2"
    echo "  $0 5 go       # Update go/day-5/task.txt with Part 2"
    echo ""
    echo "Run this after you've correctly solved Part 1 and Part 2 is available"
    exit 1
fi

DAY=$1
LANG=$2
DAY_DIR="${LANG}/day-${DAY}"

if [ ! -d "$DAY_DIR" ]; then
    echo -e "${RED}Error: Directory $DAY_DIR not found${NC}"
    exit 1
fi

echo -e "${BLUE}Fetching Part 2 task for day ${DAY}...${NC}"

# Check cache first
TASK2_CACHE="${CACHE_DIR}/day${DAY}_task_part2.html"

if [ -f "$FETCH_SCRIPT" ]; then
    # Use fetch script for proper formatting (removes HTML headers/footers, formats Part Two)
    if [ -f "$TASK2_CACHE" ]; then
        echo -e "${GREEN}✓ Using cached Part 2 task${NC}"
    fi
    if TASK=$("$FETCH_SCRIPT" "$DAY" task2 2>/dev/null); then
        echo "$TASK" > "$DAY_DIR/task.txt"
        echo -e "${GREEN}✓ Updated $DAY_DIR/task.txt with Part 2${NC}"
    else
        echo -e "${YELLOW}⚠ Could not fetch Part 2. Have you solved Part 1 yet?${NC}"
        echo ""
        echo "Part 2 only becomes available after you submit the correct answer for Part 1."
        exit 1
    fi
else
    echo -e "${RED}Error: Fetch script not found at $FETCH_SCRIPT${NC}"
    exit 1
fi
