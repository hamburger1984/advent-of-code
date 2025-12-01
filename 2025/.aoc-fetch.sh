#!/bin/bash

# Advent of Code Input/Task Fetcher
# Respects AoC automation guidelines:
# - Rate limited (3 seconds between requests)
# - Caches all responses
# - Requires User-Agent with contact info
# - Uses session token for authentication
# - Handles Part 1 and Part 2 task fetching separately

set -e

YEAR=${AOC_YEAR:-2025}
SESSION_FILE="${HOME}/.aoc-session"
CACHE_DIR="${HOME}/.aoc-cache/${YEAR}"
RATE_LIMIT_FILE="${CACHE_DIR}/.last-request"
MIN_REQUEST_INTERVAL=3  # 3 seconds between requests per AoC guidelines

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

usage() {
    echo "Usage: $0 <day> [input|task|task2|both]"
    echo ""
    echo "Examples:"
    echo "  $0 1          # Fetch input and Part 1 task"
    echo "  $0 1 input    # Fetch only input"
    echo "  $0 1 task     # Fetch only Part 1 task"
    echo "  $0 1 task2    # Fetch Part 2 task (after solving Part 1)"
    echo "  $0 1 both     # Fetch input and Part 1 task"
    echo ""
    echo "Note: Part 2 task only available after Part 1 is correctly solved"
    echo ""
    echo "Setup:"
    echo "  1. Log in to https://adventofcode.com using GitHub"
    echo "  2. Open browser DevTools (F12) → Application/Storage → Cookies"
    echo "  3. Copy the 'session' cookie value"
    echo "  4. Save it: echo 'your-session-token' > ~/.aoc-session"
    echo "  5. Secure it: chmod 600 ~/.aoc-session"
    exit 1
}

check_session() {
    if [ ! -f "$SESSION_FILE" ]; then
        echo -e "${RED}Error: Session file not found at $SESSION_FILE${NC}"
        echo ""
        echo "Setup instructions:"
        echo "  1. Log in to https://adventofcode.com using GitHub"
        echo "  2. Open browser DevTools (F12) → Application/Storage → Cookies"
        echo "  3. Copy the 'session' cookie value"
        echo "  4. Run: echo 'your-session-token' > ~/.aoc-session"
        echo "  5. Run: chmod 600 ~/.aoc-session"
        exit 1
    fi

    # Check file permissions
    if [ "$(stat -f '%A' "$SESSION_FILE" 2>/dev/null || stat -c '%a' "$SESSION_FILE" 2>/dev/null)" != "600" ]; then
        echo -e "${YELLOW}Warning: Session file has weak permissions. Securing...${NC}"
        chmod 600 "$SESSION_FILE"
    fi
}

rate_limit() {
    mkdir -p "$CACHE_DIR"

    if [ -f "$RATE_LIMIT_FILE" ]; then
        local last_request=$(cat "$RATE_LIMIT_FILE")
        local now=$(date +%s)
        local elapsed=$((now - last_request))

        if [ $elapsed -lt $MIN_REQUEST_INTERVAL ]; then
            local wait_time=$((MIN_REQUEST_INTERVAL - elapsed))
            echo -e "${YELLOW}Rate limiting: waiting ${wait_time}s...${NC}" >&2
            sleep $wait_time
        fi
    fi

    date +%s > "$RATE_LIMIT_FILE"
}

fetch_input() {
    local day=$1
    local cache_file="${CACHE_DIR}/day${day}_input.txt"

    if [ -f "$cache_file" ]; then
        echo -e "${GREEN}Using cached input for day ${day}${NC}" >&2
        cat "$cache_file"
        return 0
    fi

    echo -e "${YELLOW}Fetching input for day ${day} from adventofcode.com...${NC}" >&2
    rate_limit

    local session=$(cat "$SESSION_FILE")
    local url="https://adventofcode.com/${YEAR}/day/${day}/input"

    local response=$(curl -s -f \
        -H "Cookie: session=${session}" \
        -H "User-Agent: github.com/hamburger1984/advent-of-code via curl (contact: hamburger1984@gmail.com)" \
        "$url")

    if [ $? -eq 0 ]; then
        mkdir -p "$CACHE_DIR"
        echo "$response" > "$cache_file"
        echo -e "${GREEN}Input cached successfully${NC}" >&2
        echo "$response"
    else
        echo -e "${RED}Error: Failed to fetch input. Check your session token.${NC}" >&2
        return 1
    fi
}

