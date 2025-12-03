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
        long sum = 0;
        foreach(var line in input.Split('\n'))
        {
            if(!line.All(c => c >= '0' && c <= '9'))
            {
                Console.WriteLine($"Invalid input: {line}");
                continue;
            }

            var maxChar = '0';
            var maxCharIndex = -1;
            for(var i = 0; i < line.Length-1; i++)
            {
                if(line[i] > maxChar)
                {
                    maxChar = line[i];
                    maxCharIndex = i;
                }
                if(maxChar == '9') break;
            }

            var maxChar2 = '0';
            var maxChar2Index = -1;
            for(var i = line.Length-1; i > maxCharIndex; i--)
            {
                if(line[i] > maxChar2)
                {
                    maxChar2 = line[i];
                    maxChar2Index = i;
                }
                if(maxChar2 == '9') break;
            }

            Debug.Assert(maxChar != '0');
            Debug.Assert(maxChar2 != '0');

            Debug.Assert(maxCharIndex > -1);
            Debug.Assert(maxChar2Index > -1);
            Debug.Assert(maxCharIndex < line.Length);
            Debug.Assert(maxChar2Index < line.Length);
            Debug.Assert(maxCharIndex != maxChar2Index);
            Debug.Assert(maxCharIndex < maxChar2Index);

            //Console.WriteLine(line + $" {maxChar - '0'}{maxChar2 - '0'}");

            sum += (maxChar - '0') * 10 + (maxChar2 - '0');
        }
        return sum.ToString();
    }

    public static string Part2(string input)
    {
        long sum = 0;
        foreach(var line in input.Split('\n'))
        {
            if(!line.All(c => c >= '0' && c <= '9'))
            {
                Console.WriteLine($"Invalid input: {line}");
                continue;
            }

            var chars = findLargestChars(line, 12);

            long n = 0;
            foreach(var c in chars){
                n = n * 10 + (c - '0');
            }
            sum += n;

            Console.WriteLine($"Line: {line}, Number: {n}");
        }
        return sum.ToString();
    }

    private static char[] findLargestChars(string line, int number)
    {
        var maxChars = new char[number];
        var maxCharIndices = new int[number];
        var maxChar2Indices = new int[number];

        var cursor = 0;
        // for each digit we want to find
        for(var i = 0; i < number; i++)
        {
            var remainingDigits = number-i-1;

            // continue from where we left of, stop so we still have enough left for the rest of the digits
            for(var j = cursor; j < line.Length-remainingDigits; j++){
                //Console.WriteLine($"Checking char {line[j]} at index {j}, comparing to {maxChars[i]}");
                if(line[j] > maxChars[i]){
                    maxChars[i] = line[j];
                    maxCharIndices[i] = j;
                    if(maxChars[i] == '9') break;
                }
            }

            cursor = maxCharIndices[i]+1;
            //Console.WriteLine($"Max char for digit {i+1}: {maxChars[i]} at index {maxCharIndices[i]}, cursor {cursor}");
        }

        return maxChars;
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
        const string input = @"987654321111111
811111111111119
234234234234278
818181911112111";
        const string expected = "357";

        var result = Solution.Part1(input);
        if (result != expected)
            throw new Exception($"Part 1 test failed: expected '{expected}', got '{result}'");

        Console.WriteLine("✓ Part 1 test passed");
    }

    static void TestPart2()
    {
        const string input = @"987654321111111
811111111111119
234234234234278
818181911112111";
        const string expected = "3121910778619";

        var result = Solution.Part2(input);
        if (result != expected)
            throw new Exception($"Part 2 test failed: expected '{expected}', got '{result}'");

        Console.WriteLine("✓ Part 2 test passed");
    }
}
#endif
