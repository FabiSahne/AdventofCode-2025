using System.Diagnostics;

namespace AdventOfCode;

public static class Day02
{
    private const int Day = 2;

    private const string Test = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224," +
                                "1698522-1698528,446443-446449,38593856-38593862,565653-565659," +
                                "824824821-824824827,2121212118-2121212124";

    private const ulong TestResult1 = 1227775554;
    private const ulong TestResult2 = 4174379265;
    private static readonly string Input = $"{Day:D2}.txt";

    private static ulong Part1(TextReader input)
    {
        var line = input.ReadToEnd();
        line = line.Trim();

        ulong result = 0;

        foreach (var range in line.Split(','))
        {
            var parts = range.Split('-');
            var begin = ulong.Parse(parts[0]);
            var end = ulong.Parse(parts[1]);

            for (var i = begin; i <= end; i += 1)
            {
                var iStr = i.ToString();

                if (iStr.Length % 2 != 0) continue;
                var mid = iStr.Length / 2;
                if (iStr[..mid] == iStr[mid..])
                    result += i;
            }
        }

        return result;
    }

    private static ulong Part2(TextReader input)
    {
        var line = input.ReadToEnd();
        line = line.Trim();

        ulong result = 0;

        foreach (var range in line.Split(','))
        {
            var parts = range.Split('-');
            var begin = ulong.Parse(parts[0]);
            var end = ulong.Parse(parts[1]);

            for (var i = begin; i <= end; i += 1)
            {
                var iStr = i.ToString();
                var invalid = false;

                for (var l = 1; l <= iStr.Length / 2; l += 1)
                {
                    if (iStr.Length % l != 0)
                        continue;

                    var prefix = iStr[..l];
                    var repeat = iStr.Length / l;

                    if (string.Concat(Enumerable.Repeat(prefix, repeat)) != iStr)
                        continue;

                    invalid = true;
                    break;
                }

                if (invalid)
                    result += i;
            }
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
        Debug.Assert(test == expected);

        reader = new StreamReader(File.OpenRead(Input));
        return Timer.Time(() => doPart(reader));
    }
}