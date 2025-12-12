using System.Diagnostics;

namespace AdventOfCode;

public static class Day06
{
    private const int Day = 6;

    private const string Test = """
                                123 328  51 64 
                                 45 64  387 23 
                                  6 98  215 314
                                *   +   *   +  
                                """;

    private const ulong TestResult1 = 4277556;
    private const ulong TestResult2 = 3263827;
    private static readonly string Input = $"{Day:D2}.txt";

    private static long Calc(this Op op, List<long> numbers)
    {
        switch (op)
        {
        case Op.Mul:
            return numbers.Aggregate((prod, num) => prod * num);
        default:
        case Op.Add:
            return numbers.Sum();
        }
    }

    private static bool OpFromChar(char c, out Op op)
    {
        switch (c)
        {
        case '*':
            op = Op.Mul;
            return true;
        case '+':
            op = Op.Add;
            return true;
        default:
            op = default;
            return false;
        }
    }

    private static ulong Part1(TextReader input)
    {
        List<List<long>> problems = [];
        List<Op> ops = [];

        while (input.ReadLine() is string line)
            foreach (var (part, i) in line.Split(' ').Where(l => !string.IsNullOrWhiteSpace(l))
                         .Select((part, i) => (part, i)))
                if (OpFromChar(part[0], out var op))
                {
                    ops.Add(op);
                }
                else
                {
                    var num = long.Parse(part);
                    if (problems.Count > i)
                        problems[i].Add(num);
                    else
                        problems.Add([num]);
                }

        return (ulong)ops.Zip(problems).Select(z => z.First.Calc(z.Second)).Sum();
    }

    private static ulong Part2(TextReader input)
    {
        List<List<char>> lines = [];
        while (input.ReadLine() is string line) lines.Add(line.ToList());

        var width = lines[0].Count;
        var height = lines.Count;

        List<List<long>> numbers = [];
        List<Op> ops = [];

        var i = width;
        List<long> curNumbers = [];

        while (i > 0)
        {
            i -= 1;

            long num = 0;

            for (var h = 0; h < height; h += 1)
            {
                if (!char.IsAsciiDigit(lines[h][i])) continue;

                num *= 10;
                num += lines[h][i] - '0';
            }

            curNumbers.Add(num);

            if (!OpFromChar(lines[height - 1][i], out var op)) continue;

            ops.Add(op);
            numbers.Add([..curNumbers]);
            curNumbers.Clear();
            i -= 1;
        }

        return (ulong)ops.Zip(numbers).Select(z => z.First.Calc(z.Second)).Sum();
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

    private enum Op
    {
        Add,
        Mul
    }
}