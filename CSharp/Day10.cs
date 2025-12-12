using System.Collections;
using System.Diagnostics;
using Microsoft.Z3;

namespace AdventOfCode;

public static class Day10
{
    private const int Day = 10;

    private const string Test = """
                                [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
                                [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
                                [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
                                """;

    private const ulong TestResult1 = 7;
    private const ulong TestResult2 = 33;
    private static readonly string Input = $"{Day:D2}.txt";

    private static ulong Part1(TextReader input)
    {
        ulong sum = 0;
        while (input.ReadLine() is string line)
        {
            var parts = line.Split(' ');
            var lights = parts[0][1..^1].Select((c, i) => (i, c)).Aggregate(0,
                (lights, light) => lights ^ (Convert.ToInt32(light.c == '#') << light.i));
            var buttons = parts[1..^1]
                .Select(b => b[1..^1].Split(',').Select(int.Parse).ToList()).ToList();
            Debug.WriteLine($"Processing Line with goal: {lights:B}");
            sum += SolveViaBfs(lights, buttons);
        }

        return sum;
    }

    private static ulong Part2(TextReader input)
    {
        var readerIter = new Lines(input);
        ulong pressed = 0;

        Parallel.ForEach(readerIter, line =>
        {
            var parts = line.Split(' ');
            var joltage = parts[^1][1..^1].Split(',').Select(int.Parse).ToArray();
            var buttons = parts[1..^1]
                .Select(b => b[1..^1].Split(',').Select(int.Parse).ToList()).ToList();
            var p = SolveViaZ3(joltage, buttons);
            Interlocked.Add(ref pressed, p);
        });

        return pressed;
    }

    private static ulong SolveViaBfs(int goal, List<List<int>> buttons)
    {
        Queue<(int, ulong)> queue = [];
        queue.Enqueue((0, 0));
        HashSet<int> seen = [0];

        while (queue.Count > 0)
        {
            var (state, pressed) = queue.Dequeue();
            if (state == goal)
                return pressed;

            //Debug.WriteLine($"State: {state:B}");

            seen.Add(state);

            var neighbors = buttons
                .Select(button => button.Aggregate(state, (current, wire) => current ^ (1 << wire)))
                .Where(neighbor => !seen.Contains(neighbor));

            foreach (var neighbor in neighbors)
                queue.Enqueue((neighbor, pressed + 1));
        }

        return 0;
    }

    private static ulong SolveViaZ3(int[] goal, List<List<int>> buttons)
    {
        using var ctx = new Context();
        var opt = ctx.MkOptimize();

        var buttonVars = new ArithExpr[buttons.Count];
        for (var i = 0; i < buttons.Count; i += 1)
            buttonVars[i] = ctx.MkIntConst($"button_{i}");

        var zero = ctx.MkInt(0);
        foreach (var buttonVar in buttonVars)
            opt.Assert(ctx.MkGe(buttonVar, zero));

        for (var counterIdx = 0; counterIdx < goal.Length; counterIdx += 1)
        {
            long target = goal[counterIdx];

            var sumTerms = new List<ArithExpr>();
            for (var buttonIdx = 0; buttonIdx < buttons.Count; buttonIdx += 1)
                if (buttons[buttonIdx].Contains(counterIdx))
                    sumTerms.Add(buttonVars[buttonIdx]);

            if (sumTerms.Count == 0)
            {
                if (target != 0)
                    throw new UnreachableException();
            }
            else
            {
                var sum = sumTerms.Count == 1
                    ? sumTerms[0]
                    : ctx.MkAdd(sumTerms);
                var targetVal = ctx.MkInt(target);
                opt.Assert(ctx.MkEq(sum, targetVal));
            }
        }

        var total = buttonVars.Length == 1
            ? buttonVars[0]
            : ctx.MkAdd(buttonVars);
        opt.MkMinimize(total);

        // solve

        if (opt.Check() != Status.SATISFIABLE)
            throw new UnreachableException();

        var model = opt.Model;
        var result = model.Evaluate(total, true);
        if (result is IntNum intNum)
            return intNum.UInt64;

        throw new UnreachableException();
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
        Debug.WriteLine("Test success!");

        reader = new StreamReader(File.OpenRead(Input));
        return Timer.Time(() => doPart(reader));
    }

    private class Lines(TextReader reader) : IEnumerable<string>
    {
        public IEnumerator<string> GetEnumerator()
        {
            return new ReaderIter(reader);
        }

        IEnumerator IEnumerable.GetEnumerator()
        {
            return GetEnumerator();
        }

        private class ReaderIter(TextReader reader) : IEnumerator<string>
        {
            private string _current = string.Empty;

            public bool MoveNext()
            {
                if (reader.ReadLine() is not string line) return false;
                _current = line;
                return true;
            }

            public void Reset()
            {
                throw new NotImplementedException();
            }

            string IEnumerator<string>.Current => _current;

            object IEnumerator.Current => _current;

            public void Dispose()
            {
                GC.SuppressFinalize(this);
            }
        }
    }
}