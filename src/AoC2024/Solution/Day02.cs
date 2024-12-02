using AdventOfCode.Core;

namespace AdventOfCode.AoC2024.Solution;

public class Day02 : ISolution
{
    public string Name => "Day 2";

    public string Part1(string inputFile)
    {
        var data = Calendar.LoadInput(inputFile);
        var martix = ParseInput(data);

        var safeReports = martix.Tiles.Count(report =>
        {
            // Zip the original report and the report offset by 1 to easily compare
            // the adjacent levels.
            var pair = Enumerable.Zip(report, report.Skip(1));
            return pair.All(p => p.First - p.Second is >= 1 and <= 3) ||
                pair.All(p => p.Second - p.First is >= 1 and <= 3);
        });

        return safeReports.ToString();
    }

    public string Part2(string inputFile)
    {
        var data = Calendar.LoadInput(inputFile);
        var reports = ParseInput(data);
        var safeReports = 0;

        foreach (var report in reports.Tiles)
        {
            // Create a new matrix with the different possibilities in order to check if removing
            // one level can make the report valid.
            var levels = Enumerable.Range(0, report.Count + 1)
                .Select(i => Enumerable.Concat(report.Take(i - 1), report.Skip(i)).ToArray())
                .ToArray();

            var valid = levels.Any(level =>
            {
                // Zip the original report and the report offset by 1 to easily compare
                // the adjacent levels.
                var pair = Enumerable.Zip(level, level.Skip(1));
                return pair.All(p => p.First - p.Second is >= 1 and <= 3) ||
                    pair.All(p => p.Second - p.First is >= 1 and <= 3);
            });

            if (valid)
                safeReports++;
        }

        return safeReports.ToString();
    }

    private static Matrix<int> ParseInput(string input)
    {
        var lines = input.Split(Environment.NewLine);
        var data = lines.Select(l => l.Split(' ').Select(x => int.Parse(x)).ToList()).ToList();

        return new Matrix<int>(data);
    }
}
