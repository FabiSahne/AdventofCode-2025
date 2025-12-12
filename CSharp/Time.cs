namespace AdventOfCode;

public static class Timer
{
    public static T Time<T>(Func<T> task)
    {
        var start = DateTime.UtcNow;
        var result = task();
        var elapsed = DateTime.UtcNow - start;

        Console.WriteLine($"Took {elapsed:c}");

        return result;
    }
}