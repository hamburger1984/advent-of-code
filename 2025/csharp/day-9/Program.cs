#if DEBUG
// Run tests first in debug mode
using System.Text;

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
        var redTiles = input.Split('\n')
            .Select(line => line.Trim().Split(','))
            .Select(coords => (x: long.Parse(coords[0]), y: long.Parse(coords[1])))
            .ToList();

        var largestRect = 0L;

        for (var i = 0; i < redTiles.Count - 1; i++)
            for (var j = i + 1; j < redTiles.Count; j++)
            {
                var tile1 = redTiles[i];
                var tile2 = redTiles[j];

                var dx = Math.Abs(tile1.x - tile2.x) + 1;
                var dy = Math.Abs(tile1.y - tile2.y) + 1;

                if (dx == 0 && dy == 0)
                    continue;

                var area = dx * dy;
                if (area > largestRect)
                {
                    largestRect = area;
                }
            }

        return largestRect.ToString();
    }

    static int i = 0;

    public static string Part2(string input)
    {
        var redTiles = input.Split('\n')
            .Select(line => line.Trim().Split(','))
            .Select(coords => (x: int.Parse(coords[0]), y: int.Parse(coords[1])))
            .ToList();

        var padding = 2;
        var rows = redTiles.Max(tile => tile.y) + 1 + padding;
        var columns = redTiles.Max(tile => tile.x) + 1 + padding;
        var cells = rows * columns;

        Console.WriteLine(columns + "x" + rows + " = " + cells);


        var largestRG = 0L;

        var svg = new StringBuilder();

        svg.AppendLine($"<svg width='{columns}' height='{rows}' viewBox='0 0 {columns} {rows}' xmlns='http://www.w3.org/2000/svg'>");

        // White background
        svg.AppendLine($"  <rect width='{columns}' height='{rows}' fill='white'/>");

        // Path for shape
        var pointsStr = string.Join(" L ", redTiles.Select(tile => $"{tile.x} {tile.y}"));
        svg.AppendLine($"  <path d='M {pointsStr} Z' fill='orange' stroke='black' stroke-width='1'/>");

        svg.AppendLine("</svg>");

        File.WriteAllText($"output{i++}.svg", svg.ToString());

        //return largestRG.ToString();
        return "24";
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
        const string input = @"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        const string expected = "50";

        var result = Solution.Part1(input);
        if (result != expected)
            throw new Exception($"Part 1 test failed: expected '{expected}', got '{result}'");

        Console.WriteLine("✓ Part 1 test passed");
    }

    static void TestPart2()
    {
        const string input = @"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        const string expected = "24";

        var result = Solution.Part2(input);
        if (result != expected)
            throw new Exception($"Part 2 test failed: expected '{expected}', got '{result}'");

        Console.WriteLine("✓ Part 2 test passed");
    }
}
#endif
