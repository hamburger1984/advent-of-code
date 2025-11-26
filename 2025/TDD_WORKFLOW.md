# TDD Workflow for Advent of Code 2025

This setup includes **automatic test generation** from puzzle examples, enabling a true Test-Driven Development workflow!

## How It Works

### 1. Automatic Example Extraction

When you create a day with `--fetch`, the system:

1. **Fetches the puzzle** from Advent of Code
2. **Parses the task description** to extract example inputs and expected answers
3. **Generates test files** automatically with real data from the puzzle

**Example from Day 1:**
```
For example:

3   4
4   3
2   5
1   3
3   9
3   3

...explanation...

...a total distance of 11!
```

**Automatically becomes:**
```go
func TestPart1(t *testing.T) {
    input := `3   4
4   3
2   5
1   3
3   9
3   3`
    
    result := part1(input)
    expected := "11"  // Extracted from "total distance of 11!"
    
    if result != expected {
        t.Errorf("part1() = %v, want %v", result, expected)
    }
}
```

### 2. Watch Mode

Each language has a `watch.sh` script that automatically runs tests when you save files:

- **Go:** `cd go && ./watch.sh day-1`
- **Zig:** `cd zig && ./watch.sh 1`
- **C#:** `cd csharp && ./watch.sh day-1`
- **Dingo:** `cd dingo && ./watch.sh day-1`
- **Swift:** `cd swift && ./watch.sh day-1`
- **Kotlin:** `cd kotlin && ./watch.sh day-1`

### 3. TDD Cycle

```
┌─────────────────────────────────────────────┐
│ 1. Create day (tests auto-generated) ✓     │
└────────────────┬────────────────────────────┘
                 ▼
┌─────────────────────────────────────────────┐
│ 2. Start watch mode                         │
│    ./watch.sh 1                             │
└────────────────┬────────────────────────────┘
                 ▼
         ┌───────────────┐
         │  🔴 RED       │  Tests fail (NotImplemented)
         └───────┬───────┘
                 ▼
         ┌───────────────┐
         │  🟢 GREEN     │  Implement until tests pass
         └───────┬───────┘
                 ▼
         ┌───────────────┐
         │  🔵 REFACTOR  │  Clean up code
         └───────┬───────┘
                 ▼
         ┌───────────────┐
         │ Submit Part 1 │
         └───────┬───────┘
                 ▼
┌─────────────────────────────────────────────┐
│ 3. Update to Part 2                         │
│    ./.aoc-update-task.sh 1 zig              │
│    → Part 2 tests auto-generated!           │
└────────────────┬────────────────────────────┘
                 ▼
         ┌───────────────┐
         │ Repeat cycle  │  🔴 → 🟢 → 🔵
         └───────────────┘
```

## Complete Workflow Example

### Go Example

```bash
# Setup (one-time)
cd 2025
echo 'your-session-token' > ~/.aoc-session
chmod 600 ~/.aoc-session

# Day 1 - Part 1
cd go
./create-day.sh 1 --fetch
# Output:
# Creating day-1...
# Fetching input from Advent of Code...
# ✓ Input saved to day-1/input.txt
# Fetching task from Advent of Code...
# ✓ Task saved to day-1/task.txt
# Generating tests from examples...
# ✓ Tests generated with example data

# Start TDD mode
cd day-1
../watch.sh
# → Terminal shows test results, auto-updates on save

# In another terminal/editor:
# Edit main.go, implement part1()
# Save file → tests run automatically
# Keep iterating until green!

# Submit Part 1 answer on website

# Get Part 2
cd ../..
./.aoc-update-task.sh 1 go
# ✓ Using cached Part 2 task
# ✓ Updated go/day-1/task.txt with Part 2

# Check tests - Part 2 test now populated!
cd go/day-1
cat main_test.go  # See TestPart2 with real data

# Continue TDD
../watch.sh
# Implement part2(), iterate until green!
```

### Zig Example

```bash
# Create day
cd zig
./create-day.sh 1 --fetch
# → Tests auto-generated

# Start watch mode
./watch.sh 1
# → Auto-runs: zig build test-day-1

# Edit day-1/src/lib.zig
# Implement part1() and part2()
# Tests run on every save!

# Get Part 2 after solving Part 1
cd .. && ./.aoc-update-task.sh 1 zig
# → Tests updated

# Continue
cd zig && ./watch.sh 1
```

