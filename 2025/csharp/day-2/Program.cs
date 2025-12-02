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
      var invalidIds = new List<long>();

      foreach(var range in input.Split(','))
      {
        var parts = range.Split('-');
        // skip if both start end end are odd length
        if(parts.First().Length == parts.Last().Length &&
          parts.First().Length%2==1) continue;

        var start = long.Parse(parts[0]);
        var end = long.Parse(parts[1]);

        for(var i = start; i <= end; i++){
          var s = i.ToString();
          if(s.Length%2==1) continue;

          var halves = s.Length/2;
          if(s.Substring(0, halves)==s.Substring(halves)){
            invalidIds.Add(i);
          }
        }
      }

      return invalidIds.Sum().ToString();
    }

    public static string Part2(string input)
    {
      var invalidIds = new List<long>();

      foreach(var range in input.Split(','))
      {
        var parts = range.Split('-');

        var start = long.Parse(parts[0]);
        var end = long.Parse(parts[1]);

        for(var i = start; i <= end; i++){
          var s = i.ToString();

          for(var j = 1; j <= s.Length/2; j++){
            if(testForSequenceLength(s, j))
            {
              Console.WriteLine($"Invalid ID: {i}");
              invalidIds.Add(i);
              break;
            }
          }
        }
      }

      return invalidIds.Sum().ToString();
    }

    private static bool testForSequenceLength(string s, int length)
    {
      var target = s.Substring(0, length);
      var remaining = s.Substring(length);
      while(true)
      {
        if(remaining.Length == 0) return true;
        if(remaining.Length < target.Length) return false;
        if(!remaining.StartsWith(target)) return false;
        remaining = remaining.Substring(target.Length);
      }
      return true;
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
        const string input = @"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        const string expected = "1227775554";

        var result = Solution.Part1(input);
        if (result != expected)
            throw new Exception($"Part 1 test failed: expected '{expected}', got '{result}'");

        Console.WriteLine("✓ Part 1 test passed");
    }

    static void TestPart2()
    {
        const string input = @"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        const string expected = "4174379265";

        var result = Solution.Part2(input);
        if (result != expected)
            throw new Exception($"Part 2 test failed: expected '{expected}', got '{result}'");

        Console.WriteLine("✓ Part 2 test passed");
    }
}
#endif
