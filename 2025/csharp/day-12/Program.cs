#if DEBUG
// Run tests first in debug mode
Tests.RunTests();
Console.WriteLine();
#endif

var input = File.ReadAllText("input.txt").TrimEnd();

Console.WriteLine($"Part 1: {Solution.Part1(input)}");
Console.WriteLine($"Part 2: {Solution.Part2(input)}");

// Shape orientation with normalized coordinates
readonly record struct ShapeOrientation
{
    public readonly (int x, int y)[] Cells;
    public readonly int Width;
    public readonly int Height;

    public ShapeOrientation((int x, int y)[] cells)
    {
        Cells = cells;
        Width = cells.Length > 0 ? cells.Max(c => c.x) + 1 : 0;
        Height = cells.Length > 0 ? cells.Max(c => c.y) + 1 : 0;
    }
}

// Preprocessed shape with all unique orientations
class Shape
{
    public ushort Original;
    public ShapeOrientation[] Orientations = null!;
    public int CellCount;
}

// Grid for collision detection and placement
class Grid
{
    private readonly bool[,] occupied;
    public readonly int Width;
    public readonly int Height;
    private int filledCells;

    public Grid(int width, int height)
    {
        Width = width;
        Height = height;
        occupied = new bool[height, width];
        filledCells = 0;
    }

    public int RemainingCells => (Width * Height) - filledCells;

    public bool CanPlace(ShapeOrientation shape, int posX, int posY)
    {
        foreach (var (dx, dy) in shape.Cells)
        {
            int x = posX + dx;
            int y = posY + dy;

            if (x < 0 || x >= Width || y < 0 || y >= Height)
                return false;

            if (occupied[y, x])
                return false;
        }

        return true;
    }

    public void Place(ShapeOrientation shape, int posX, int posY)
    {
        foreach (var (dx, dy) in shape.Cells)
        {
            int x = posX + dx;
            int y = posY + dy;
            occupied[y, x] = true;
            filledCells++;
        }
    }

    public void Remove(ShapeOrientation shape, int posX, int posY)
    {
        foreach (var (dx, dy) in shape.Cells)
        {
            int x = posX + dx;
            int y = posY + dy;
            occupied[y, x] = false;
            filledCells--;
        }
    }
}

static class Solution
{
    private static readonly char[] regionSeparators = new char[] { ' ', ':', 'x' };
    private static Shape[] preprocessedShapes = null!;

    public static string Part1(string input)
    {
        var lines = input.Split('\n');
        var l = 0;
        var rawShapes = new List<ushort>();

        // Parse shapes
        while (l < lines.Length && lines[l].StartsWith(rawShapes.Count + ":"))
        {
            l++; // skip shape number
            ushort shape = 0;
            for (int i = 0; i < 3; i++)
            {
                var shapeline = lines[l + i];
                foreach (var c in shapeline)
                {
                    shape <<= 1;
                    shape |= (ushort)(c == '#' ? 1 : 0);
                }
            }
            rawShapes.Add(shape);
            l += 3;

            // Skip blank line between shapes if present
            if (l < lines.Length && string.IsNullOrWhiteSpace(lines[l]))
                l++;
        }

        // Parse regions
        var regions = new List<(uint width, uint height, uint[] shapes)>();
        while (l < lines.Length)
        {
            if (string.IsNullOrWhiteSpace(lines[l]))
            {
                l++;
                continue;
            }

            var parts = lines[l].Split(regionSeparators, StringSplitOptions.RemoveEmptyEntries);
            if (parts.Length < 2)
            {
                l++;
                continue;
            }

            var width = uint.Parse(parts[0]);
            var height = uint.Parse(parts[1]);
            var regionShapes = new uint[parts.Length - 2];
            for (int i = 0; i < regionShapes.Length; i++)
            {
                regionShapes[i] = uint.Parse(parts[i + 2]);
            }
            regions.Add((width, height, regionShapes));
            l++;
        }

        // Preprocess shapes once
        preprocessedShapes = PreprocessShapes(rawShapes);

#if DEBUG
        Console.WriteLine($"Preprocessed {preprocessedShapes.Length} shapes");
        for (int i = 0; i < preprocessedShapes.Length; i++)
        {
            Console.WriteLine($"Shape {i}: {preprocessedShapes[i].CellCount} cells, {preprocessedShapes[i].Orientations.Length} orientations");
        }
        Console.WriteLine();
#endif

        // Solve each region
        int solvableCount = 0;
        for (int i = 0; i < regions.Count; i++)
        {
            var (width, height, requirements) = regions[i];
            var grid = new Grid((int)width, (int)height);

#if DEBUG
            var sw = System.Diagnostics.Stopwatch.StartNew();
#endif
            bool canFit = CanFitAllShapes(grid, preprocessedShapes, requirements);
#if DEBUG
            Console.WriteLine($"Region {i}: {width}x{height}, shapes [{string.Join(",", requirements)}] - {(canFit ? "SOLVABLE" : "IMPOSSIBLE")} ({sw.ElapsedMilliseconds}ms)");
#endif

            if (canFit)
                solvableCount++;
        }

        return solvableCount.ToString();
    }