## Parser Capabilities

The example parser (`.aoc-parse-examples.sh`) handles:

### Supported Patterns

✅ **Input Detection:**
- Indented blocks after "For example:"
- "Here are the same example lists again:"
- Data blocks (numbers, structured text)

✅ **Answer Detection:**
- "total distance of 11!"
- "similarity score is 31"
- "answer is X"
- "result is X"
- Numbers followed by `!` or `.` in conclusions

### Limitations

⚠️ **May need manual adjustment for:**
- Multiple examples per part (only first is used)
- Complex output formats (grids, strings with spaces)
- Implicit answers (not stated explicitly)
- Multi-step examples

💡 **Solution:** Generated tests are real code - edit them if needed!

## Watch Mode Details

### Requirements

**Recommended:** Install `fswatch` for best performance:
```bash
brew install fswatch  # macOS
apt-get install fswatch  # Linux
```

**Fallback:** Scripts work without fswatch (simple polling, slower)

### Features

- **Automatic test runs** on file save
- **Clear output** with colored test results
- **Fast feedback loop** (< 1 second with fswatch)
- **Ctrl+C to stop**

### Language-Specific Behavior

**Go:** Watches entire directory, runs `go test -v ./...`

**Zig:** Watches `day-N/src/`, runs `zig build test-day-N`

**C#:** Uses `dotnet watch run` (tests not yet configured)

**Dingo:** Watches source files, transpiles and runs Go tests

**Swift:** Watches `src/` directory, runs `swift test`

**Kotlin:** Watches `src/` directory, runs `./gradlew test --quiet`

## Tips & Tricks

### 1. Start with Examples

The auto-generated tests use the puzzle's examples - these are **always simpler** than the full input. Get them passing first!

### 2. Check Answers Before Implementing

Look at the generated test to see what answer format is expected:
```go
expected := "11"  // String, not int!
```

### 3. Multiple Examples

Some puzzles have multiple examples. The parser only captures the first one. You can:
- Manually add more test cases
- Extract others with: `./.aoc-parse-examples.sh day-1/task.txt 1`

### 4. Part 2 Updates

After fetching Part 2:
```bash
# Check what changed
git diff day-1/task.txt

# Verify tests updated
cat day-1/main_test.go  # Go
cat day-1/src/lib.zig   # Zig
```

### 5. Manual Test Editing

Generated tests are normal code - edit freely!

```go
// Add more test cases
func TestPart1_EdgeCase(t *testing.T) {
    input := `...`
    result := part1(input)
    expected := "..."
    if result != expected {
        t.Errorf("...")
    }
}
```

### 6. Debug with Print Statements

While in watch mode, add debug output:
```go
fmt.Println("Debug:", someValue)
```
Save → see output immediately in watch terminal!

## Troubleshooting

### Tests not generated

**Check:**
1. Task file exists: `ls day-1/task.txt`
2. Parser works: `./.aoc-parse-examples.sh day-1/task.txt all`
3. Permissions: `chmod +x .aoc-*.sh`

### Parser returns empty/wrong answers

**Common causes:**
- Unusual answer format in puzzle
- Multiple answers mentioned (parser picks last)
- Complex example structure

**Solution:** Manually edit generated tests

### Watch mode not updating

**Check:**
1. fswatch installed: `brew install fswatch`
2. Correct directory: `cd day-1` (Go) or `cd zig` (Zig)
3. File actually saved (check modification time)

### Tests pass but answer wrong

Remember:
- Examples are **simpler** than full input
- Full input might have edge cases
- Check your algorithm, not just implementation

## Advanced: Custom Test Generation

You can customize test generation by editing `.aoc-generate-tests.sh`:

```bash
# Add custom test template
# Support additional assertions
# Generate multiple test cases
```

## Benefits Summary

### Without TDD Workflow
```
1. Create day manually
2. Copy example input to test file manually
3. Copy expected answer manually
4. Run tests manually
5. Edit code
6. Run tests manually again
7. Repeat...
```

### With TDD Workflow
```
1. ./create-day.sh 1 --fetch
   → Everything auto-generated!
2. ./watch.sh 1
   → Tests run automatically!
3. Edit code
   → Immediate feedback!
4. Done! ✓
```

**Time saved:** ~5-10 minutes per puzzle  
**Focus gained:** Pure problem-solving, no setup!

---

Happy TDD-ing! 🎄🧪✨
