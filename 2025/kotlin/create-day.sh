#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
YEAR=2025

if [ $# -ne 1 ]; then
    echo "Usage: $0 <day>"
    echo "Example: $0 1"
    exit 1
fi

DAY=$1
DAY_DIR="${SCRIPT_DIR}/day-${DAY}"
TEMPLATE_DIR="${SCRIPT_DIR}/template"

# Fetch task and input using shared script
"${SCRIPT_DIR}/../.aoc-fetch.sh" "$DAY"

# Create day directory if it doesn't exist
if [ -d "$DAY_DIR" ]; then
    echo "Day ${DAY} already exists. Skipping template copy."
else
    echo "Creating day-${DAY} from template..."
    cp -r "$TEMPLATE_DIR" "$DAY_DIR"

    # Update settings.gradle.kts with correct day number
    sed -i.bak "s/rootProject.name = \"day\"/rootProject.name = \"day-${DAY}\"/" "${DAY_DIR}/settings.gradle.kts"
    rm "${DAY_DIR}/settings.gradle.kts.bak"

    # Set up Gradle wrapper
    cd "$DAY_DIR"
    gradle wrapper --gradle-version 8.5
    cd "$SCRIPT_DIR"
fi

# Copy input and task
CACHE_DIR="${HOME}/.aoc-cache/${YEAR}"
cp "${CACHE_DIR}/day${DAY}_input.txt" "${DAY_DIR}/input.txt"
cp "${CACHE_DIR}/day${DAY}_task_part1.html" "${DAY_DIR}/task.txt"

# Parse examples and generate tests
EXAMPLES_JSON="${CACHE_DIR}/day${DAY}_examples.json"
"${SCRIPT_DIR}/../.aoc-parse-examples.sh" "$DAY" > "$EXAMPLES_JSON"

# Generate tests from examples
"${SCRIPT_DIR}/../.aoc-generate-tests.sh" "$DAY" kotlin

echo ""
echo "Day ${DAY} ready! Next steps:"
echo "  cd day-${DAY}"
echo "  ./gradlew test --continuous    # TDD watch mode"
echo "  ./gradlew run                  # Run solution"
echo ""
echo "Or use watch mode:"
echo "  cd ${SCRIPT_DIR}"
echo "  ./watch.sh day-${DAY}"
