#!/bin/bash

# Generate test files from parsed examples
# Usage: .aoc-generate-tests.sh <language> <day-dir> <examples-json>

set -e

LANG=$1
DAY_DIR=$2
EXAMPLES_JSON=$3

if [ $# -lt 3 ]; then
    echo "Usage: $0 <language> <day-dir> <examples-json>"
    echo "Example: $0 go day-1 '{\"part1\":[{...}],\"part2\":[...]}'"
    exit 1
fi

# Extract examples using jq or python
extract_example() {
    local part=$1
    local field=$2

    if command -v jq &> /dev/null; then
        echo "$EXAMPLES_JSON" | jq -r ".${part}[0].${field} // \"\"" 2>/dev/null || echo ""
    elif command -v python3 &> /dev/null; then
        echo "$EXAMPLES_JSON" | python3 -c "
import sys, json
data = json.load(sys.stdin)
part = data.get('$part', [])
if part and len(part) > 0:
    print(part[0].get('$field', ''))
" 2>/dev/null || echo ""
    else
        echo ""
    fi
}

PART1_INPUT=$(extract_example "part1" "input")
PART1_ANSWER=$(extract_example "part1" "answer")
PART2_INPUT=$(extract_example "part2" "input")
PART2_ANSWER=$(extract_example "part2" "answer")

case $LANG in
    go)
        # Generate Go tests
        cat > "$DAY_DIR/main_test.go" << 'EOF'
package main

import (
	"testing"
)

func TestPart1(t *testing.T) {
	input := `PART1_INPUT_PLACEHOLDER`

	result := part1(input)
	expected := "PART1_ANSWER_PLACEHOLDER"

	if result != expected {
		t.Errorf("part1() = %v, want %v", result, expected)
	}
}

func TestPart2(t *testing.T) {
	input := `PART2_INPUT_PLACEHOLDER`

	result := part2(input)
	expected := "PART2_ANSWER_PLACEHOLDER"

	if result != expected {
		t.Errorf("part2() = %v, want %v", result, expected)
	}
}

func BenchmarkPart1(b *testing.B) {
	for i := 0; i < b.N; i++ {
		part1(input)
	}
}

func BenchmarkPart2(b *testing.B) {
	for i := 0; i < b.N; i++ {
		part2(input)
	}
}
EOF

        # Replace placeholders
        if [ -n "$PART1_INPUT" ]; then
            # Use perl for multi-line replacement
            perl -i -pe "s|PART1_INPUT_PLACEHOLDER|${PART1_INPUT}|g" "$DAY_DIR/main_test.go"
        fi
        if [ -n "$PART1_ANSWER" ]; then
            sed -i '' "s/PART1_ANSWER_PLACEHOLDER/${PART1_ANSWER}/g" "$DAY_DIR/main_test.go"
        fi
        if [ -n "$PART2_INPUT" ]; then
            perl -i -pe "s|PART2_INPUT_PLACEHOLDER|${PART2_INPUT}|g" "$DAY_DIR/main_test.go"
        fi
        if [ -n "$PART2_ANSWER" ]; then
            sed -i '' "s/PART2_ANSWER_PLACEHOLDER/${PART2_ANSWER}/g" "$DAY_DIR/main_test.go"
        fi
        ;;

    zig)
        # Update Zig lib.zig tests
        if [ -f "$DAY_DIR/src/lib.zig" ]; then
            # Create temp file with updated tests
            cat > "$DAY_DIR/src/lib.zig.tmp" << 'EOF'
const std = @import("std");
const testing = std.testing;

pub fn part1(allocator: std.mem.Allocator, input: []const u8) ![]const u8 {
    _ = allocator;
    _ = input;
    return error.NotImplemented;
}

pub fn part2(allocator: std.mem.Allocator, input: []const u8) ![]const u8 {
    _ = allocator;
    _ = input;
    return error.NotImplemented;
}

test "part1 example" {
    const input =
        \\PART1_INPUT_PLACEHOLDER
    ;
    const result = try part1(testing.allocator, input);
    defer testing.allocator.free(result);
    try testing.expectEqualStrings("PART1_ANSWER_PLACEHOLDER", result);
}

test "part2 example" {
    const input =
        \\PART2_INPUT_PLACEHOLDER
    ;
    const result = try part2(testing.allocator, input);
    defer testing.allocator.free(result);
    try testing.expectEqualStrings("PART2_ANSWER_PLACEHOLDER", result);
}
EOF
            # Replace placeholders
            if [ -n "$PART1_INPUT" ]; then
                # For Zig, need to handle multi-line with \\ prefix
                ZIG_INPUT=$(echo "$PART1_INPUT" | sed 's/^/\\\\/g')
                perl -i -pe "s|PART1_INPUT_PLACEHOLDER|${ZIG_INPUT}|g" "$DAY_DIR/src/lib.zig.tmp"
            fi
            if [ -n "$PART1_ANSWER" ]; then
                sed -i '' "s/PART1_ANSWER_PLACEHOLDER/${PART1_ANSWER}/g" "$DAY_DIR/src/lib.zig.tmp"
            fi
            if [ -n "$PART2_INPUT" ]; then
                ZIG_INPUT=$(echo "$PART2_INPUT" | sed 's/^/\\\\/g')
                perl -i -pe "s|PART2_INPUT_PLACEHOLDER|${ZIG_INPUT}|g" "$DAY_DIR/src/lib.zig.tmp"
            fi
            if [ -n "$PART2_ANSWER" ]; then
                sed -i '' "s/PART2_ANSWER_PLACEHOLDER/${PART2_ANSWER}/g" "$DAY_DIR/src/lib.zig.tmp"
            fi

            mv "$DAY_DIR/src/lib.zig.tmp" "$DAY_DIR/src/lib.zig"
        fi
        ;;

    csharp)
        # C# doesn't have a separate test file by default
        # Could add xUnit tests, but for now, skip
        echo "C# test generation not yet implemented (tests inline in Program.cs)"
        ;;

    *)
        echo "Unknown language: $LANG"
        exit 1
        ;;
esac

echo "Tests generated for $LANG in $DAY_DIR"
