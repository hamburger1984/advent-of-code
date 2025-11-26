package main

import (
	"testing"
)

func TestPart1(t *testing.T) {
	input := `TODO: Add example input`

	result := part1(input)
	expected := "TODO"

	if result != expected {
		t.Errorf("part1() = %v, want %v", result, expected)
	}
}

func TestPart2(t *testing.T) {
	input := `TODO: Add example input`

	result := part2(input)
	expected := "TODO"

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
