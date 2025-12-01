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

      var pos = 50;
      var zeros = 0;

      foreach(var line in lines)
      {
        var dir = line[0];
        var dist = int.Parse(line.Substring(1));

        if(dir == 'L')
        {
          pos -= dist;
        }
        else
        {
          pos += dist;
        }
        pos = (pos + 100) % 100;

        if(pos == 0)
        {
          zeros++;
        }
      }

      return zeros.ToString();
    }

    public static string Part2(string input)
    {
      var lines = input.Split('\n');

      var pos = 50;
      var zeros = 0;

      foreach(var line in lines)
      {
        var steps = int.Parse(line.Substring(1));
        var step = line[0]=='R'?1:-1;

        while(steps > 0)
        {
          steps--;
          pos += step;

          if(pos < 0) pos += 100;
          if(pos > 99) pos -= 100;

          if(pos == 0)
          {
            zeros++;
          }
        }
      }

      return zeros.ToString();
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
        const string input = @"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        const string expected = "3";

        var result = Solution.Part1(input);
        if (result != expected)
            throw new Exception($"Part 1 test failed: expected '{expected}', got '{result}'");

        Console.WriteLine("✓ Part 1 test passed");
    }

    static void TestPart2()
    {
        const string input = @"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        const string expected = "6";

        var result = Solution.Part2(input);
        if (result != expected)
            throw new Exception($"Part 2 test failed: expected '{expected}', got '{result}'");

        Console.WriteLine("✓ Part 2 test passed");
    }
}
#endif
