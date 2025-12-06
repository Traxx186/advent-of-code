using AdventOfCode.Core;
using AdventOfCode.Core.Numerics;

namespace AdventOfCode.AoC2025.Solution;

public class Day05 : ISolution
{

    public string Name => "Cafeteria";

    public string Part1(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var (idRanges, ids) = ParseInput(input);
        
        var freshCount = ids.Count(id => idRanges.Any(range => range.Contains(id)));
        return freshCount.ToString();
    }

    public string Part2(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var ranges = ParseInput(input).ranges
            .OrderBy(range => range.Start)
            .ToArray();

        for (var i = 0; i < ranges.Length - 1; i++)
        {
            if (ranges[i + 1].Start > ranges[i].End) 
                continue;
            
            var end = Math.Max(ranges[i].End, ranges[i + 1].End);
            ranges[i] = new RangeLong(ranges[i].Start, ranges[i+1].Start - 1);
            ranges[i+1] = new RangeLong(ranges[i+1].Start, end);
        }
        
        return ranges.Sum(range => range.End - range.Start + 1).ToString();
    }

    private static (RangeLong[] ranges, long[] ids) ParseInput(string input)
    {
        var parts = input.Split($"{Environment.NewLine}{Environment.NewLine}", StringSplitOptions.RemoveEmptyEntries);
        
        var idRanges = parts.First()
            .Split(Environment.NewLine, StringSplitOptions.RemoveEmptyEntries)
            .Select(line => line.Split('-', StringSplitOptions.RemoveEmptyEntries).Select(long.Parse).ToArray())
            .Select(range => new RangeLong(range[0], range[1]))
            .ToArray();

        var ids = parts.Last()
            .Split(Environment.NewLine, StringSplitOptions.RemoveEmptyEntries)
            .Select(long.Parse)
            .ToArray();

        return (idRanges, ids);
    }
    
    
}