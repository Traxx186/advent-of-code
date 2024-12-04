using AdventOfCode.Core;
using System.Text.RegularExpressions;

namespace AdventOfCode.AoC2024.Solution;

public partial class Day04 : ISolution
{
    public string Name => "Day 4";

    [GeneratedRegex("XMAS", RegexOptions.IgnoreCase | RegexOptions.Compiled, "en-US")]
    private static partial Regex XmasRegex();

    public string Part1(string inputFile)
    {
        var data = Calendar.LoadInput(inputFile);
        var grid = data.Split(Environment.NewLine)
            .Select(line => line.Select(ch => ch).ToArray())
            .ToArray();

        var total = 0;
        foreach (var row in grid)
        {
            total += XmasRegex().Matches(string.Join(string.Empty, row)).Count;
            total += XmasRegex().Matches(string.Join(string.Empty, row.Reverse())).Count;
        }

        for (var i = 0; i < grid[0].Length; i++)
        {
            var col = grid.Select(row => row[i]).ToArray();
            var colText = string.Join(string.Empty, col);
            var revColText = string.Join(string.Empty, col.Reverse());

            total += XmasRegex().Matches(colText).Count;
            total += XmasRegex().Matches(revColText).Count;
        }

        for (var i = 0; i < grid[0].Length - 3; i++)
        {
            for (var j = 0; j < grid[i].Length - 3; j++)
            {
                var miniGrid = new char[4, 4];
                var diagonalList = new char[miniGrid.GetLength(0)];
                var antiDiagonalList = new char[miniGrid.GetLength(0)];

                for (var x = 0; x < miniGrid.GetLength(0); x++)
                {
                    for (var y = 0; y < miniGrid.GetLength(1); y++)
                        miniGrid[x, y] = grid[i + x][j + y];
                }

                for (var y = 0; y < miniGrid.GetLength(0); y++)
                {
                    diagonalList[y] = miniGrid[y, y];
                    antiDiagonalList[y] = miniGrid[y, 3 - y];
                }

                total += XmasRegex().Matches(string.Join(string.Empty, diagonalList)).Count;
                total += XmasRegex().Matches(string.Join(string.Empty, diagonalList.Reverse())).Count;

                total += XmasRegex().Matches(string.Join(string.Empty, antiDiagonalList)).Count;
                total += XmasRegex().Matches(string.Join(string.Empty, antiDiagonalList.Reverse())).Count;
            }
        }

        return total.ToString();
    }

    public string Part2(string inputFile)
    {
        var data = Calendar.LoadInput(inputFile);
        string[] validText = ["MAS", "SAM"];
        var grid = data.Split(Environment.NewLine)
            .Select(line => line.Select(ch => ch).ToArray())
            .ToArray();

        var total = 0;

        for (var i = 0; i < grid[0].Length - 2; i++)
        {
            for (var j = 0; j < grid[i].Length - 2; j++)
            {
                var miniGrid = new char[3, 3];
                var diagonalList = new List<char>();
                var antiDiagonalList = new List<char>();

                for (var x = 0; x < miniGrid.GetLength(0); x++)
                {
                    for (var y = 0; y < miniGrid.GetLength(1); y++)
                        miniGrid[x, y] = grid[i + x][j + y];
                }

                for (var y = 0; y < miniGrid.GetLength(0); y++)
                {
                    diagonalList.Add(miniGrid[y, y]);
                    antiDiagonalList.Add(miniGrid[y, 2 - y]);
                }

                var diagonal = string.Join(string.Empty, diagonalList);
                var antiDiagonal = string.Join(string.Empty, antiDiagonalList);

                if (validText.Contains(diagonal) && validText.Reverse().Contains(antiDiagonal))
                    total++;
            }
        }

        return total.ToString();
    }
}
