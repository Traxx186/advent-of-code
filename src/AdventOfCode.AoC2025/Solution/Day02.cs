using AdventOfCode.Core;

namespace AdventOfCode.AoC2025.Solution;

public class Day02 : ISolution
{
    public string Name => "Secret Entrance";

    public string Part1(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var ranges = input.Split(',', StringSplitOptions.RemoveEmptyEntries)
            .Select(part => part.Split('-'))
            .Select(ids => new Range(ids[0], ids[1]))
            .ToArray();

        var sum = 0L;
        foreach (var range in ranges)
        {
            for (var i = range.Start; i <= range.End; i++)
            {
                if (Periodic(i.ToString(), 2))
                    sum += i;
            }
        }
        
        return sum.ToString();
    }

    public string Part2(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var ranges = input.Split(',', StringSplitOptions.RemoveEmptyEntries)
            .Select(part => part.Split('-'))
            .Select(ids => new Range(ids[0], ids[1]))
            .ToArray();

        var sum = 0L;
        foreach (var range in ranges)
        {
            for (var i = range.Start; i <= range.End; i++)
            {
                var id = i.ToString();
                if (Enumerable.Range(2, id.Length - 1).Any(c => Periodic(id, c)))
                    sum += i;
            }
        }
        
        return sum.ToString();
    }

    private static bool Periodic(string id, int repetitionCount)
    {
        if (id.Length % repetitionCount != 0) 
            return false;

        var period = id.Length / repetitionCount;
        for (var i = period; i < id.Length; i += period) 
        {
            if (id[..period] != id[i..(i + period)]) 
                return false;
        }
        
        return true;
    }

    private readonly struct Range(string start, string end)
    {
        public long Start { get; } = long.Parse(start);
        public long End { get; } = long.Parse(end);
    }
}