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

        var start = lines.First().IndexOf('S');
        Debug.Assert(start!=-1);

        var splits = 0;

        var lasers = new HashSet<int>();
        lasers.Add(start);

        foreach(var l in lines.Skip(1))
        {
            var nextLaser = l.IndexOf('^');
            while(nextLaser!=-1)
            {
                if(lasers.Contains(nextLaser)){
                    splits++;
                    lasers.Remove(nextLaser);
                    if(nextLaser > 0)
                        lasers.Add(nextLaser-1);
                    if(nextLaser+1 < l.Length)
                        lasers.Add(nextLaser+1);
                }

                nextLaser = l.IndexOf('^', nextLaser+1);
            }
        }

        return splits.ToString();
    }

    public static string Part2(string input)
    {

        var total = 0;
        foreach(var l in input.Split('\n').Skip(1))
        {
            var nextLaser = l.IndexOf('^');
            while(nextLaser!=-1)
            {
                if(lasers.Contains(nextLaser)){
                    total++;
                    lasers.Remove(nextLaser);
                    if(nextLaser > 0)
                        lasers.Add(nextLaser-1);
                    if(nextLaser+1 < l.Length)
                        lasers.Add(nextLaser+1);
                }
            }
        }

        return total.ToString();
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
        const string input = @".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        const string expected = "21";

        var result = Solution.Part1(input);
        if (result != expected)
            throw new Exception($"Part 1 test failed: expected '{expected}', got '{result}'");

        Console.WriteLine("✓ Part 1 test passed");
    }

    static void TestPart2()
    {
        const string input = @".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        const string expected = "40";

        var result = Solution.Part2(input);
        if (result != expected)
            throw new Exception($"Part 2 test failed: expected '{expected}', got '{result}'");

        Console.WriteLine("✓ Part 2 test passed");
    }
}
#endif
