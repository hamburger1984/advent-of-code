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
        var lines = input.Trim('\n', '\r').Split(Environment.NewLine);
        var operatorsLine = lines.Last();
        var columnLeftIdx = AllIndices(operatorsLine.ToCharArray(), c => c != ' ').ToArray();
        //Console.WriteLine($"Operators: {string.Join(", ", columnLeftIdx)}, Length: {operatorsLine.Length}");
        var operators = columnLeftIdx.Select(i => operatorsLine[i]).ToArray();
        var columnRightIdx = columnLeftIdx.Skip(1).Select(i => i - 2).Append(lines.Max(l => l.Length) - 1).ToArray();

        var operands = lines.Take(lines.Length - 1).ToArray();

        Debug.Assert(columnLeftIdx.Length == columnRightIdx.Length);

        var total = 0L;
        for (var i = 0; i < columnLeftIdx.Length; i++)
        {
            var op = operators[i];
            //Console.WriteLine($"Column {i}: {op}, {columnLeftIdx[i]}-{columnRightIdx[i]}");

            var values = new Dictionary<int, long>();
            for (var c = columnLeftIdx[i]; c <= columnRightIdx[i]; c++)
            {
                values[c] = 0;
                for (var o = 0; o < operands.Length; o++)
                {
                    var l = operands[o];
                    var opC = c >= l.Length ? ' ' : l[c];
                    if (opC == ' ')
                        continue;
                    values[c] = values[c] * 10 + opC - '0';
                }
            }

            var seed = op == '+' ? 0L : 1L;
            var v = values.Values.Aggregate(seed, (acc, val) =>
                op == '+' ? acc + val : acc * val
            );
            //Console.WriteLine($"Values: {string.Join(", ", values)} -> {v}");

            total += v;
        }

        return total.ToString();
    }

    private static int[] AllIndices<T>(T[] array, Func<T, bool> predicate)
    {
        return array.Select((value, index) => predicate(value) ? index : -1).Where(i => i != -1).ToArray();
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
        const string input = @"123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +";
        const string expected = "3263827";

        var result = Solution.Part2(input);
        if (result != expected)
            throw new Exception($"Part 2 test failed: expected '{expected}', got '{result}'");

        Console.WriteLine("✓ Part 2 test passed");
    }
}
#endif
