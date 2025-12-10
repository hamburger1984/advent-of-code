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
        var vectors = new List<(long, long, long)>();

        foreach(var l in input.Trim().Split('\n'))
        {
            var parts = l.Trim().Split(',');
            vectors.Add((long.Parse(parts[0]), long.Parse(parts[1]), long.Parse(parts[2])));
        }

        var distances = new List<(int, int, double)>();

        for(var i = 0; i < vectors.Count-1; i++)
        {
            for(var j = i + 1; j < vectors.Count; j++)
            {
                var (x1, y1, z1) = vectors[i];
                var (x2, y2, z2) = vectors[j];

                var distance = Math.Sqrt((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1) + (z2 - z1) * (z2 - z1));

                distances.Add((i, j, distance));
            }
        }

        var closest = vectors.Count() > 30? 1000: 10;
        var connected = new List<(int,int)>();

        while(closest-- > 0)
        {
            var min = distances.MinBy(d => d.Item3);
            Console.WriteLine($"Closest pair: {min.Item1}={vectors[min.Item1]} and {min.Item2}={vectors[min.Item2]} with distance {min.Item3}");
            connected.Add((min.Item1, min.Item2));

            distances.Remove(min);
        }

        connected.Sort();
        Console.WriteLine($"Connected pairs: {string.Join(", ", connected)}");

        var circuits = new List<HashSet<int>>();

        while(connected.Any())
        {
            var circuit = new HashSet<int>();
            var current = connected.First();
            circuit.Add(current.Item1);
            circuit.Add(current.Item2);
            connected.Remove(current);

            while(connected.Any())
            {
                var next = connected.FirstOrDefault(c => circuit.Contains(c.Item1) || circuit.Contains(c.Item2));
                if(next.Item1==next.Item2) break;
                circuit.Add(next.Item1);
                circuit.Add(next.Item2);
                connected.Remove(next);
            }

            circuits.Add(circuit);
        }

        var largest = circuits.OrderByDescending(c => c.Count).Take(3).ToArray();

        return largest.Aggregate(1, (acc, curr) => acc * curr.Count).ToString();
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
        const string input = @"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        const string expected = "40";

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
