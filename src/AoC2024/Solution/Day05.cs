using AdventOfCode.Core;

namespace AdventOfCode.AoC2024.Solution;

public class Day05 : ISolution
{
    public string Name => "Day 5";
    
    public string Part1(string inputFile)
    {
        var data = ParseInput(Calendar.LoadInput(inputFile));
        var validUpdates = new List<int[]>();

        foreach (var update in data.Item2)
        {
            var rules = data.Item1
                .Where(rule => update.Contains(rule.Item1) && update.Contains(rule.Item2))
                .ToArray();

            var valid = rules.All(rule => Array.IndexOf(update, rule.Item1) <= Array.IndexOf(update, rule.Item2));
            if (valid)
                validUpdates.Add(update);
        }

        var result = validUpdates
            .Select(update => update[(int)Math.Floor((double)update.Length / 2)])
            .Sum();
        
        return result.ToString();
    }

    public string Part2(string inputFile)
    {
        var data = ParseInput(Calendar.LoadInput(inputFile));
        var invalidUpdates = new List<int[]>();

        foreach (var update in data.Item2)
        {
            var rules = data.Item1
                .Where(rule => update.Contains(rule.Item1) && update.Contains(rule.Item2))
                .ToArray();

            var valid = rules.All(rule => Array.IndexOf(update, rule.Item1) <= Array.IndexOf(update, rule.Item2));
            if (valid)
               continue;
            
            invalidUpdates.Add(update);
        }

        foreach (var update in invalidUpdates)
        {
            var rules = data.Item1
                .Where(rule => update.Contains(rule.Item1) && update.Contains(rule.Item2))
                .ToArray();
            
            Array.Sort(update, (item1, item2) =>
            {
                var ruleBefore = rules.FirstOrDefault(rule => rule.Item1 == item1 && rule.Item2 == item2);
                var ruleAfter = rules.FirstOrDefault(rule => rule.Item1 == item2 && rule.Item2 == item1);

                if (ruleBefore is { Item1: 0, Item2: 0 }) return -1;
                
                return ruleAfter is { Item1: 0, Item2: 0 } 
                    ? 1
                    : 0;
            });
        }

        var result = invalidUpdates
            .Select(update => update[(int)Math.Floor((double)update.Length / 2)])
            .Sum();
        
        return result.ToString();
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
            .Where(line => !string.IsNullOrWhiteSpace(line))
            .Select(line => line.Split(",").Select(int.Parse).ToArray())
            .ToArray();
        
        return (pageRules, pageUpdates);
    }
}