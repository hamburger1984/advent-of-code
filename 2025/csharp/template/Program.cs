var input = File.ReadAllText("input.txt").TrimEnd();

Console.WriteLine($"Part 1: {Part1(input)}");
Console.WriteLine($"Part 2: {Part2(input)}");

static string Part1(string input)
{
    // TODO: Implement part 1
    throw new NotImplementedException();
}

static string Part2(string input)
{
    // TODO: Implement part 2
    throw new NotImplementedException();
}

// Test examples - auto-generated from task description
#if DEBUG
public static class Tests
{
    public static void RunTests()
    {
        TestPart1();
        TestPart2();
        Console.WriteLine("All tests passed!");
    }

    static void TestPart1()
    {
        const string input = @"TODO: Add example input";
        const string expected = "TODO";

        var result = Part1(input);
        if (result != expected)
            throw new Exception($"Part 1 test failed: expected '{expected}', got '{result}'");

        Console.WriteLine("✓ Part 1 test passed");
    }

    static void TestPart2()
    {
        const string input = @"TODO: Add example input";
        const string expected = "TODO";

        var result = Part2(input);
        if (result != expected)
            throw new Exception($"Part 2 test failed: expected '{expected}', got '{result}'");

        Console.WriteLine("✓ Part 2 test passed");
    }
}
#endif
