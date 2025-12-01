using AdventOfCode.Core;

namespace AdventOfCode.AoC2025.Solution;

public class Day01 : ISolution
{
    public string Name => "Secret Entrance";

    public string Part1(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var rotations = input.Split(Environment.NewLine, StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries)
            .Select(line => int.Parse(line[1..]) * DetermineRotation(line[0]))
            .ToArray();
        
        return Dial(rotations).Count(r => r == 0).ToString();
    }

    public string Part2(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var rotations = input.Split(Environment.NewLine, StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries)
            .SelectMany(line => Enumerable.Range(0, int.Parse(line[1..])).Select(_ => DetermineRotation(line[0])))
            .ToArray();
        
        return Dial(rotations).Count(r => r == 0).ToString();
    }

    private IEnumerable<int> Dial(IEnumerable<int> rotations)
    {
        var position = 50;
        foreach (var rotation in rotations)
        {
            position = (position + rotation) % 100;
            yield return position;
        }
    }

    private static int DetermineRotation(char direction) => (direction == 'L') ? -1 : 1;
}