    private static Shape[] PreprocessShapes(List<ushort> rawShapes)
    {
        var shapes = new Shape[rawShapes.Count];

        for (int i = 0; i < rawShapes.Count; i++)
        {
            var cells = UshortToCells(rawShapes[i]);
            var orientations = GenerateUniqueOrientations(cells);

            shapes[i] = new Shape
            {
                Original = rawShapes[i],
                Orientations = orientations,
                CellCount = cells.Length
            };
        }

        return shapes;
    }

    private static (int x, int y)[] UshortToCells(ushort shape)
    {
        var cells = new List<(int, int)>();

        // Extract only # cells from 3x3 grid (MSB first, row-major)
        for (int row = 0; row < 3; row++)
        {
            for (int col = 0; col < 3; col++)
            {
                int bitPosition = row * 3 + col;
                if ((shape & (1 << (8 - bitPosition))) != 0)
                {
                    cells.Add((col, row));
                }
            }
        }

        return [.. cells];
    }

    private static ShapeOrientation[] GenerateUniqueOrientations((int x, int y)[] cells)
    {
        var unique = new HashSet<string>();
        var orientations = new List<ShapeOrientation>();

        // Try all 8 combinations (4 rotations × 2 flips)
        for (int flip = 0; flip < 2; flip++)
        {
            var current = flip == 0 ? cells : FlipHorizontal(cells);

            for (int rotation = 0; rotation < 4; rotation++)
            {
                var transformed = Rotate90(current, rotation);
                var normalized = Normalize(transformed);

                // Create unique key for deduplication
                var key = string.Join(";", normalized.Select(c => $"{c.Item1},{c.Item2}"));

                if (unique.Add(key))
                {
                    orientations.Add(new ShapeOrientation(normalized));
                }
            }
        }

        return [.. orientations];
    }

    private static (int x, int y)[] Rotate90((int x, int y)[] cells, int times)
    {
        var result = cells;
        for (int i = 0; i < times; i++)
        {
            // Rotate 90° clockwise: (x, y) → (-y, x)
            result = Normalize([.. result.Select(c => (-c.y, c.x))]);
        }
        return result;
    }

    private static (int x, int y)[] FlipHorizontal((int x, int y)[] cells)
    {
        // Flip across Y axis: (x, y) → (-x, y)
        return Normalize([.. cells.Select(c => (-c.x, c.y))]);
    }

    private static (int x, int y)[] Normalize((int x, int y)[] cells)
    {
        if (cells.Length == 0) return cells;

        int minX = cells.Min(c => c.x);
        int minY = cells.Min(c => c.y);

        return [.. cells.Select(c => (x: c.x - minX, y: c.y - minY))
                        .OrderBy(c => c.y)
                        .ThenBy(c => c.x)];
    }

    private static bool CanFitAllShapes(Grid grid, Shape[] shapes, uint[] requirements)
    {
        // Build queue of shape indices to place
        var shapeQueue = new List<int>();

        for (int i = 0; i < requirements.Length; i++)
        {
            for (int count = 0; count < requirements[i]; count++)
            {
                shapeQueue.Add(i);
            }
        }

        // Early pruning: check if total cells fit
        int totalCells = shapeQueue.Sum(idx => shapes[idx].CellCount);
        if (totalCells > grid.Width * grid.Height)
            return false;

        // Sort: largest shapes first (heuristic)
        shapeQueue.Sort((a, b) => shapes[b].CellCount.CompareTo(shapes[a].CellCount));

        return Backtrack(grid, shapes, shapeQueue, 0);
    }

    private static bool Backtrack(Grid grid, Shape[] shapes, List<int> queue, int index)
    {
        // Base case: all shapes placed successfully
        if (index >= queue.Count)
            return true;

        // Pruning: check if remaining shapes can possibly fit
        int remainingCells = 0;
        for (int i = index; i < queue.Count; i++)
            remainingCells += shapes[queue[i]].CellCount;

        if (remainingCells > grid.RemainingCells)
            return false;

        var shape = shapes[queue[index]];

        // Try each orientation
        foreach (var orientation in shape.Orientations)
        {
            // Try each position
            for (int y = 0; y <= grid.Height - orientation.Height; y++)
            {
                for (int x = 0; x <= grid.Width - orientation.Width; x++)
                {
                    if (grid.CanPlace(orientation, x, y))
                    {
                        grid.Place(orientation, x, y);

                        if (Backtrack(grid, shapes, queue, index + 1))
                            return true;

                        grid.Remove(orientation, x, y);
                    }
                }
            }
        }

        return false;
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
        // TestPart2();
        Console.WriteLine("All tests passed!");
    }

    static void TestPart1()
    {
        const string input = @"0:
###
##.
##.
1:
###
##.
.##
2:
.##
###
##.
3:
##.
###
##.
4:
###
#..
###
5:
###
.#.
###
4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";
        const string expected = "2";

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
