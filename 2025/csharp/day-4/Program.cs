#if DEBUG
// Run tests first in debug mode
Tests.RunTests();
Console.WriteLine();
#endif

var input = File.ReadAllText("input.txt").TrimEnd();

Console.WriteLine($"Part 1: {Solution.Part1(input)}");
Console.WriteLine($"Part 2: {Solution.Part2(input)}");

static class Solution
{
    public static string Part1(string input)
    {
        var previous = string.Empty;
        var current = string.Empty;
        var next = string.Empty;

        var accessible = 0;

        foreach(var line in input.Split('\n'))
        {
            previous = current;
            current = next;
            next = line;

            if(string.IsNullOrEmpty(current))
                continue;

            var accessibleInLine = countAccessible(previous, current, next);
            //Console.WriteLine($"{line}: {accessibleInLine}");

            accessible += accessibleInLine;
        }

        previous = current;
        current = next;
        next = string.Empty;
        accessible += countAccessible(previous, current, next);

        return accessible.ToString();
    }

    private static int countAccessible(string previous, string current, string next)
    {
        var accessible = 0;

        for(var i = 0; i < current.Length; i++)
        {
            if(current[i] == '.')
                continue;

            var left = i == 0 || current[i - 1] == '.' ? 0 : 1;
            var upLeft = string.IsNullOrEmpty(previous) || i == 0 || previous[i - 1] == '.' ? 0 : 1;
            var up = string.IsNullOrEmpty(previous) || previous[i] == '.' ? 0 : 1;
            var upRight = string.IsNullOrEmpty(previous) || i == current.Length - 1 || previous[i + 1] == '.' ? 0 : 1;
            var right = i == current.Length - 1 || current[i + 1] == '.' ? 0 : 1;
            var downRight = string.IsNullOrEmpty(next) || i == current.Length - 1 || next[i + 1] == '.' ? 0 : 1;
            var down = string.IsNullOrEmpty(next) || next[i] == '.' ? 0 : 1;
            var downLeft = string.IsNullOrEmpty(next) || i == 0 || next[i - 1] == '.' ? 0 : 1;

            var neighborRolls = left + upLeft + up + upRight + right + downRight + down + downLeft;
            accessible += neighborRolls < 4 ? 1: 0;
        }

        return accessible;
    }

    public static string Part2(string input)
    {
        var grid = input.Split('\n').Select(line => line.Trim().ToCharArray()).ToArray();

        var totalRemoved = 0;
        var removed = 0;

        do
        {
            removed = 0;
            var accessible = new Dictionary<int, int[]>();
            for(var row = 0; row < grid.Length; row++)
            {
                var rowAccessible = accessibleIndices(grid, row);
                if(rowAccessible.Length > 0)
                {
                    accessible[row] = rowAccessible;
                }
            }

            foreach(var kvp in accessible)
            {
                var row = kvp.Key;
                var indices = kvp.Value;
                foreach(var index in indices)
                {
                    grid[row][index] = 'x';
                    removed++;
                }
            }
            totalRemoved += removed;

            Console.WriteLine($"Removed: {removed} Total removed: {totalRemoved}");
            Console.WriteLine($"=====");
        } while(removed > 0);

        return totalRemoved.ToString();
    }

    private static int[] accessibleIndices(char[][] grid, int rowIndex){
        var width = grid[rowIndex].Length;
        var height = grid.Length;

        var accessible = new List<int>();
        for(var i = 0; i < width; i++)
        {
            if(grid[rowIndex][i] != '@')
                continue;

            var neighbors = 0;

            if(i > 0){
                // left
                if(grid[rowIndex][i - 1]=='@') neighbors++;

                // up left
                if(rowIndex> 0 && grid[rowIndex - 1][i - 1]=='@') neighbors++;

                // down left
                if(rowIndex < height - 1 && grid[rowIndex + 1][i - 1]=='@') neighbors++;
            }

            if(i < width - 1){
                // right
                if(grid[rowIndex][i + 1]=='@') neighbors++;

                // up right
                if(rowIndex > 0 && grid[rowIndex - 1][i + 1]=='@') neighbors++;

                // down right
                if(rowIndex < height - 1 && grid[rowIndex + 1][i + 1]=='@') neighbors++;
            }

            // up
            if(rowIndex > 0  && grid[rowIndex - 1][i ]=='@') neighbors++;

            // down
            if(rowIndex < height-1  && grid[rowIndex + 1][i ]=='@') neighbors++;

            if (neighbors < 4) accessible.Add(i);
        }

        return accessible.ToArray();
    }
}

// Test examples - auto-generated from task description
#if DEBUG
static class Tests
{
    public static void RunTests()
    {
        TestPart1();
        TestPart2();
        Console.WriteLine("All tests passed!");
    }

    static void TestPart1()
    {
        const string input = @"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        const string expected = "13";

        var result = Solution.Part1(input);
        if (result != expected)
            throw new Exception($"Part 1 test failed: expected '{expected}', got '{result}'");

        Console.WriteLine("✓ Part 1 test passed");
    }

    static void TestPart2()
    {
        const string input = @"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        const string expected = "43";

        var result = Solution.Part2(input);
        if (result != expected)
            throw new Exception($"Part 2 test failed: expected '{expected}', got '{result}'");

        Console.WriteLine("✓ Part 2 test passed");
    }
}
#endif
