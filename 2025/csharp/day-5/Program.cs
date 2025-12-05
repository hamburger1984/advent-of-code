using System.Diagnostics;

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
        var lines = input.Split('\n');
        var ranges = lines.TakeWhile(line => line.Trim().Length > 0).ToArray();

        var converted = ranges.Select(r =>
            r.Trim().Split('-').Select(long.Parse).ToArray()
        ).ToArray();

        Console.WriteLine($"Converted ranges: {string.Join(", ", converted.Select(r => $"[{r[0]}, {r[1]}]"))}");

        var spoiled = 0l;
        var fresh = 0l;

        foreach(var line in lines.Skip(converted.Length+1)){
                var id = long.Parse(line);
                var f = false;

                foreach (var range in converted)
                {
                    if (range[0] <= id && id <= range[1])
                    {
                        f = true;
                        Console.WriteLine($"ID {id} in range [{range[0]}, {range[1]}]");
                        break;
                    }
                }

                if(!f) Console.WriteLine($"ID {id} --- no range");

                if(f) fresh++;
                else spoiled++;
        }

        Console.WriteLine($"Spoiled: {spoiled}, Fresh: {fresh}, Ranges: {converted.Length}, Lines: {lines.Length}");
        Debug.Assert(lines.Length == spoiled + fresh + converted.Length + 1 /* empty line */);

        return fresh.ToString();
    }

    public static string Part2(string input)
    {
        // TODO: Implement part 2
        throw new NotImplementedException();
    }
}

// Test examples - auto-generated from task description
#if DEBUG
static class Tests
{
    public static void RunTests()
    {
        TestPart1();
        //TestPart2();
        Console.WriteLine("All tests passed!");
    }

    static void TestPart1()
    {
        const string input = @"3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        const string expected = "3";

        var result = Solution.Part1(input);
        if (result != expected)
            throw new Exception($"Part 1 test failed: expected '{expected}', got '{result}'");

        Console.WriteLine("✓ Part 1 test passed");
    }

    static void TestPart2()
    {
        const string input = @"TODO: Add example input";
        const string expected = "TODO";

        var result = Solution.Part2(input);
        if (result != expected)
            throw new Exception($"Part 2 test failed: expected '{expected}', got '{result}'");

        Console.WriteLine("✓ Part 2 test passed");
    }
}
#endif
