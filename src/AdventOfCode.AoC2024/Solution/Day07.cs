using AdventOfCode.Core;

namespace AdventOfCode.AoC2024.Solution;

public class Day07 : ISolution
{
    public string Name => "Day 7";

    public string Part1(string inputFile)
    {
        var data = Calendar.LoadInput(inputFile);
        var input = ParseInput(data);

        return input.Where(target => IsPossible(target.Value, 1, target.Key, target.Value[0], false))
            .Sum(correct => correct.Key)
            .ToString();
    }

    public string Part2(string inputFile)
    {
        var data = Calendar.LoadInput(inputFile);
        var input = ParseInput(data);

        return input.Where(target => IsPossible(target.Value, 1, target.Key, target.Value[0], true))
            .Sum(correct => correct.Key)
            .ToString();
    }

    private static KeyValuePair<long, int[]>[] ParseInput(string input)
    {
        return input.Split(Environment.NewLine, StringSplitOptions.RemoveEmptyEntries)
            .Select(line => line.Split(':'))
            .Select(parts =>
                new KeyValuePair<long, int[]>(long.Parse(parts[0]),
                    parts[1].Trim().Split(' ').Select(int.Parse).ToArray()))
            .ToArray();
    }

    private static bool IsPossible(int[] numbers, int index, long target, long current, bool concat)
    {
        if (index == numbers.Length)
            return current == target;

        var next = numbers[index];
        return IsPossible(numbers, index + 1, target, current + next, concat)
               || IsPossible(numbers, index + 1, target, current * next, concat)
               || (concat && IsPossible(numbers, index + 1, target, long.Parse($"{current}{next}"), concat));
    }
}