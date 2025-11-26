# Advent of Code 2025 - Kotlin Solutions

Solutions using Kotlin with Gradle build system.

## Prerequisites

- Kotlin 1.9+ (install via [SDKMAN](https://sdkman.io/))
- Gradle 8.0+ (wrapper included)

```bash
# Install SDKMAN
curl -s "https://get.sdkman.io" | bash

# Install Kotlin
sdk install kotlin

# Gradle wrapper is included in each day's project
```

## Project Structure

Each day is a standalone Gradle project with Kotlin DSL:

```
day-N/
├── build.gradle.kts        # Gradle build configuration
├── settings.gradle.kts     # Project settings
├── input.txt               # Puzzle input (gitignored)
├── task.txt                # Puzzle description
└── src/
    ├── main/kotlin/
    │   └── Solution.kt     # Main solution code
    └── test/kotlin/
        └── SolutionTest.kt # Unit tests with examples
```

## Creating a New Day

Use the helper script:

```bash
cd 2025/kotlin
./create-day.sh N  # where N is 1-25
```

This will:
1. Check cache for task/input (or fetch from AoC)
2. Copy template to day-N/
3. Parse examples and generate tests
4. Set up Gradle wrapper

## Workflow

### 1. Navigate to day folder
```bash
cd day-N
```

### 2. Run tests (TDD mode)
```bash
./gradlew test --continuous  # Watch mode
# or
./gradlew test              # Single run
```

### 3. Run solution
```bash
./gradlew run
```

### 4. Watch mode for instant feedback
```bash
cd 2025/kotlin
./watch.sh day-N
```

## Development Tips

### Kotlin Features for AoC

```kotlin
// Functional collections
val numbers = lines.map { it.toInt() }
val sum = numbers.sum()
val filtered = numbers.filter { it > 10 }

// Grouping and partitioning
val (evens, odds) = numbers.partition { it % 2 == 0 }
val grouped = items.groupBy { it.type }

// Destructuring
val (x, y) = point
val (first, second) = pair

// Extension functions
fun String.toIntList() = split(" ").map { it.toInt() }

// Sequence for lazy evaluation
val result = lines.asSequence()
    .map { process(it) }
    .filter { it.isValid() }
    .take(10)
    .toList()

// Pattern matching
when (val value = compute()) {
    is String -> handleString(value)
    is Int -> handleInt(value)
    else -> handleOther(value)
}
```

### Testing with JUnit 5

```kotlin
import org.junit.jupiter.api.Test
import kotlin.test.assertEquals

class SolutionTest {
    @Test
    fun `part1 example`() {
        val input = """
            example input
        """.trimIndent()
        
        assertEquals("expected", part1(input))
    }
}
```

## Troubleshooting

**Gradle wrapper not executable:**
```bash
chmod +x gradlew
```

**Build fails:**
```bash
./gradlew clean build
```

**Tests not running:**
```bash
./gradlew test --info
```

## Performance Tips

- Use `Sequence` for large datasets to avoid intermediate collections
- Use `buildString` for string concatenation
- Consider `Array` instead of `List` for primitive types
- Profile with `measureTimeMillis { }`

## Learning Resources

- [Kotlin Docs](https://kotlinlang.org/docs/home.html)
- [Kotlin Collections](https://kotlinlang.org/docs/collections-overview.html)
- [Kotlin Sequences](https://kotlinlang.org/docs/sequences.html)
