# Advent of Code 2025

Solutions for [Advent of Code 2025](https://adventofcode.com/2025) in multiple languages.

## Languages

This year I'm exploring six different languages and paradigms:

### 🔷 [Zig](./zig/) - Systems Programming
- Modern low-level language with safety and performance
- Manual memory management with allocators
- Comptime for compile-time execution
- Zero-cost abstractions

### 🟣 [C#](./csharp/) - High-Level Productivity  
- Modern .NET 10.0 LTS with latest C# 14 features
- Top-level statements for concise code
- LINQ for functional-style data processing
- Pattern matching and records

### 🔵 [Go with Lo](./go/) - Functional Go
- Go 1.23 with functional programming
- [Lo](https://github.com/samber/lo) - Lodash-style utilities
- Built-in testing and benchmarking
- Fast compilation and execution

### 🟠 [Dingo](./dingo/) - Modern Go Transpiler
- Transpiles to Go with modern language features
- Sum types and pattern matching
- Pipe operators and functional utilities
- Error propagation with `?` operator

### 🟢 [Swift](./swift/) - Apple's Modern Language
- Swift 5.9+ with Swift Package Manager
- Powerful type inference and safety
- Functional programming with map/filter/reduce
- Fast native performance

### 🟤 [Kotlin](./kotlin/) - JVM Powerhouse
- Kotlin 1.9+ with Gradle build system
- Null safety and smart casts
- Extension functions and DSLs
- Seamless Java interop

## Quick Start

### TDD Workflow (Recommended) 🚀

**New!** Automatic test generation from puzzle examples:

```bash
# 1. Set up session token (one-time): See AOC_FETCH_SETUP.md
echo 'your-session-token' > ~/.aoc-session
chmod 600 ~/.aoc-session

# 2. Create day with auto-fetch (generates tests automatically!)
cd zig
./create-day.sh 1 --fetch
# → Input fetched
# → Part 1 task fetched  
# → Tests auto-generated from examples!

# 3. Start TDD mode
./watch.sh 1
# → Tests run automatically on file save
# → Red → Green → Refactor cycle!

# 4. After solving Part 1, get Part 2
cd .. && ./.aoc-update-task.sh 1 zig
# → Part 2 tests auto-generated!

# 5. Continue TDD for Part 2
cd zig && ./watch.sh 1
```

**Note:** Puzzle data is cached and shared across all languages. Fetch once in any language, use everywhere!

### With Auto-Fetch (No Watch Mode)

First, set up your AoC session token (one-time): See [AOC_FETCH_SETUP.md](./AOC_FETCH_SETUP.md)

**Part 2:** After solving Part 1, update the task with: `./.aoc-update-task.sh <day> <language>`

```bash
# Zig
cd zig && ./create-day.sh 1 --fetch
cd day-1 && zig build test-day-1 && zig build run-day-1-part1

# C#
cd csharp && ./create-day.sh 1 --fetch
cd day-1 && dotnet run

# Go
cd go && ./create-day.sh 1 --fetch
cd day-1 && go test -v && go run .

# Dingo (transpiles to Go)
cd dingo && ./create-day.sh 1
cd day-1 && dingo build -o main && ./main

# Swift
cd swift && ./create-day.sh 1
cd day-1 && swift test && swift run

# Kotlin
cd kotlin && ./create-day.sh 1
cd day-1 && ./gradlew test && ./gradlew run
```

### Manual Setup

```bash
# Zig
cd zig && ./create-day.sh 1
cd day-1
# Manually add input.txt and task.txt
zig build test-day-1 && zig build run-day-1-part1

# C#
cd csharp && ./create-day.sh 1
cd day-1
# Manually add input.txt and task.txt
dotnet run

# Go
cd go && ./create-day.sh 1
cd day-1
# Manually add input.txt and task.txt
go test -v && go run .
```

## Structure

Each language follows a similar pattern:

```
LANGUAGE/
├── README.md           # Language-specific documentation
├── create-day.sh       # Script to create new day
├── template/           # Boilerplate template
└── day-N/              # Individual day solutions
    ├── input.txt       # Puzzle input
    ├── task.txt        # Problem description
    └── ...             # Language-specific files
```

## Previous Years

- [2024](../2024/) - Rust (refined workspace)
- [2023](../2023/) - Rust (workspace with tooling)
- [2022](../2022/) - Rust (standalone projects)
- [2021](../2021/) - C#

## Progress

| Day | Zig | C# | Go | Dingo | Swift | Kotlin |
|-----|-----|----|----| ------|-------|--------|
| 1   | ⬜  | ✅ | ⬜ | ⬜    | ⬜    | ⬜     |
| 2   | ⬜  | ⭐ | ⬜ | ⬜    | ⬜    | ⬜     |
| 3   | ⬜  | ⬜ | ⬜ | ⬜    | ⬜    | ⬜     |
| 4   | ⬜  | ⬜ | ⬜ | ⬜    | ⬜    | ⬜     |
| 5   | ⬜  | ⬜ | ⬜ | ⬜    | ⬜    | ⬜     |
| 6   | ⬜  | ⬜ | ⬜ | ⬜    | ⬜    | ⬜     |
| 7   | ⬜  | ⬜ | ⬜ | ⬜    | ⬜    | ⬜     |
| 8   | ⬜  | ⬜ | ⬜ | ⬜    | ⬜    | ⬜     |
| 9   | ⬜  | ⬜ | ⬜ | ⬜    | ⬜    | ⬜     |
| 10  | ⬜  | ⬜ | ⬜ | ⬜    | ⬜    | ⬜     |
| 11  | ⬜  | ⬜ | ⬜ | ⬜    | ⬜    | ⬜     |
| 12  | ⬜  | ⬜ | ⬜ | ⬜    | ⬜    | ⬜     |
| 13  | ⬜  | ⬜ | ⬜ | ⬜    | ⬜    | ⬜     |
| 14  | ⬜  | ⬜ | ⬜ | ⬜    | ⬜    | ⬜     |
| 15  | ⬜  | ⬜ | ⬜ | ⬜    | ⬜    | ⬜     |
| 16  | ⬜  | ⬜ | ⬜ | ⬜    | ⬜    | ⬜     |
| 17  | ⬜  | ⬜ | ⬜ | ⬜    | ⬜    | ⬜     |
| 18  | ⬜  | ⬜ | ⬜ | ⬜    | ⬜    | ⬜     |
| 19  | ⬜  | ⬜ | ⬜ | ⬜    | ⬜    | ⬜     |
| 20  | ⬜  | ⬜ | ⬜ | ⬜    | ⬜    | ⬜     |
| 21  | ⬜  | ⬜ | ⬜ | ⬜    | ⬜    | ⬜     |
| 22  | ⬜  | ⬜ | ⬜ | ⬜    | ⬜    | ⬜     |
| 23  | ⬜  | ⬜ | ⬜ | ⬜    | ⬜    | ⬜     |
| 24  | ⬜  | ⬜ | ⬜ | ⬜    | ⬜    | ⬜     |
| 25  | ⬜  | ⬜ | ⬜ | ⬜    | ⬜    | ⬜     |

Legend: ⬜ Not started | 🟡 Part 1 | ✅ Complete

## Notes

- Each day's `input.txt` contains the personalized puzzle input
- Each day's `task.txt` contains the problem description
- Solutions are self-contained within each day directory
- All three languages use embedded/compiled input for performance

## Verified Setup

All six language environments are ready:
- ✅ Zig 0.15.2 - Build system configured
- ✅ C# .NET 10.0 LTS - Solution and projects ready
- ✅ Go 1.23 with Lo - Module dependencies configured
- ✅ Dingo - Transpiler with modern Go features
- ✅ Swift 5.9+ - Package Manager configured
- ✅ Kotlin 1.9+ - Gradle build system ready

Happy coding! 🎄
