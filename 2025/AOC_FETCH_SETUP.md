# Advent of Code Auto-Fetch Setup

This repository includes an automated tool to fetch puzzle inputs and descriptions from Advent of Code while respecting their automation policies.

## Features

✅ **Policy-Compliant**
- Rate limited (3 seconds between requests)
- Caches all responses locally
- Includes proper User-Agent header
- Uses authenticated session tokens

✅ **Easy to Use**
- One-time setup
- Automatic fetching with `--fetch` flag
- Works with all three language setups

## One-Time Setup

### Step 1: Get Your Session Token

1. **Log in** to [adventofcode.com](https://adventofcode.com) using GitHub

2. **Open Browser DevTools:**
   - Chrome/Edge: Press `F12` or `Ctrl+Shift+I` (Windows) / `Cmd+Option+I` (Mac)
   - Firefox: Press `F12` or `Ctrl+Shift+I` (Windows) / `Cmd+Option+I` (Mac)
   - Safari: Enable Developer menu in Preferences, then `Cmd+Option+I`

3. **Find the Session Cookie:**
   - **Chrome/Edge:** DevTools → `Application` tab → `Storage` → `Cookies` → `https://adventofcode.com`
   - **Firefox:** DevTools → `Storage` tab → `Cookies` → `https://adventofcode.com`
   - **Safari:** DevTools → `Storage` tab → `Cookies` → `adventofcode.com`

4. **Copy the value** of the cookie named `session`
   - It looks like: `53616c7465645f5f...` (long hexadecimal string)

### Step 2: Save Your Session Token

```bash
# Save your session token
echo 'your-session-token-here' > ~/.aoc-session

# Secure the file (important!)
chmod 600 ~/.aoc-session
```

**Important:** Keep this token secret! Anyone with your session token can access your Advent of Code account.

### Step 3: Verify Setup

```bash
cd 2025
./.aoc-fetch.sh 1 input
```

If you see puzzle input, you're all set! 🎉

## Usage

### Automatic Fetching (Recommended)

When creating a new day, add the `--fetch` flag:

```bash
# Zig
cd zig
./create-day.sh 5 --fetch

# C#
cd csharp
./create-day.sh 5 --fetch

# Go
cd go
./create-day.sh 5 --fetch
```

This will:
1. Create the day directory from template
2. Automatically fetch puzzle input
3. Automatically fetch puzzle description
4. Save both to `input.txt` and `task.txt`

### Manual Fetching

You can also fetch data manually:

```bash
# Fetch just the input
./.aoc-fetch.sh 5 input > zig/day-5/input.txt

# Fetch just the Part 1 task description
./.aoc-fetch.sh 5 task > zig/day-5/task.txt

# Fetch both input and Part 1 task (separated by ---TASK---)
./.aoc-fetch.sh 5 both
```

### Updating to Part 2

After you solve Part 1 correctly and submit your answer, Part 2 becomes available. Update your task description:

```bash
# From the 2025 directory
./.aoc-update-task.sh 5 zig      # Updates zig/day-5/task.txt
./.aoc-update-task.sh 5 csharp   # Updates csharp/day-5/task.txt
./.aoc-update-task.sh 5 go       # Updates go/day-5/task.txt
```

Or fetch Part 2 manually:

```bash
./.aoc-fetch.sh 5 task2 > zig/day-5/task.txt
```

**Note:** Part 2 is only available after submitting the correct Part 1 answer on the AoC website.

## How It Works

### Shared Caching Across Languages

All fetched data is cached in a **single shared location** at `~/.aoc-cache/2025/`:
```
~/.aoc-cache/2025/
├── day1_input.txt
├── day1_task_part1.html
├── day1_task_part2.html    # Only after Part 1 is solved
├── day2_input.txt
├── day2_task_part1.html
└── ...
```

**Key benefit:** If you fetch Day 5 for Zig, then later create Day 5 for Go or C#, the data is **instantly available from cache** - no additional network requests! 

Example workflow:
```bash
cd zig
./create-day.sh 5 --fetch   # Downloads from AoC (first time)

cd ../go
./create-day.sh 5 --fetch   # ✓ Uses cached data (instant, no download!)

cd ../csharp
./create-day.sh 5 --fetch   # ✓ Uses cached data (instant, no download!)
```

This means:
- **Fetch once, use everywhere** - Data downloaded once is available to all languages
- **Faster workflow** - No waiting for repeat downloads
- **Respects AoC servers** - Only one request per day per year, ever
- **Consistent data** - All languages use identical input/task

### Rate Limiting

The tool automatically enforces a 3-second delay between requests to Advent of Code servers. You'll see a message if you need to wait:

```
Rate limiting: waiting 2s...
```

This prevents overwhelming the AoC servers and follows community guidelines.

### Privacy & Security

- Session token stored in `~/.aoc-session` with `600` permissions (user-only access)
- Token never logged or displayed
- Token used only for AoC authentication
- No data sent to third parties

## Troubleshooting

### "Session file not found"

```bash
# Create the session file
echo 'your-token' > ~/.aoc-session
chmod 600 ~/.aoc-session
```

### "Failed to fetch input"

Common causes:
1. **Invalid session token** - Get a fresh token from your browser
2. **Session expired** - Log out and back in to AoC, get new token
3. **Puzzle not yet released** - Wait until puzzle is available (midnight EST)
4. **Network issues** - Check your internet connection

To test your token:
```bash
curl -H "Cookie: session=$(cat ~/.aoc-session)" \
     https://adventofcode.com/2025/day/1/input
```

### "Rate limiting: waiting Xs..."

This is normal! Just wait a few seconds. The tool respects AoC's servers.

### Workflow: Part 1 → Part 2

1. **Start the day:**
   ```bash
   cd zig
   ./create-day.sh 5 --fetch   # Fetches input + Part 1 task
   ```

2. **Solve Part 1:**
   - Implement your solution
   - Test and run it
   - Submit answer on adventofcode.com

3. **Update to Part 2:**
   ```bash
   cd ..
   ./.aoc-update-task.sh 5 zig   # Fetches Part 2 task
   ```

4. **Solve Part 2:**
   - Task file now includes both parts
   - Implement Part 2 solution
   - Submit answer

5. **Want to solve in another language?**
   ```bash
   cd go
   ./create-day.sh 5 --fetch     # Uses cached input + Part 1 task
   cd ..
   ./.aoc-update-task.sh 5 go    # Uses cached Part 2 task (if available)
   ```

### Clear the Cache

If you need to re-fetch (e.g., to get Part 2 after solving Part 1):

```bash
# Clear specific day's Part 2 to re-fetch
rm ~/.aoc-cache/2025/day5_task_part2.html

# Clear entire day
rm ~/.aoc-cache/2025/day5_*

# Clear entire year
rm -rf ~/.aoc-cache/2025

# Clear all years
rm -rf ~/.aoc-cache
```

## Responsible Use Guidelines

Following the [Advent of Code automation guidelines](https://www.reddit.com/r/adventofcode/comments/z9dhtd/please_include_your_contact_info_in_the_useragent/):

✅ **DO:**
- Use for your personal puzzle inputs only
- Cache responses to avoid repeat requests
- Respect rate limits (3+ seconds between requests)
- Include contact info in User-Agent

❌ **DON'T:**
- Share puzzle inputs publicly
- Redistribute puzzle text/descriptions
- Make rapid-fire requests
- Use for competitive advantage on leaderboards

## References

- [AoC Automation Guidelines Discussion](https://www.reddit.com/r/adventofcode/comments/z9dhtd/please_include_your_contact_info_in_the_useragent/)
- [Monday Morning Haskell - Fetching Puzzle Input](https://mmhaskell.com/blog/2023/1/30/advent-of-code-fetching-puzzle-input-using-the-api)
- [advent-of-code-data Library](https://github.com/wimglenn/advent-of-code-data) (Python reference)

## Support

If you encounter issues:
1. Verify your session token is valid
2. Check file permissions: `ls -la ~/.aoc-session`
3. Test direct curl request (see troubleshooting)
4. Check cache directory: `ls -la ~/.aoc-cache/2025/`

Happy coding! 🎄⭐
