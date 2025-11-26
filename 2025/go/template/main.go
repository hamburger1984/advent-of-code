package main

import (
	_ "embed"
	"fmt"
	"strings"
)

//go:embed input.txt
var input string

func main() {
	input = strings.TrimSpace(input)

	fmt.Printf("Part 1: %v\n", part1(input))
	fmt.Printf("Part 2: %v\n", part2(input))
}

func part1(input string) any {
	// TODO: Implement part 1
	lines := strings.Split(input, "\n")
	_ = lines

	// Example: use lo for functional operations
	// filtered := lo.Filter(lines, func(line string, _ int) bool { return len(line) > 0 })
	// mapped := lo.Map(filtered, func(line string, _ int) int { return len(line) })

	return "not implemented"
}

func part2(input string) any {
	// TODO: Implement part 2
	lines := strings.Split(input, "\n")
	_ = lines
	return "not implemented"
}
