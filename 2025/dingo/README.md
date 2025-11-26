# Advent of Code 2025 - Dingo

## Setup

Dingo is a transpiler to Go that provides modern language features.

### Installation

```bash
git clone https://github.com/MadAppGang/dingo.git
cd dingo
go build -o dingo ./cmd/dingo
# Add to PATH or use ./dingo directly
```

### Requirements
- Go 1.21+
- Dingo transpiler

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
├── main.dingo       # Solution in Dingo syntax
├── go.mod           # Generated Go module
├── input.txt        # Puzzle input
└── task.txt         # Problem description
```

## Workflow

1. Create a new day: `./create-day.sh 1 --fetch`
2. Add puzzle input to `day-1/input.txt`
3. Add problem description to `day-1/task.txt`
4. Implement solutions in `day-1/main.dingo`
5. Transpile and run:
   ```bash
   cd day-1
   dingo build main.dingo
   go run .
   ```

## Dingo Language Features

Dingo extends Go with:

### Sum Types
```dingo
type Result = Ok(int) | Err(string)

fn process(r: Result) -> int {
    match r {
        Ok(val) => val,
        Err(msg) => {
            println("Error: {}", msg)
            0
        }
    }
}
```

### Error Propagation
```dingo
fn readFile(path: string) -> Result<string, error> {
    content := fs.readToString(path)?  // The ? operator!
    return Ok(content)
}
```

### Safe Navigation
```dingo
let value = obj?.field ?? "default"
```

### Functional Utilities
```dingo
let numbers = [1, 2, 3, 4, 5]
let doubled = numbers.map(|n| n * 2)
let evens = numbers.filter(|n| n % 2 == 0)
let sum = numbers.reduce(0, |acc, n| acc + n)
```

## Build and Run

```bash
# Transpile Dingo to Go
cd day-1
dingo build main.dingo

# Run the generated Go code
go run .

# Or compile to binary
go build -o solution
./solution
```

## Tips

- Dingo transpiles to idiomatic Go with zero runtime overhead
- Generated Go code has no external dependencies
- Use Dingo features for cleaner, safer code
- The transpiler provides accurate error messages
- LSP support available for editor integration

## Resources

- [Dingo GitHub](https://github.com/MadAppGang/dingo)
- [Dingo Documentation](https://github.com/MadAppGang/dingo/tree/main/docs)
