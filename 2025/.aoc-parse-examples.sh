#!/bin/bash

# Advent of Code Example Parser
# Extracts example inputs and expected answers from task descriptions
#
# This is a best-effort parser. For complex cases, you can manually edit
# the generated test files.

set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

if [ $# -lt 1 ]; then
    echo "Usage: $0 <task-file> [part] [--input-only]"
    echo ""
    echo "Examples:"
    echo "  $0 zig/day-1/task.txt 1              # Parse Part 1"
    echo "  $0 zig/day-1/task.txt 2              # Parse Part 2"
    echo "  $0 zig/day-1/task.txt all            # Parse both (JSON)"
    echo "  $0 zig/day-1/task.txt 1 --input-only # Just extract input"
    exit 1
fi

TASK_FILE=$1
PART=${2:-"all"}
INPUT_ONLY=${3:-""}

if [ ! -f "$TASK_FILE" ]; then
    echo -e "${RED}Error: Task file not found: $TASK_FILE${NC}" >&2
    exit 1
fi

# Extract example input from content
# Looks for indented blocks after "For example:"
extract_example_input() {
    local content="$1"
    local example_input=""
    local in_example=0
    local collecting=0

    while IFS= read -r line; do
        # Start after "For example:" or "Here are the same example" or similar
        if echo "$line" | grep -qiE "(for example:|same example|example.*(lists?|input))"; then
            in_example=1
            collecting=1
            continue
        fi

        if [ $in_example -eq 1 ] && [ $collecting -eq 1 ]; then
            # Skip empty lines at start
            if [ -z "$example_input" ] && [ -z "$(echo "$line" | tr -d '[:space:]')" ]; then
                continue
            fi

            # If line starts with prose markers, stop
            if echo "$line" | grep -qE "^(Maybe|Within|In the|The |To find|Your|So,|For these|Here|This)"; then
                break
            fi

            # If we hit a separator or new section, stop
            if echo "$line" | grep -qE "^---|^$"; then
                if [ -n "$example_input" ]; then
                    break
                fi
                continue
            fi

            # Collect the line
            if [ -n "$example_input" ]; then
                example_input="${example_input}
${line}"
            else
                example_input="$line"
            fi
        fi
    done <<< "$content"

    # Clean up
    example_input=$(echo "$example_input" | sed 's/^[[:space:]]*//; s/[[:space:]]*$//')
    echo "$example_input"
}

# Extract answer from content
# Looks for common patterns: "total of X", "is X", etc.
extract_example_answer() {
    local content="$1"
    local answer=""

    # Look for common answer patterns
    # Priority order: most specific to least specific

    # Pattern: "total distance of 11!"
    answer=$(echo "$content" | grep -oE "total [a-z]+ of [0-9]+" | grep -oE "[0-9]+" | tail -1)
    if [ -n "$answer" ]; then
        echo "$answer"
        return
    fi

    # Pattern: "similarity score... is 31"
    answer=$(echo "$content" | grep -iE "score.{0,50}is [0-9]+" | grep -oE "is [0-9]+" | grep -oE "[0-9]+" | tail -1)
    if [ -n "$answer" ]; then
        echo "$answer"
        return
    fi

    # Pattern: "answer is X" or "result is X"
    answer=$(echo "$content" | grep -iE "(answer|result).{0,10}is [0-9]+" | grep -oE "[0-9]+" | tail -1)
    if [ -n "$answer" ]; then
        echo "$answer"
        return
    fi

    # Fallback: look for number followed by ! or period in prose
    answer=$(echo "$content" | grep -oE "[0-9]+[!.]" | grep -oE "[0-9]+" | tail -1)
    if [ -n "$answer" ]; then
        echo "$answer"
        return
    fi

    echo "UNKNOWN"
}

# Read the task file
TASK_CONTENT=$(<"$TASK_FILE")

# Split into Part 1 and Part 2
if echo "$TASK_CONTENT" | grep -q "Part Two"; then
    # Use awk to split at "--- Part Two ---"
    PART1_CONTENT=$(echo "$TASK_CONTENT" | awk '/--- Part Two ---/{exit} {print}')
    PART2_CONTENT=$(echo "$TASK_CONTENT" | awk '/--- Part Two ---/,0')
else
    PART1_CONTENT="$TASK_CONTENT"
    PART2_CONTENT=""
fi

# Create JSON output
create_json_example() {
    local input="$1"
    local answer="$2"

    if [ -z "$input" ]; then
        echo "null"
        return
    fi

    # Escape for JSON
    local escaped_input
    if command -v python3 &> /dev/null; then
        escaped_input=$(echo "$input" | python3 -c 'import sys, json; print(json.dumps(sys.stdin.read())[1:-1])')
    else
        escaped_input=$(echo "$input" | sed 's/\\/\\\\/g; s/"/\\"/g' | awk '{printf "%s\\n", $0}' | sed 's/\\n$//')
    fi

    echo "{\"input\":\"${escaped_input}\",\"answer\":\"${answer}\"}"
}

# Parse based on requested part
case $PART in
    1)
        echo -e "${BLUE}Parsing Part 1...${NC}" >&2
        input=$(extract_example_input "$PART1_CONTENT")
        if [ "$INPUT_ONLY" = "--input-only" ]; then
            echo "$input"
        else
            answer=$(extract_example_answer "$PART1_CONTENT")
            example=$(create_json_example "$input" "$answer")
            if [ "$example" != "null" ]; then
                echo "[$example]"
            else
                echo "[]"
            fi
        fi
        ;;
    2)
        if [ -z "$PART2_CONTENT" ]; then
            echo -e "${YELLOW}Part 2 not found${NC}" >&2
            echo "[]"
        else
            echo -e "${BLUE}Parsing Part 2...${NC}" >&2
            input=$(extract_example_input "$PART2_CONTENT")
            if [ "$INPUT_ONLY" = "--input-only" ]; then
                echo "$input"
            else
                answer=$(extract_example_answer "$PART2_CONTENT")
                example=$(create_json_example "$input" "$answer")
                if [ "$example" != "null" ]; then
                    echo "[$example]"
                else
                    echo "[]"
                fi
            fi
        fi
        ;;
    all)
        input1=$(extract_example_input "$PART1_CONTENT")
        answer1=$(extract_example_answer "$PART1_CONTENT")
        example1=$(create_json_example "$input1" "$answer1")

        if [ -z "$PART2_CONTENT" ]; then
            example2="null"
        else
            input2=$(extract_example_input "$PART2_CONTENT")
            answer2=$(extract_example_answer "$PART2_CONTENT")
            example2=$(create_json_example "$input2" "$answer2")
        fi

        echo "{"
        echo "  \"part1\": "
        if [ "$example1" != "null" ]; then
            echo "    [$example1]"
        else
            echo "    []"
        fi
        echo "  ,"
        echo "  \"part2\": "
        if [ "$example2" != "null" ]; then
            echo "    [$example2]"
        else
            echo "    []"
        fi
        echo "}"
        ;;
    *)
        echo -e "${RED}Error: Part must be 1, 2, or all${NC}" >&2
        exit 1
        ;;
esac
