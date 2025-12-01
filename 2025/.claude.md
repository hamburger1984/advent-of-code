# Claude Instructions for Advent of Code 2025

## Important: Always Update Progress Table

**Before committing any solved puzzle**, you MUST update the progress table in `README.md`.

### Progress Table Symbols

- ⬜ Not started
- 🟡 Part 1 complete (only)
- ✅ Both parts complete

### Location

The progress table is in `/2025/README.md` under the `## Progress` section.

### Example

When Day 5 Part 1 is solved in Zig:
```markdown
| 5   | 🟡  | ⬜ | ⬜ | ⬜    | ⬜    | ⬜     |
```

When Day 5 Part 2 is solved in Zig (completing the day):
```markdown
| 5   | ✅  | ⬜ | ⬜ | ⬜    | ⬜    | ⬜     |
```

When the same day is solved in C#:
```markdown
| 5   | ✅  | ✅ | ⬜ | ⬜    | ⬜    | ⬜     |
```

### Workflow

1. User solves puzzle and asks for commit
2. **Check**: Is this a Part 1 or Part 2 solution?
3. **Update**: Modify README.md progress table
   - Part 1 only: Use 🟡
   - Both parts: Use ✅
4. **Commit**: Include README.md in the commit with the solution

### Multiple Languages

If a day is solved in multiple languages, each language gets its own column updated.

Example - Day 3 solved in Zig (both parts) and C# (Part 1 only):
```markdown
| 3   | ✅  | 🟡 | ⬜ | ⬜    | ⬜    | ⬜     |
```

## Standard Commit Messages for Solutions

### Part 1
```
Solve 2025 Day X Part 1 in [Language] - Answer: [number]

[Brief description of solution approach]

Answer: [number]
```

### Part 2 (completing the day)
```
Solve 2025 Day X Part 2 in [Language] - Answer: [number]

[Brief description of Part 2 changes]

Part 1: [answer1]
Part 2: [answer2]

Day X complete! ⭐⭐
```

## Files to Include in Commits

### For Part 1
- `2025/[language]/day-X/Program.cs` (or equivalent solution file)
- `2025/[language]/day-X/input.txt` (if not already committed)
- `2025/[language]/day-X/task.txt` (if not already committed)
- `2025/README.md` (progress table update)
- Any project files (.csproj, build.zig, etc.) if first time

### For Part 2
- `2025/[language]/day-X/Program.cs` (or equivalent solution file)
- `2025/[language]/day-X/task.txt` (updated with Part 2 content)
- `2025/README.md` (progress table update from 🟡 to ✅)

## Task.txt Updates

When solving Part 2, remember to update task.txt:
```bash
cd /Users/andreas/Source/advent-of-code/2025
./.aoc-update-task.sh X [language]
```

This fetches Part 2 description and updates the task file with proper formatting.

## Reminder Checklist

Before every solution commit, verify:
- [ ] README.md progress table updated
- [ ] Correct symbol (🟡 for Part 1, ✅ for both parts)
- [ ] Correct language column
- [ ] task.txt includes Part 2 content (if Part 2 commit)
- [ ] Commit message follows template
- [ ] All relevant files staged

---

**This file helps Claude remember to update the progress table. If you're reading this and about to commit a solution, check the README.md first!**
