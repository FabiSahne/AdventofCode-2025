using System.Diagnostics;

namespace AdventOfCode;

public static class Day11
{
    private const int Day = 11;

    private const ulong TestResult1 = 5;
    private const ulong TestResult2 = 2;

    private static readonly string[] Tests =
    [
        """
        aaa: you hhh
        you: bbb ccc
        bbb: ddd eee
        ccc: ddd eee fff
        ddd: ggg
        eee: out
        fff: out
        ggg: out
        hhh: ccc fff iii
        iii: out
        """,
        """
        svr: aaa bbb
        aaa: fft
        fft: ccc
        bbb: tty
        tty: ccc
        ccc: ddd eee
        ddd: hub
        hub: fff
        eee: dac
        dac: fff
        fff: ggg hhh
        ggg: out
        hhh: out
        """
    ];

    private static readonly string Input = $"{Day:D2}.txt";

    private static ulong Part1(TextReader input)
    {
        var map = new Dictionary<string, string[]>();

        while (input.ReadLine() is string line)
        {
            var parts = line.Split(": ");
            var device = parts[0];
            var outputs = parts[1].Split(' ');
            map.Add(device, outputs);
        }

        var queue = new Queue<string>();
        foreach (var output in map["you"])
            queue.Enqueue(output);

        ulong count = 0;
        while (queue.Count > 0)
        {
            var next = queue.Dequeue();

            if (next == "out")
                count += 1;
            else
                foreach (var output in map[next])
                    queue.Enqueue(output);
        }

        return count;
    }

    private static ulong CountPaths(string device, bool dac, bool fft, Dictionary<string, string[]> map,
        Dictionary<(string, bool, bool), ulong> cache)
    {
        if (cache.TryGetValue((device, dac, fft), out var n))
            return n;

        if (device == "out")
            return Convert.ToUInt64(dac && fft);

        var nDac = dac || device == "dac";
        var nFft = fft || device == "fft";

        n += map[device].Select(dev => CountPaths(dev, nDac, nFft, map, cache)).Aggregate((a, b) => a + b);

        cache.Add((device, dac, fft), n);
        return n;
    }

    private static ulong Part2(TextReader input)
    {
        var map = new Dictionary<string, string[]>();

        while (input.ReadLine() is string line)
        {
            var parts = line.Split(' ');
            map.Add(parts[0][..3], parts[1..]);
        }

        return CountPaths("svr", false, false, map, new Dictionary<(string, bool, bool), ulong>());
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

        TextReader reader = new StringReader(Tests[partNumber - 1]);
        var test = doPart(reader);
        Debug.Assert(test == expected, $"test: {test} != expected: {expected}");
        Debug.WriteLine("Test success!");

        reader = new StreamReader(File.OpenRead(Input));
        return Timer.Time(() => doPart(reader));
    }
}