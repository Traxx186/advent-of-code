using AdventOfCode.Core;

namespace AdventOfCode.AoC2025.Solution;

public class Day06 : ISolution
{
    public string Name => "Trash Compactor";
    
    public string Part1(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var problems = ParseBlocks(input)
            .Select(blocks => new Problem(blocks.Last()[0], blocks[..^1].Select(long.Parse).ToArray()))
            .ToArray();
        
        var result = 0L;
        foreach (var problem in problems)
        {
            if (problem.Operator == '+')
                result += problem.Numbers.Sum();
            else
                result += problem.Numbers.Aggregate(1L, (a, b) => a * b);
        }
        
        return result.ToString();
    }

    public string Part2(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var problems = ParseBlocks(input)
            .Select(blocks => Enumerable.Range(0, blocks[0].Length).Select(r => GetColumn(blocks, r)).ToArray())
            .Select(blocks => new Problem(blocks.First()[^1], blocks.Select(b => long.Parse(b[..^1])).ToArray()))
            .ToArray();
        
        var result = 0L;
        foreach (var problem in problems)
        {
            if (problem.Operator == '+')
                result += problem.Numbers.Sum();
            else
                result += problem.Numbers.Aggregate(1L, (a, b) => a * b);
        }
        
        return result.ToString();
    }

    private static IEnumerable<string[]> ParseBlocks(string input)
    {
        var lines = input.Split(Environment.NewLine, StringSplitOptions.RemoveEmptyEntries);
        var columnCount = lines[0].Length;
        var blockStart = 0;

        for (var i = 0; i < columnCount; i++)
        {
            if (!string.IsNullOrWhiteSpace(GetColumn(lines, i).Trim()))
                continue;
            
            yield return GetBlock(lines, blockStart, i);
            blockStart = i + 1;
        }
        
        yield return GetBlock(lines, blockStart, columnCount);
    }

    private static string[] GetBlock(string[] lines, int start, int end) =>
        lines.Select(line => line[start..end]).ToArray();
    
    private static string GetColumn(string[] lines, int index) => 
        string.Join(string.Empty, lines.Select(line => line[index]));
    
    private record Problem(char Operator, long[] Numbers);
}