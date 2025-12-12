using System.Diagnostics;

namespace AdventOfCode;

public static class Day08
{
    private const int Day = 8;

    private const string Test = """
                                162,817,812
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
                                425,690,689
                                """;

    private const ulong TestResult1 = 40;
    private const ulong TestResult2 = 25272;
    private static readonly string Input = $"{Day:D2}.txt";

    private static bool _isTest;

    private static ulong Part1(TextReader input)
    {
        var numberToConnect = _isTest ? 10 : 1000;

        List<Vec3> boxes = [];
        while (input.ReadLine() is string line)
        {
            var parts = line.Split(',').Select(ulong.Parse).ToList();
            boxes.Add(new Vec3(parts[0], parts[1], parts[2]));
        }

        var pairs = new PriorityQueue<(Vec3, Vec3), ulong>();

        for (var i = 0; i < boxes.Count; i += 1)
        for (var j = i + 1; j < boxes.Count; j += 1)
            pairs.Enqueue((boxes[i], boxes[j]), Vec3.DistanceSquared(boxes[i], boxes[j]));

        var circuits = boxes.Select(b => new HashSet<Vec3> { b }).ToList();

        for (var taken = 0; taken < numberToConnect; taken += 1)
        {
            var (box1, box2) = pairs.Dequeue();

            var c1 = circuits.FirstOrDefault(c => c.Contains(box1));
            var c2 = circuits.FirstOrDefault(c => c.Contains(box2));

            Debug.Assert(c1 is not null && c2 is not null);

            if (c1 == c2) continue;

            circuits.Remove(c2);
            foreach (var box in c2)
                c1.Add(box);
        }

        return circuits
            .Select(a => (ulong)a.Count)
            .OrderDescending()
            .Take(3)
            .Aggregate((prod, len) => prod * len);
    }

    private static ulong Part2(TextReader input)
    {
        List<Vec3> boxes = [];
        while (input.ReadLine() is string line)
        {
            var parts = line.Split(',').Select(ulong.Parse).ToList();
            boxes.Add(new Vec3(parts[0], parts[1], parts[2]));
        }

        var pairs = new PriorityQueue<(Vec3, Vec3), ulong>();

        for (var i = 0; i < boxes.Count; i += 1)
        for (var j = i + 1; j < boxes.Count; j += 1)
            pairs.Enqueue((boxes[i], boxes[j]), Vec3.DistanceSquared(boxes[i], boxes[j]));

        var circuits = boxes.Select(b => new HashSet<Vec3> { b }).ToList();

        while (circuits.Count > 1)
        {
            var (box1, box2) = pairs.Dequeue();

            var c1 = circuits.FirstOrDefault(c => c.Contains(box1));
            var c2 = circuits.FirstOrDefault(c => c.Contains(box2));

            Debug.Assert(c1 is not null && c2 is not null);

            if (c1 == c2)
                continue;

            circuits.Remove(c2);
            foreach (var box in c2)
                c1.Add(box);

            if (circuits.Count == 1) return box1.X * box2.X;
        }

        return 0;
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
        _isTest = true;
        var test = doPart(reader);
        Debug.Assert(test == expected, $"test: {test} != expected: {expected}");

        reader = new StreamReader(File.OpenRead(Input));
        _isTest = false;
        return Timer.Time(() => doPart(reader));
    }

    private readonly record struct Vec3(ulong X, ulong Y, ulong Z)
    {
        public static ulong DistanceSquared(Vec3 self, Vec3 other)
        {
            return (self.X - other.X) * (self.X - other.X) + (self.Y - other.Y) * (self.Y - other.Y) +
                   (self.Z - other.Z) * (self.Z - other.Z);
        }
    }
}