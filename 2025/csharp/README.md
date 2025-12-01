# Advent of Code 2025 - C#

## Setup

Requires .NET 10.0 SDK (or .NET 9.0 if 10 is not yet installed).

Download: https://dotnet.microsoft.com/download/dotnet/10.0

## Creating a New Day

```bash
# With auto-fetch (requires session token setup)
./create-day.sh 1 --fetch

# Or manually
./create-day.sh 1
```

This creates a `day-1` directory with the template structure and adds it to the solution. With `--fetch`, it automatically downloads the puzzle input and description. See [../AOC_FETCH_SETUP.md](../AOC_FETCH_SETUP.md) for setup.

## Structure

Each day follows this pattern:

```
day-N/
├── DayN.csproj      # Project file
├── Program.cs       # Solution with Part1() and Part2() functions
├── input.txt        # Puzzle input
└── task.txt         # Problem description
```

## Workflow

### TDD Workflow (Recommended)

When using `--fetch`, tests are automatically generated from the problem's examples:

```bash
./create-day.sh 1 --fetch
cd day-1

# The template includes inline tests that fail initially
# Implement Part1() until tests pass
dotnet run

# After solving Part 1 on AoC website, fetch Part 2
../.aoc-update-task.sh 1 csharp

# Tests are updated automatically - implement Part 2
dotnet run
```

### Manual Workflow

1. Create a new day: `./create-day.sh 1`
2. Add puzzle input to `day-1/input.txt`
3. Add problem description to `day-1/task.txt`
4. Update test examples in `Program.cs` (in the `#if DEBUG` section)
5. Implement `Part1()` and `Part2()` functions in `day-1/Program.cs`
6. Run: `cd day-1 && dotnet run`

### Test Structure

Tests are embedded in `Program.cs` within `#if DEBUG` blocks:

```csharp
#if DEBUG
public static class Tests
{
    static void TestPart1()
    {
        const string input = @"example input";
        const string expected = "42";
        
        var result = Part1(input);
        if (result != expected)
            throw new Exception($"Part 1 test failed: expected '{expected}', got '{result}'");
    }
}
#endif
```

- Tests run automatically in Debug mode
- They're excluded in Release builds
- Verbatim strings (`@"..."`) support multi-line examples
- Tests fail loudly with clear error messages

## Running Solutions

```bash
# Run in debug mode
cd day-1
dotnet run

# Run with optimizations
cd day-1
dotnet run -c Release

# Run from solution root
dotnet run --project day-1/Day1.csproj
```

## Building

```bash
# Build specific day
cd day-1
dotnet build

# Build all days
dotnet build

# Build with optimizations
dotnet build -c Release
```

## Tips

- Both parts are in the same `Program.cs` file
- Use top-level statements for clean code
- Input is read from `input.txt` automatically
- Implement both `Part1()` and `Part2()` as local functions
- Use modern C# features: pattern matching, LINQ, records, etc.
- Enable nullable reference types for safer code
