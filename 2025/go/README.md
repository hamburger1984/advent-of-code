# Advent of Code 2025 - Go with Lo

## Setup

Requires Go 1.23 or later.

This setup uses [Lo](https://github.com/samber/lo) - a popular functional programming library for Go inspired by Lodash.

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
├── go.mod           # Module file with Lo dependency
├── main.go          # Solution with part1() and part2() functions
├── main_test.go     # Tests and benchmarks
├── input.txt        # Puzzle input (embedded at compile time)
└── task.txt         # Problem description
```

## Workflow

1. Create a new day: `./create-day.sh 1`
2. Add puzzle input to `day-1/input.txt`
3. Add problem description to `day-1/task.txt`
4. Implement `part1()` and `part2()` functions in `day-1/main.go`
5. Run: `cd day-1 && go run .`
6. Test: `cd day-1 && go test -v`

## Running Solutions

```bash
# Run solution
cd day-1
go run .

# Run with race detector
go run -race .

# Build optimized binary
go build -ldflags="-s -w" -o solution
./solution
```

## Testing

```bash
# Run tests
cd day-1
go test -v

# Run benchmarks
go test -bench=.

# Run benchmarks with memory stats
go test -bench=. -benchmem

# Profile CPU
go test -bench=. -cpuprofile=cpu.prof
go tool pprof cpu.prof
```

## Using Lo

Lo provides functional programming utilities for working with slices, maps, and more:

```go
import "github.com/samber/lo"

// Filter and Map
result := lo.Map(
    lo.Filter(numbers, func(n int, _ int) bool { return n > 0 }),
    func(n int, _ int) int { return n * 2 },
)

// Reduce
sum := lo.Reduce(numbers, func(acc int, n int, _ int) int {
    return acc + n
}, 0)

// Chunk slices
chunks := lo.Chunk(items, 3)  // [[1,2,3], [4,5,6], ...]

// Unique values
unique := lo.Uniq([]int{1, 2, 2, 3, 3, 3})

// Group by key
grouped := lo.GroupBy(items, func(item Item) string {
    return item.Category
})

// Parallel processing with lo.Async*
results := lo.Map(items, func(item string, _ int) Result {
    return processItem(item)
})

// Common utilities
min := lo.Min([]int{1, 2, 3})
max := lo.Max([]int{1, 2, 3})
sum := lo.Sum([]int{1, 2, 3})
```

Key Lo functions for AoC:
- `lo.Filter()` - Filter elements
- `lo.Map()` - Transform elements
- `lo.Reduce()` - Reduce to single value
- `lo.Chunk()` - Split into chunks
- `lo.Uniq()` - Remove duplicates
- `lo.GroupBy()` - Group by key function
- `lo.Sum()`, `lo.Min()`, `lo.Max()` - Math operations
- `lo.Reverse()` - Reverse slice
- `lo.Flatten()` - Flatten nested slices
- `lo.Count()`, `lo.CountBy()` - Count elements

See the [Lo documentation](https://github.com/samber/lo) for complete API.

## Tips

- Input is embedded at compile time using `//go:embed`
- Each day is an independent module
- Use Lo's functional style for clean, expressive solutions
- Tests and benchmarks are in `main_test.go`
- Return `any` type from part functions for flexibility
- Use `strings.TrimSpace()` to clean input
- Lo works with Go generics for type safety
