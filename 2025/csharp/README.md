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

1. Create a new day: `./create-day.sh 1`
2. Add puzzle input to `day-1/input.txt`
3. Add problem description to `day-1/task.txt`
4. Implement `Part1()` and `Part2()` functions in `day-1/Program.cs`
5. Run: `cd day-1 && dotnet run`

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
