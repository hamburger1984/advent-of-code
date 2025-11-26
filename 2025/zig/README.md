# Advent of Code 2025 - Zig

## Setup

Requires Zig 0.13.0 or later.

## Creating a New Day

```bash
# With auto-fetch (requires session token setup)
./create-day.sh 1 --fetch

# Or manually
./create-day.sh 1
```

This creates a `day-1` directory with the template structure. With `--fetch`, it automatically downloads the puzzle input and description. See [../AOC_FETCH_SETUP.md](../AOC_FETCH_SETUP.md) for setup.

## Structure

Each day follows this pattern:

```
day-N/
├── src/
│   ├── lib.zig      # Solution logic with tests
│   ├── part1.zig    # Part 1 executable
│   └── part2.zig    # Part 2 executable
├── input.txt        # Puzzle input
└── task.txt         # Problem description
```

## Workflow

1. Create a new day: `./create-day.sh 1`
2. Add puzzle input to `day-1/input.txt`
3. Add problem description to `day-1/task.txt`
4. Implement solutions in `day-1/src/lib.zig`
5. Run tests: `zig build test-day-1`
6. Run solutions:
   - `zig build run-day-1-part1`
   - `zig build run-day-1-part2`

## Running Tests

```bash
# Test specific day
zig build test-day-1

# Test all days
zig build test
```

## Building and Running

```bash
# Run part 1
zig build run-day-1-part1

# Run part 2
zig build run-day-1-part2

# Build with optimizations
zig build -Doptimize=ReleaseFast
```

## Tips

- Implement `part1()` and `part2()` functions in `lib.zig`
- Both functions take an allocator and input string
- Write tests using example inputs from the problem
- Use `@embedFile()` to include input at compile time
- The allocator is your friend - use it for dynamic allocations
