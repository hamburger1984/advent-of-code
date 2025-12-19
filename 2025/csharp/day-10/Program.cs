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
        var presses = 0L;
        foreach (var l in input.Split('\n'))
        {
            Console.WriteLine(l);

            var parts = l.Split(' ', 2);

            // the target bitmask we want to achieve
            var target = parts[0].Trim('[').Trim(']').Aggregate(0, (acc, c) => acc * 2 + (c == '#' ? 1 : 0));
            Console.WriteLine(parts.First() + " -> " + target + " -> " + target.ToString("B"));

            parts = parts[1].Split(' ');

            // masks of bits the buttons each toggle
            var buttons = parts.Take(parts.Length - 1).Select(b => b.Trim('(', ')').Split(',').Select(int.Parse).Aggregate(0, (acc, i) => acc + (1 << i))).ToArray();
            Console.WriteLine(string.Join(' ', buttons.Select(i => i.ToString("B"))));


            // Ignore for now
            var joltage = parts.Last();
            Console.WriteLine(joltage);

            presses += countPresses(target, buttons);

        }

        return presses.ToString();
    }

    /// <summary>
    /// Counts the number of button presses required to achieve the target bitmask.
    /// </summary>
    /// <param name="target">The target bitmask.</param>
    /// <param name="buttons">The masks of bits the buttons each toggle.</param>
    /// <returns>The number of button presses required.</returns>
    private static long countPresses(int target, int[] buttons)
    {
        int maxBit = Math.Max(31 - int.LeadingZeroCount(target), buttons.Max(b => 31 - int.LeadingZeroCount(b)));
        int numBits = maxBit + 1;
        int numButtons = buttons.Length;

        throw new NotImplementedException();
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
        TestPart2();
        Console.WriteLine("All tests passed!");
    }

    static void TestPart1()
    {
        const string input = @"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        const string expected = "7";

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
