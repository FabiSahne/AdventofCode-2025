using System.Diagnostics;

namespace AdventOfCode;

public static class Day04
{
    private const int Day = 4;

    private const string Test = """
                                ..@@.@@@@.
                                @@@.@.@.@@
                                @@@@@.@.@@
                                @.@@@@..@.
                                @@.@@@@.@@
                                .@@@@@@@.@
                                .@.@.@.@@@
                                @.@@@.@@@@
                                .@@@@@@@@.
                                @.@.@@@.@.
                                """;

    private const ulong TestResult1 = 13;
    private const ulong TestResult2 = 43;
    private static readonly string Input = $"{Day:D2}.txt";

    private static readonly int[][] Dirs = [[-1, 0], [-1, 1], [0, 1], [1, 1], [1, 0], [1, -1], [0, -1], [-1, -1]];

    private static ulong DoPart1(TextReader input)
    {
        ulong sum = 0;

        var grid = new List<List<char>>();
        while (input.ReadLine() is string line) grid.Add(line.ToList());

        var height = grid.Count;
        var width = grid[0].Count;

        for (var y = 0; y < height; y += 1)
        for (var x = 0; x < width; x += 1)
        {
            if (grid[y][x] != '@')
                continue;

            var count = 0;
            foreach (var dir in Dirs)
            {
                var (dx, dy) = (x + dir[0], y + dir[1]);
                if (dx >= 0 && dx < width && dy >= 0 && dy < height && grid[dy][dx] == '@')
                    count += 1;
            }

            if (count < 4)
                sum += 1;
        }

        return sum;
    }

    private static ulong DoPart2(TextReader input)
    {
        ulong total = 0;

        IList<IList<char>> grid = [];
        while (input.ReadLine() is string line) grid.Add(line.ToList());

        var height = grid.Count;
        var width = grid[0].Count;

        ulong removed;
        do
        {
            removed = 0;

            for (var y = 0; y < height; y += 1)
            for (var x = 0; x < width; x += 1)
            {
                if (grid[y][x] != '@')
                    continue;

                var count = 0;
                foreach (var dir in Dirs)
                {
                    var (dx, dy) = (x + dir[0], y + dir[1]);
                    if (dx >= 0 && dx < width && dy >= 0 && dy < height && grid[dy][dx] == '@')
                        count += 1;
                }

                if (count >= 4)
                    continue;

                grid[y][x] = 'x';
                removed += 1;
            }

            total += removed;
        } while (removed > 0);

        return total;
    }

    public static void Run()
    {
        Console.WriteLine($"\n\n=== Day {Day:D2} ===");

        Console.WriteLine(Part(1, TestResult1, DoPart1));
        Console.WriteLine(Part(2, TestResult2, DoPart2));
    }

    private static ulong Part(int partNumber, ulong expected, Func<TextReader, ulong> doPart)
    {
        Console.WriteLine($"\n == Part {partNumber} ==");

        TextReader reader = new StringReader(Test);
        var test = doPart(reader);
        Debug.Assert(test == expected);

        reader = new StreamReader(File.OpenRead(Input));
        return Timer.Time(() => doPart(reader));
    }
}