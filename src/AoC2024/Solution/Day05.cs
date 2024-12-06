using AdventOfCode.Core;

namespace AdventOfCode.AoC2024.Solution;

public class Day05 : ISolution
{
    public string Name => "Day 5";
    
    public string Part1(string inputFile)
    {
        var data = ParseInput(Calendar.LoadInput(inputFile));
        var validUpdates = new List<List<int>>();

        foreach (var update in data.Item2)
        {
            for (var i = 0; i < update.Length; i++)
            {
                var current = update[i];
                
            }
        }
        
        return string.Empty;
    }

    public string Part2(string inputFile)
    {
        return string.Empty;
    }
    
    private static ((int, int)[], int[][]) ParseInput(string input)
    {
        var parts = input.Split(Environment.NewLine + Environment.NewLine);
        var pageRules = parts.First()
            .Split(Environment.NewLine)
            .Select(line => line.Split("|").Select(int.Parse).ToArray())
            .Select(line => (line.First(), line.Last()))
            .ToArray();
        
        var pageUpdates = parts.Last()
            .Split(Environment.NewLine)
            .Select(line => line.Split(",").Select(int.Parse).ToArray())
            .ToArray();
        
        return (pageRules, pageUpdates);
    }
}