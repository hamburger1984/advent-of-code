# Advent of Code 2025 - Swift

## Setup

Requires Swift 5.9+ (comes with Xcode or can be installed standalone).

```bash
swift --version
```

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
├── Package.swift    # Swift Package Manager manifest
├── Sources/
│   └── Main.swift   # Solution with part1() and part2()
├── Tests/
│   └── SolutionTests.swift  # Tests (auto-generated from examples)
├── input.txt        # Puzzle input
└── task.txt         # Problem description
```

## Workflow

1. Create a new day: `./create-day.sh 1 --fetch`
2. Implement solutions in `day-1/Sources/Main.swift`
3. Run tests: `cd day-1 && swift test`
4. Run solution: `cd day-1 && swift run`

## Running Solutions

```bash
# Run solution
cd day-1
swift run

# Run tests
swift test

# Run in release mode (optimized)
swift run -c release

# Build executable
swift build -c release
./.build/release/day-1
```

## Testing

```bash
# Run tests
cd day-1
swift test

# Run specific test
swift test --filter testPart1

# Run with verbose output
swift test --verbose
```

## Swift Language Features

### Modern Syntax
```swift
let numbers = [1, 2, 3, 4, 5]
let doubled = numbers.map { $0 * 2 }
let evens = numbers.filter { $0 % 2 == 0 }
let sum = numbers.reduce(0, +)
```

### Pattern Matching
```swift
enum Result {
    case success(Int)
    case failure(String)
}

switch result {
case .success(let value):
    print("Got: \\(value)")
case .failure(let error):
    print("Error: \\(error)")
}
```

### Optionals & Error Handling
```swift
guard let value = optionalValue else { return }

do {
    let data = try String(contentsOfFile: "input.txt")
} catch {
    print("Error: \\(error)")
}
```

### Collections
```swift
// Arrays
let array = [1, 2, 3]

// Sets
let set = Set([1, 2, 3])

// Dictionaries
let dict = ["a": 1, "b": 2]

// Slices
let slice = array[1...]
```

## Tips

- Swift Package Manager handles dependencies
- Use `swift test --watch` (if available) for TDD
- Leverage Swift's type safety for correctness
- Use `print()` for debugging
- Swift's standard library is powerful - explore it!

## Resources

- [Swift Language Guide](https://docs.swift.org/swift-book/)
- [Swift Standard Library](https://developer.apple.com/documentation/swift/swift_standard_library)
- [Swift Package Manager](https://www.swift.org/package-manager/)
