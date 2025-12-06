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

        //Console.WriteLine($"Converted ranges: {string.Join(", ", converted.Select(r => $"[{r[0]}, {r[1]}]"))}");

        var spoiled = 0L;
        var fresh = 0L;

        foreach(var line in lines.Skip(converted.Length+1)){
                var id = long.Parse(line);
                var f = false;

                foreach (var range in converted)
                {
                    if (range[0] <= id && id <= range[1])
                    {
                        f = true;
                        //Console.WriteLine($"ID {id} in range [{range[0]}, {range[1]}]");
                        break;
                    }
                }

                //if(!f) Console.WriteLine($"ID {id} --- no range");

                if(f) fresh++;
                else spoiled++;
        }

        Console.WriteLine($"Spoiled: {spoiled}, Fresh: {fresh}, Ranges: {converted.Length}, Lines: {lines.Length}");
        Debug.Assert(lines.Length == spoiled + fresh + converted.Length + 1 /* empty line */);

        return fresh.ToString();
    }

    public static string Part2(string input)
    {
        var lines = input.Split('\n');
        var ranges = lines.TakeWhile(line => line.Trim().Length > 0).ToArray();

        var ordered = ranges.Select(r =>
            r.Trim().Split('-').Select(long.Parse).ToArray()
        ).OrderBy(r => r[0]).ThenBy(r => r[1]).ToList();

        var csvContent = DumpToCsv(ordered);
        File.WriteAllText("ranges.csv", csvContent);

        for(var i = 0; i < ordered.Count - 1; i++){
            var current = ordered[i];
            var next = ordered[i + 1];

            if(current[1] >= next[0]){
                //Console.WriteLine($"Merging {current[0]}-{current[1]} and {next[0]}-{next[1]}");
                ordered[i] = new long[]{current[0], Math.Max(current[1], next[1])};
                ordered.RemoveAt(i + 1);
                i--;
            }
        }

        var mergedCsvContent = DumpToCsv(ordered);
        File.WriteAllText("merged-ranges.csv", mergedCsvContent);

        var total = 0L;

        foreach(var range in ordered){
            total += range[1] - range[0] + 1;
        }

        Console.WriteLine($"Total: {total}, Ranges: {ordered.Count}");


        return total.ToString();
    }

    private static string DumpToCsv(List<long[]> ranges)
    {
        var header = "Start,End";
        var body = string.Join("\n", ranges.Select(r => $"{r[0]},{r[1]}"));
        return $"{header}\n{body}";
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
        const string expected = "14";

        var result = Solution.Part2(input);
        if (result != expected)
            throw new Exception($"Part 2 test failed: expected '{expected}', got '{result}'");

        Console.WriteLine("✓ Part 2 test passed");
    }
}
#endif
