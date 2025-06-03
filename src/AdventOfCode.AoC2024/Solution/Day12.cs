using System.Numerics;
using AdventOfCode.Core;

namespace AdventOfCode.AoC2024.Solution;

public class Day12 : ISolution
{
    public string Name => "Garden Groups";

    private Grid<char> _grid;
    
    private readonly Dictionary<Direction, (int, int)> _directions = new()
    {
        { Direction.North, (0, -1) },
        { Direction.East, (1, 0) },
        { Direction.South, (0, 1) },
        { Direction.West, (-1, 0) },
    };
    
    public string Part1(string inputFile)
    {
        _grid = ParseInput(Calendar.LoadInput(inputFile));
        var visited = new HashSet<Cell<char>>();
        var cost = 0;

        for (var row = 0; row < _grid.Height; row++)
        {
            for (var col = 0; col < _grid.Width; col++)
            {
                _grid.TryGetTile(row, col, out var plant);
                if (visited.Contains(plant))
                    continue;
                
                var plot = FindPlot(visited, plant);
                cost += plot.edges * plot.cells;
            }
        }
        
        return cost.ToString();
    }

    public string Part2(string inputFile)
    {
        _grid = ParseInput(Calendar.LoadInput(inputFile));
        var visited = new HashSet<Cell<char>>();
        var cost = 0;

        for (var row = 0; row < _grid.Height; row++)
        {
            for (var col = 0; col < _grid.Width; col++)
            {
                _grid.TryGetTile(row, col, out var plant);
                if (visited.Contains(plant))
                    continue;
                
                var plot = FindPlot(visited, plant);
                cost += plot.corners * plot.cells;
            }
        }
        
        return cost.ToString();
    }

    private static Grid<char> ParseInput(string input)
    {
        var lines = input.Split(Environment.NewLine)
            .Select(line => line.ToList())
            .ToList();
        
        return new Grid<char>(lines);
    }

    private (int edges, int cells, int corners) FindPlot(HashSet<Cell<char>> visited, Cell<char> plant)
    {
        if (visited.Contains(plant) || !_grid.TryGetTile(plant.Coordinates.X, plant.Coordinates.Y, out _))
            return (0, 0, 0);
        
        visited.Add(plant);
        var edges = 0;
        var cells = 1;
        var corners = 0;

        for (var i = 0; i < _directions.Count; i++)
        {
            var coordinates = _directions.Values.ElementAt(i);
            var nextCellCoords = new Vector2(plant.Coordinates.X + coordinates.Item1, plant.Coordinates.Y + coordinates.Item2);
            var isCellOut = !_grid.TryGetTile(nextCellCoords.Y, nextCellCoords.X, out var nextCell);

            if (isCellOut || nextCell.Value != plant.Value)
                edges++;

            if (!isCellOut && !visited.Contains(nextCell) && nextCell.Value == plant.Value)
            {
                var nextPlot = FindPlot(visited, nextCell);
                edges += nextPlot.edges;
                cells += nextPlot.cells;
                corners += nextPlot.corners;
            }

            if (!isCellOut && nextCell.Value == plant.Value) 
                continue;
            
            var cornerCoords = _directions.Values.ElementAt((i + 1) % 4);
            var nextCornerCoords = new Vector2(plant.Coordinates.X + cornerCoords.Item1, plant.Coordinates.Y + cornerCoords.Item2);
            var isCornerOut = !_grid.TryGetTile(nextCornerCoords.Y, nextCornerCoords.X, out var nextCorner);

            if (isCornerOut || nextCorner.Value != plant.Value)
            {
                corners++;
                continue;
            }

            var innerCornerCoords = new Vector2(nextCellCoords.X + cornerCoords.Item1, nextCellCoords.Y + cornerCoords.Item2);
            var isInnerCornerOut =
                !_grid.TryGetTile(innerCornerCoords.Y, innerCornerCoords.X, out var nextInnerCorner);

            if (!isInnerCornerOut && nextInnerCorner.Value == plant.Value)
                corners++;
        }
        
        return (edges, cells, corners);
    }
}