using System.Diagnostics;
using System.Text;

namespace AdventOfCode;

public static class Day03
{
    private const int Day = 3;

    private const string Test = """
                                987654321111111
                                811111111111119
                                234234234234278
                                818181911112111
                                """;

    private const ulong TestResult1 = 357;
    private const ulong TestResult2 = 3121910778619;
    private static readonly string Input = $"{Day:D2}.txt";

    private static ulong DoPart1(TextReader input)
    {
        ulong sum = 0;
        while (input.ReadLine() is string line)
        {
            var hasDigitOne = line[..^1];
            var (idx, max1) = hasDigitOne.Index().MaxBy(c => c.Item);
            var hasDigitTwo = line[(idx + 1)..];
            var max2 = hasDigitTwo.Max();
            sum += ulong.Parse($"{max1}{max2}");
        }

        return sum;
    }

    private static ulong DoPart2(TextReader reader)
    {
        ulong sum = 0;
        while (reader.ReadLine() is string line)
        {
            var batteries = new StringBuilder();
            var left = 0;
            for (var right = 11; right >= 0; right -= 1)
            {
                var hasDigit = line[left..^right];
                var (offset, max) = hasDigit.Index().MaxBy(c => c.Item);
                left += offset + 1;
                batteries.Append(max);
            }

            sum += ulong.Parse(batteries.ToString());
        }

        return sum;
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

    public static void Run()
    {
        Console.WriteLine($"\n\n=== Day {Day:D2} ===");

        Console.WriteLine(Part(1, TestResult1, DoPart1));
        Console.WriteLine(Part(2, TestResult2, DoPart2));
    }
}