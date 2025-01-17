using System.Numerics;
using AdventOfCode.Core;

namespace AdventOfCode.AoC2024.Solution;

public class Day10 : ISolution
{
    public string Name => "Day 10";

    public string Part1(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var matrix = ParseInput(input);
        var startPoints = matrix.SearchForValues(0);

        var trails = startPoints.Sum(startPoint => FindTrail(matrix, startPoint));
        return trails.ToString();
    }

    public string Part2(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var matrix = ParseInput(input);
        var startPoints = matrix.SearchForValues(0);

        var trails = startPoints.Sum(startPoint => FindUniqueTrail(matrix, startPoint));
        return trails.ToString();
    }

    private static Grid<int> ParseInput(string input)
    {
        var parsedInput = input.Split(Environment.NewLine)
            .Select(line => line.Select(c => c - '0').ToList())
            .ToList();

        return new Grid<int>(parsedInput);
    }

    private static int FindTrail(Grid<int> grid, Cell<int> point)
    {
        var visited = new HashSet<Cell<int>>();
        var queue = new Queue<Vector2>();

        visited.Add(point);
        queue.Enqueue(point.Coordinates);

        while (queue.TryDequeue(out var current))
        {
            if (!grid.TryGetTile(current.Y, current.X, out var tile))
                continue;

            grid.TryGetTile(current.Y - 1, current.X, out var nextUpTile);
            if (nextUpTile.Coordinates.Y >= 0 && nextUpTile.Value == tile.Value + 1)
                if (visited.Add(nextUpTile))
                    queue.Enqueue(nextUpTile.Coordinates);

            grid.TryGetTile(current.Y + 1, current.X, out var nextDownTile);
            if (nextDownTile.Coordinates.Y < grid.Height && nextDownTile.Value == tile.Value + 1)
                if (visited.Add(nextDownTile))
                    queue.Enqueue(nextDownTile.Coordinates);

            grid.TryGetTile(current.Y, current.X - 1, out var nextLeftTile);
            if (nextLeftTile.Coordinates.X >= 0 && nextLeftTile.Value == tile.Value + 1)
                if (visited.Add(nextLeftTile))
                    queue.Enqueue(nextLeftTile.Coordinates);

            grid.TryGetTile(current.Y, current.X + 1, out var nextRightTile);
            if (nextRightTile.Coordinates.X >= grid.Width || nextRightTile.Value != tile.Value + 1)
                continue;

            if (visited.Add(nextRightTile))
                queue.Enqueue(nextRightTile.Coordinates);
        }

        return visited.Count(cell => cell.Value == 9);
    }

    private static int FindUniqueTrail(Grid<int> grid, Cell<int> point)
    {
        var paths = new List<List<Cell<int>>>();
        var queue = new Queue<List<Cell<int>>>();
        queue.Enqueue([point]);

        while (queue.TryDequeue(out var currentPath))
        {
            var current = currentPath[currentPath.Count - 1];
            if (current.Value == 9)
            {
                paths.Add(currentPath);
                continue;
            }

            grid.TryGetTile(current.Coordinates.Y - 1, current.Coordinates.X, out var nextUpTile);
            if (nextUpTile.Coordinates.Y >= 0 && nextUpTile.Value == current.Value + 1)
            {
                var foundPaths = currentPath
                    .Count(path =>
                        path.Coordinates.Y == nextUpTile.Coordinates.Y &&
                        path.Coordinates.X == nextUpTile.Coordinates.X);

                if (foundPaths == 0)
                {
                    var newPath = currentPath.ToList();
                    newPath.Add(nextUpTile);
                    queue.Enqueue(newPath);
                }
            }

            grid.TryGetTile(current.Coordinates.Y + 1, current.Coordinates.X, out var nextDownTile);
            if (nextDownTile.Coordinates.Y < grid.Height && nextDownTile.Value == current.Value + 1)
            {
                var foundPaths = currentPath
                    .Count(path =>
                        path.Coordinates.Y == nextDownTile.Coordinates.Y &&
                        path.Coordinates.X == nextDownTile.Coordinates.X);

                if (foundPaths == 0)
                {
                    var newPath = currentPath.ToList();
                    newPath.Add(nextDownTile);
                    queue.Enqueue(newPath);   
                }
            }

            grid.TryGetTile(current.Coordinates.Y, current.Coordinates.X - 1, out var nextLeftTile);
            if (nextLeftTile.Coordinates.X >= 0 && nextLeftTile.Value == current.Value + 1)
            {
                var foundPaths = currentPath
                    .Count(path =>
                        path.Coordinates.Y == nextLeftTile.Coordinates.Y &&
                        path.Coordinates.X == nextLeftTile.Coordinates.X);

                if (foundPaths == 0)
                {
                    var newPath = currentPath.ToList();
                    newPath.Add(nextLeftTile);
                    queue.Enqueue(newPath);   
                }
            }

            grid.TryGetTile(current.Coordinates.Y, current.Coordinates.X + 1, out var nextRightTile);
            if (nextRightTile.Coordinates.X >= grid.Width || nextRightTile.Value != current.Value + 1)
                continue;

            var numPaths = currentPath
                .Count(path =>
                    path.Coordinates.Y == nextRightTile.Coordinates.Y &&
                    path.Coordinates.X == nextRightTile.Coordinates.X);

            if (numPaths != 0) 
                continue;
            
            var path = currentPath.ToList();
            path.Add(nextRightTile);
            queue.Enqueue(path);
        }

        return paths.Count;
    }
}