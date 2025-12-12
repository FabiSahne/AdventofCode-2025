using System.Diagnostics;

namespace AdventOfCode;

public static class Day07
{
    private const int Day = 7;

    private const string Test = """
                                .......S.......
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
                                ...............
                                """;

    private const ulong TestResult1 = 21;
    private const ulong TestResult2 = 40;
    private static readonly string Input = $"{Day:D2}.txt";

    private static ulong Part1(TextReader input)
    {
        var start = input.ReadLine()!;
        var startIdx = start.IndexOf('S');

        var beams = new List<bool>(start.Length);
        for (var i = 0; i < start.Length; i += 1) beams.Add(false);
        beams[startIdx] = true;

        ulong splits = 0;

        for (input.ReadLine(); input.ReadLine() is string line; input.ReadLine())
            foreach (var (c, i) in line.Select((c, i) => (c, i)))
            {
                if (c != '^' || !beams[i]) continue;

                splits += 1;
                beams[i] = false;
                beams[i - 1] = true;
                beams[i + 1] = true;
            }

        return splits;
    }

    private static ulong Part2(TextReader input)
    {
        List<List<char>> grid = [];

        while (input.ReadLine() is string line) grid.Add(line.ToList());

        var rows = grid.Count;
        var cols = grid[0].Count;
        var startIdx = grid[0].IndexOf('S');

        var timelines = new List<ulong>(cols);
        for (var i = 0; i < cols; i += 1) timelines.Add(0);

        for (var row = rows - 2; row >= 0; row -= 2)
        for (var i = 0; i < cols; i += 1)
        {
            if (grid[row][i] != '^') continue;

            var left = timelines[i - 1];
            var right = timelines[i + 1];
            timelines[i] = 1 + left + right;
        }

        return timelines[startIdx] + 1;
    }

    public static void Run()
    {
        Console.WriteLine($"\n\n=== Day {Day:D2} ===");

        Console.WriteLine(RunPart(1, TestResult1, Part1));
        Console.WriteLine(RunPart(2, TestResult2, Part2));
    }

    private static ulong RunPart(int partNumber, ulong expected, Func<TextReader, ulong> doPart)
    {
        Console.WriteLine($"\n == Part {partNumber} ==");

        TextReader reader = new StringReader(Test);
        var test = doPart(reader);
        Debug.Assert(test == expected, $"test: {test} != expected: {expected}");

        reader = new StreamReader(File.OpenRead(Input));
        return Timer.Time(() => doPart(reader));
    }
}