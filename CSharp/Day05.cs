using System.Diagnostics;

namespace AdventOfCode;

public static class Day05
{
    private const int Day = 5;

    private const string Test = """
                                3-5
                                10-14
                                16-20
                                12-18

                                1
                                5
                                8
                                11
                                17
                                32
                                """;

    private const ulong TestResult1 = 3;
    private const ulong TestResult2 = 14;
    private static readonly string Input = $"{Day:D2}.txt";

    private static ulong Part1(TextReader input)
    {
        List<Range> ranges = [];
        List<ulong> ingredients = [];

        var readRanges = false;

        while (input.ReadLine() is string line)
        {
            if (string.IsNullOrWhiteSpace(line))
            {
                readRanges = true;
                continue;
            }

            if (readRanges)
            {
                var ing = ulong.Parse(line);
                ingredients.Add(ing);
            }
            else
            {
                var parts = line.Split('-');
                var begin = ulong.Parse(parts[0]);
                var end = ulong.Parse(parts[1]);
                ranges.Add(new Range(begin, end));
            }
        }

        return (ulong)ingredients.Count(i => ranges.Any(r => r.Contains(i)));
    }

    private static ulong Part2(TextReader input)
    {
        List<Range> ranges = [];

        while (input.ReadLine() is string line)
        {
            if (string.IsNullOrWhiteSpace(line))
                break;

            var parts = line.Split('-');
            var begin = ulong.Parse(parts[0]);
            var end = ulong.Parse(parts[1]);
            ranges.Add(new Range(begin, end));
        }

        ranges.Sort((a, b) => a.End.CompareTo(b.End));

        for (var i = ranges.Count - 1; i > 0; i -= 1)
            if (ranges[i - 1].End >= ranges[i].Begin)
            {
                ranges[i - 1].End = ranges[i].End;
                ranges[i - 1].Begin = Math.Min(ranges[i - 1].Begin, ranges[i].Begin);
                ranges.RemoveAt(i);
            }

        return ranges.Aggregate<Range, ulong>(0, (current, range) => current + range.Size);
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
        Debug.Assert(test == expected);

        reader = new StreamReader(File.OpenRead(Input));
        return Timer.Time(() => doPart(reader));
    }

    private class Range(ulong begin, ulong end)
    {
        public ulong Begin { get; set; } = begin;
        public ulong End { get; set; } = end;

        public ulong Size => End - Begin + 1;

        public bool Contains(ulong val)
        {
            return val >= Begin && val <= End;
        }
    }
}