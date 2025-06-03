using System.Numerics;
using AdventOfCode.Core;

namespace AdventOfCode.AoC2024.Solution;

public class Day08 : ISolution
{
    public string Name => "Resonant Collinearity";

    public string Part1(string inputFile)
    {
        var data = Calendar.LoadInput(inputFile);
        var matrix = ParseInput(data);
        var antinodes = new HashSet<Vector2>();

        for (var y = 0; y < matrix.Height; y++)
        {
            for (var x = 0; x < matrix.Width; x++)
            {
                var col = matrix.Tiles[y][x];
                var frequencyGrid = matrix.Tiles
                    .Select(r =>
                        r.Select(c => c == col ? col : '.')
                            .ToArray()
                    )
                    .ToArray();

                for (var frequencyY = 0; frequencyY < frequencyGrid.Length; frequencyY++)
                {
                    for (var frequencyX = 0; frequencyX < frequencyGrid[0].Length; frequencyX++)
                    {
                        var frequencyCol = frequencyGrid[frequencyY][frequencyX];
                        if (frequencyY == y && frequencyX == x || frequencyCol == '.')
                            continue;

                        var rowDistance = frequencyY - y;
                        var colDistance = frequencyX - x;

                        var antinodes1 = new Vector2(x - colDistance, y - rowDistance);
                        var antinodes2 = new Vector2(frequencyX + colDistance, frequencyY + rowDistance);

                        if (antinodes1.Y < matrix.Height && antinodes1.Y >= 0 && antinodes1.X < matrix.Width && antinodes1.X >= 0)
                            antinodes.Add(antinodes1);

                        if (antinodes2.Y < matrix.Height && antinodes2.Y >= 0 && antinodes2.X < matrix.Width && antinodes2.X >= 0)
                            antinodes.Add(antinodes2);
                    }
                }
            }
        }

        return antinodes.Count.ToString();
    }

    public string Part2(string inputFile)
    {
        var data = Calendar.LoadInput(inputFile);
        var matrix = ParseInput(data);
        var antinodes = new HashSet<Vector2>();

        for (var y = 0; y < matrix.Height; y++)
        {
            for (var x = 0; x < matrix.Width; x++)
            {
                var col = matrix.Tiles[y][x];
                var frequencyGrid = matrix.Tiles
                    .Select(r =>
                        r.Select(c => c == col ? col : '.')
                            .ToArray()
                    )
                    .ToArray();
                
                var visitedTiles = new HashSet<Vector2>();
                for (var frequencyY = 0; frequencyY < frequencyGrid.Length; frequencyY++)
                {
                    for (var frequencyX = 0; frequencyX < frequencyGrid[0].Length; frequencyX++)
                    {
                        var frequencyCol = frequencyGrid[frequencyY][frequencyX];
                        if (frequencyY == y && frequencyX == x || frequencyCol == '.')
                            continue;

                        var rowDistance = frequencyY - y;
                        var colDistance = frequencyX - x;

                        var step1 = new Vector2(x, y);
                        var step2 = new Vector2(frequencyX, frequencyY);
                        var step1Out = false;
                        var step2Out = false;

                        if (frequencyGrid[(int)step1.Y][(int)step1.X] == col)
                        {
                            if (!visitedTiles.Add(step1))
                                antinodes.Add(step1);
                        }

                        if (frequencyGrid[(int)step2.Y][(int)step2.X] == col)
                        {
                            if (!visitedTiles.Add(step2))
                                antinodes.Add(step2);
                        }

                        while (true)
                        {
                            var antinodes1 = new Vector2(step1.X - colDistance, step1.Y - rowDistance);
                            var antinodes2 = new Vector2(step2.X + colDistance, step2.Y + rowDistance);

                            if (!step1Out && antinodes1.Y < matrix.Height && antinodes1.Y >= 0 && antinodes1.X < matrix.Width && antinodes1.X >= 0)
                                antinodes.Add(antinodes1);
                            else
                                step1Out = true;

                            if (!step2Out && antinodes2.Y < matrix.Height && antinodes2.Y >= 0 && antinodes2.X < matrix.Width && antinodes2.X >= 0)
                                antinodes.Add(antinodes2);
                            else
                                step2Out = true;

                            step1 = antinodes1;
                            step2 = antinodes2;

                            if (step1Out && step2Out)
                                break;
                        }
                    }
                }
            }
        }

        return antinodes.Count.ToString();
    }

    private static Grid<char> ParseInput(string input)
    {
        var data = input.Split(Environment.NewLine)
            .Select(line => line.ToList())
            .ToList();

        return new Grid<char>(data);
    }
}