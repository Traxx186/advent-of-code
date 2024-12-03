using AdventOfCode.Core;
using System.Text.RegularExpressions;

namespace AdventOfCode.AoC2024.Solution;

public partial class Day03 : ISolution
{
    public string Name => "Day 3";

    [GeneratedRegex("mul\\(\\d+,\\d+\\)|do\\(\\)|don't\\(\\)", RegexOptions.IgnoreCase | RegexOptions.Multiline | RegexOptions.Compiled, "en-US")]
    private static partial Regex ValidOperatorsRegex();

    public string Part1(string inputFile)
    {
        var data = Calendar.LoadInput(inputFile);
        var matches = ValidOperatorsRegex().Matches(data);
        var result = matches
            .Where(match => match.Value.Contains("mul"))
            .Select(match => match.Value.Split(',')
                .Select(s => string.Join(string.Empty, s.Where(c => char.IsNumber(c))))
                .ToArray()
            )
            .Select(nums => int.Parse(nums.First()) * int.Parse(nums.Last()))
            .Sum();

        return result.ToString();
    }

    public string Part2(string inputFile)
    {
        var data = Calendar.LoadInput(inputFile);
        var matches = ValidOperatorsRegex()
            .Matches(data)
            .Select(match => match.Value.ToLower());

        var enabled = true;
        var sum = 0;

        foreach (var match in matches)
        {
            if (match == "do()")
            {
                enabled = true;
                continue;
            }

            if (match == "don't()")
            {
                enabled = false;
                continue;
            }

            if (!enabled)
                continue;

            var numbers = match.Split(',')
                .Select(s => string.Join(string.Empty, s.Where(c => char.IsNumber(c))))
                .Select(int.Parse)
                .ToArray();

            sum += numbers.First() * numbers.Last();
        }

        return sum.ToString();
    }
}
