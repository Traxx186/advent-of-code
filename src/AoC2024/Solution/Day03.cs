using AdventOfCode.Core;
using System.Text.RegularExpressions;

namespace AdventOfCode.AoC2024.Solution;

public partial class Day03 : ISolution
{
    public string Name => "Day 3";

    private static readonly char[] StartLetters = ['m', 'd'];

    [GeneratedRegex("(?:mul\\()\\d+,\\d+(?:\\))", RegexOptions.IgnoreCase | RegexOptions.Multiline | RegexOptions.Compiled, "en-US")]
    private static partial Regex MultiplyRegex();

    public string Part1(string inputFile)
    {
        var data = Calendar.LoadInput(inputFile);
        var matches = MultiplyRegex().Matches(data);
        var result = matches
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
        var lines = data.Split(Environment.NewLine);
        var enabled = true;
        var multiplications = new List<string>();

        foreach (var line in lines)
        {
            var startIndex = 0;

            for (var i = 0; i < line.Length; i++)
            {
                var ch = line[i];
                if (StartLetters.Contains(ch))
                {
                    startIndex = i;
                    continue;
                }

                if (ch == ')')
                {
                    var sub = line.Substring(startIndex, (i + 1) - startIndex);
                    if (sub.Contains("mul") && enabled)
                    {
                        if (sub.Split('(').First() != "mul")
                            continue;

                        var body = sub[sub.IndexOf('(')..]
                                .Replace(",", string.Empty)
                                .Replace("(", string.Empty)
                                .Replace(")", string.Empty);

                        if (!body.All(char.IsNumber))
                            continue;

                        multiplications.Add(sub);
                        startIndex = i;
                        continue;
                    }

                    if (sub.Contains("don't"))
                        enabled = false;
                    else if (sub.Contains("do"))
                        enabled = true;
                    else
                        continue;
                }
            }
        }

        var result = multiplications.Select(multi => multi.Split(',')
                .Select(s => string.Join(string.Empty, s.Where(c => char.IsNumber(c))))
                .ToArray()
            )
            .Select(nums => int.Parse(nums.First()) * int.Parse(nums.Last()))
            .Sum();

        return result.ToString();
    }
}
