using System.Diagnostics;

namespace AdventOfCode;

public static class Day01
{
    private const int Day = 1;

    private const string Test = """
                                L68
                                L30
                                R48
                                L5
                                R60
                                L55
                                L1
                                L99
                                R14
                                L82
                                """;

    private const ulong TestResult1 = 3;
    private const ulong TestResult2 = 6;
    private static readonly string Input = $"{Day:D2}.txt";

    private static ulong Part1(TextReader input)
    {
        var position = 50;
        ulong zeros = 0;

        while (input.ReadLine() is string line)
        {
            var (dir, amountStr) = (line[0], line[1..]);
            var amount = int.Parse(amountStr);

            position += dir switch
            {
                'L' => -amount,
                'R' => amount,

                _ => throw new InvalidDataException()
            };

            while (position < 0) position += 100;

            position %= 100;

            if (position is 0 or 100) zeros += 1;
        }

        return zeros;
    }

    private static ulong Part2(TextReader input)
    {
        var position = 50;
        ulong zeros = 0;

        while (input.ReadLine() is string line)
        {
            var (dir, amountStr) = (line[0], line[1..]);
            var amount = int.Parse(amountStr);

            switch (dir)
            {
                case 'L':
                    for (var i = 0; i < amount; i += 1)
                    {
                        position = position switch
                        {
                            0 => 99,
                            > 0 => position - 1,
                            _ => position
                        };
                        if (position == 0)
                            zeros += 1;
                    }

                    break;
                case 'R':
                    for (var i = 0; i < amount; i++)
                    {
                        position = position switch
                        {
                            99 => 0,
                            < 99 => position + 1,
                            _ => position
                        };
                        if (position == 0)
                            zeros += 1;
                    }

                    break;
            }
        }

        return zeros;
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
}