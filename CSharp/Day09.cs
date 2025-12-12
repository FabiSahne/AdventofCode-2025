using System.Diagnostics;
using System.Runtime.CompilerServices;

namespace AdventOfCode;

public static class Day09
{
    private const int Day = 9;

    private const string Test = """
                                7,1
                                11,1
                                11,7
                                9,7
                                9,5
                                2,5
                                2,3
                                7,3
                                """;

    private const ulong TestResult1 = 50;
    private const ulong TestResult2 = 24;
    private static readonly string Input = $"{Day:D2}.txt";

    private static ulong Part1(TextReader input)
    {
        List<(ulong, ulong)> tiles = [];
        while (input.ReadLine() is string line)
        {
            var parts = line.Split(',').Select(ulong.Parse).ToArray();
            tiles.Add((parts[0], parts[1]));
        }

        ulong max = 0;
        for (var i = 0; i < tiles.Count; i += 1)
        for (var j = i + 1; j < tiles.Count; j += 1)
        {
            var (c11, c12) = tiles[i];
            var (c21, c22) = tiles[j];

            var size = (Math.Max(c11, c21) - Math.Min(c11, c21) + 1) * (Math.Max(c12, c22) - Math.Min(c12, c22) + 1);
            max = Math.Max(max, size);
        }

        return max;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    private static (ulong, ulong) Sorted(this (ulong, ulong) self)
    {
        return self.Item1 < self.Item2 ? self : (self.Item2, self.Item1);
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    private static bool Intersection(List<Edge> edges, ulong minX, ulong maxX, ulong minY, ulong maxY)
    {
        foreach (var edge in edges)
        {
            var (eMinX, eMaxX) = (edge.X1, edge.X2).Sorted();
            var (eMinY, eMaxY) = (edge.Y1, edge.Y2).Sorted();

            if (minX < eMaxX && maxX > eMinX && minY < eMaxY && maxY > eMinY)
                return true;
        }

        return false;
    }

    private static ulong Part2(TextReader input)
    {
        List<(ulong, ulong)> redTiles = [];
        List<Edge> edges = [];
        while (input.ReadLine() is string line)
        {
            var parts = line.Split(',').Select(ulong.Parse).ToArray();
            if (redTiles.Count > 0)
            {
                var last = redTiles.Last();
                edges.Add(new Edge(parts[0], last.Item1, parts[1], last.Item2));
            }

            redTiles.Add((parts[0], parts[1]));
        }

        edges.Add(new Edge(redTiles[0].Item1, redTiles.Last().Item1, redTiles[0].Item2, redTiles.Last().Item2));

        ulong result = 0;

        for (var i = 0; i < redTiles.Count; i += 1)
        for (var j = i + 1; j < redTiles.Count; j += 1)
        {
            var tile1 = redTiles[i];
            var tile2 = redTiles[j];
            var (minX, maxX) = (tile1.Item1, tile2.Item1).Sorted();
            var (minY, maxY) = (tile1.Item2, tile2.Item2).Sorted();

            if (Intersection(edges, minX, maxX, minY, maxY))
                continue;

            var area = (maxX - minX + 1) * (maxY - minY + 1);
            result = Math.Max(result, area);
        }

        return result;
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

    private readonly record struct Edge(ulong X1, ulong X2, ulong Y1, ulong Y2);
}