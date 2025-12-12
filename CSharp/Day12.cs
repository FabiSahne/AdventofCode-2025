namespace AdventOfCode;

public static class Day12
{
    private const int Day = 12;
    private static readonly string Input = $"{Day:D2}.txt";

    public static void Run()
    {
        Console.WriteLine($"\n\n=== Day {Day:D2} ===");

        var reader = new StreamReader(File.OpenRead(Input));
        Console.WriteLine(Timer.Time(() => RunPart(reader)));
    }

    private static int RunPart(StreamReader reader)
    {
        var lines = new List<string>();
        while (reader.ReadLine() is string line)
            lines.Add(line);

        var result = lines[30..].Where(tree =>
        {
            var numbers = tree.Split(':', ' ', 'x').Where(s => !string.IsNullOrWhiteSpace(s)).Select(int.Parse)
                .ToArray();
            var width = numbers[0];
            var height = numbers[1];
            var required = numbers[2..].Sum();
            return width / 3 * height / 3 >= required;
        }).Count();

        return result;
    }
}