fetch_task() {
    local day=$1
    local part=${2:-1}  # Default to part 1
    local cache_file="${CACHE_DIR}/day${day}_task_part${part}.html"

    if [ -f "$cache_file" ]; then
        echo -e "${GREEN}Using cached Part ${part} task for day ${day}${NC}" >&2
        cat "$cache_file"
        return 0
    fi

    echo -e "${YELLOW}Fetching Part ${part} task for day ${day} from adventofcode.com...${NC}" >&2
    rate_limit

    local session=$(cat "$SESSION_FILE")
    local url="https://adventofcode.com/${YEAR}/day/${day}"

    local response=$(curl -s -f \
        -H "Cookie: session=${session}" \
        -H "User-Agent: github.com/hamburger1984/advent-of-code via curl (contact: hamburger1984@gmail.com)" \
        "$url")

    if [ $? -eq 0 ]; then
        mkdir -p "$CACHE_DIR"

        # Check if Part 2 exists in the response
        if [ "$part" -eq 2 ]; then
            if echo "$response" | grep -q "Part Two"; then
                echo "$response" > "$cache_file"
                echo -e "${GREEN}Part 2 task cached successfully${NC}" >&2
                echo "$response"
            else
                echo -e "${BLUE}Part 2 not yet available. Solve Part 1 first!${NC}" >&2
                return 1
            fi
        else
            echo "$response" > "$cache_file"
            echo -e "${GREEN}Part 1 task cached successfully${NC}" >&2
            echo "$response"
        fi
    else
        echo -e "${RED}Error: Failed to fetch task. Check your session token.${NC}" >&2
        return 1
    fi
}

html_to_text() {
    # Simple HTML to text conversion
    # Removes HTML tags and decodes common entities
    # Also removes content before "--- Day" and footer content
    sed 's/<[^>]*>//g' | \
    sed 's/&lt;/</g; s/&gt;/>/g; s/&amp;/\&/g; s/&quot;/"/g; s/&#39;/'"'"'/g' | \
    sed 's/^[[:space:]]*//; s/[[:space:]]*$//' | \
    grep -v '^$' | \
    awk '
        /^--- Day [0-9]+:/ {
            flag=1
            # Split header from any text that follows
            match($0, /^--- Day [0-9]+: [^-]+ ---/)
            header = substr($0, 1, RLENGTH)
            rest = substr($0, RLENGTH + 1)
            print header
            print ""
            if (rest != "") print rest
            next
        }
        flag {
            # Stop at footer content
            if (/^You can also/ || /^\[Share/ || /^To play, please identify yourself/ || /^Although it hasn.*t changed/) {
                exit
            }
            # Separate Part Two with blank lines and add line break after header
            if (/^--- Part Two ---/) {
                # Split header from any text that follows
                match($0, /^--- Part Two ---/)
                header = substr($0, 1, RLENGTH)
                rest = substr($0, RLENGTH + 1)
                print ""
                print ""
                print header
                print ""
                if (rest != "") print rest
                next
            }
            print
        }
    ' || true
}

# Main script
if [ $# -lt 1 ]; then
    usage
fi

DAY=$1
MODE=${2:-both}

# Validate day
if ! [[ "$DAY" =~ ^[0-9]+$ ]] || [ "$DAY" -lt 1 ] || [ "$DAY" -gt 25 ]; then
    echo -e "${RED}Error: Day must be between 1 and 25${NC}"
    exit 1
fi

check_session

case $MODE in
    input)
        fetch_input $DAY
        ;;
    task)
        fetch_task $DAY 1 | html_to_text
        ;;
    task2)
        fetch_task $DAY 2 | html_to_text
        ;;
    both)
        INPUT=$(fetch_input $DAY)
        echo "$INPUT"
        echo "---TASK---"
        fetch_task $DAY 1 | html_to_text
        ;;
    *)
        usage
        ;;
esac
