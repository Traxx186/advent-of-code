using AdventOfCode.Core;

namespace AdventOfCode.AoC2024.Solution;

public class Day02 : ISolution
{
    public string Name => "Day 2";

    public string Part1(string inputFile)
    {
        var data = Calendar.LoadInput(inputFile);
        var matrix = ParseInput(data);

        var safeReports = matrix.Tiles.Count(report =>
        {
            // Zip the original report and the report offset by 1 to easily compare
            // the adjacent levels.
            var pair = report.Zip(report.Skip(1));
            return pair.All(p => p.First - p.Second is >= 1 and <= 3) ||
                pair.All(p => p.Second - p.First is >= 1 and <= 3);
        });

        return safeReports.ToString();
    }

    public string Part2(string inputFile)
    {
        var data = Calendar.LoadInput(inputFile);
        var reports = ParseInput(data);
        var safeReports = reports.Tiles
            .Select(report => Enumerable.Range(0, report.Count + 1)
                .Select(i => report.Take(i - 1).Concat(report.Skip(i)).ToArray())
                .ToArray())
            .Select(levels => levels.Any(level =>
            {
                // Zip the original report and the report offset by 1 to easily compare
                // the adjacent levels.
                var pair = level.Zip(level.Skip(1));
                return pair.All(p => p.First - p.Second is >= 1 and <= 3) ||
                       pair.All(p => p.Second - p.First is >= 1 and <= 3);
            }))
            .Count(valid => valid);

        return safeReports.ToString();
    }

    private static Matrix<int> ParseInput(string input)
    {
        var lines = input.Split(Environment.NewLine);
        var data = lines.Select(l => l.Split(' ').Select(int.Parse).ToList()).ToList();

        return new Matrix<int>(data);
    }
}
