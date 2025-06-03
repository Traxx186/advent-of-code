using AdventOfCode.Core;

namespace AdventOfCode.AoC2024.Solution;

public class Day01 : ISolution
{
    public string Name => "Day 1";

    public string Part1(string inputFile)
    {
        var data = Calendar.LoadInput(inputFile);
        var (left, right) = ParseInput(data);
        
        left.Sort();
        right.Sort();
        
        var output = left.Zip(right, (l, r) => Math.Abs(l - r));
        return output.Sum().ToString();
    }

    public string Part2(string inputFile)
    {
        var data = Calendar.LoadInput(inputFile);
        var (left, right) = ParseInput(data);
        
        var output = left.Select(l => l * right.Count(r => l == r))
            .ToArray();
        
        return output.Sum().ToString();
    }

    private static (List<int> left, List<int> right) ParseInput(string input)
    {
        var lines = input.Split(Environment.NewLine);
        var left = new List<int>(lines.Length);
        var right = new List<int>(lines.Length);

        foreach (var line in lines)
        {
            var split = line.Split("   ");
            left.Add(int.Parse(split[0].Trim()));
            right.Add(int.Parse(split[1].Trim()));
        }
        
        return (left, right);
    }
}