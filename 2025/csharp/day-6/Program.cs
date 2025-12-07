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
        var operands = lines.Take(lines.Length - 1).Select(line => line.Trim().Split(' ', StringSplitOptions.RemoveEmptyEntries).Select(long.Parse).ToArray()).ToArray();
        var operations = lines.Last().Trim().Split(' ', StringSplitOptions.RemoveEmptyEntries).Select(l => l[0]).ToArray();
        var sum = 0L;
        for (var i = 0; i < operations.Length; i++)
        {
            var op = operations[i];
            var res = operands.Select(o => o[i]).Aggregate((a, b) => op switch
            {
                '+' => a + b,
                '*' => a * b,
                _ => throw new InvalidOperationException($"Unknown operation '{op}'")
            });

            sum += res;
        }

        return sum.ToString();
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
        const string input = @"123 328  51 64
45 64  387 23
6 98  215 314
*   +   *   +";
        const string expected = "4277556";

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
