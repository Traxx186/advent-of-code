using System.Numerics;
using AdventOfCode.Core;

namespace AdventOfCode.AoC2024.Solution;

public class Day06 : ISolution
{
    private const char Step = '.';
    private static readonly char[] Obstructions = ['#', 'O'];
    
    public string Name => "Day 6";
    
    public string Part1(string inputFile)
    {
        var data = Calendar.LoadInput(inputFile);
        var matrix = ParseInput(data);
        
        var guard = FindGuard(matrix);
        if (!guard.HasValue)
            return string.Empty;
        
        var visitedTiles = new HashSet<Vector2>([guard.Value.Position]);
        while (true)
        {
            var (nextX, nextY) = NextPosition(guard.Value.Position.X, guard.Value.Position.Y, guard.Value.Direction);
            if(!matrix.TryGetTile(nextY, nextX, out var tile))
                break;

            if (!Obstructions.Contains(tile.Value))
            {
                guard = guard.Value with { Position = tile.Coordinates };
                visitedTiles.Add(tile.Coordinates);
                continue;
            }
            
            guard = guard.Value with { Direction = GetNextDirection(guard.Value.Direction) };
        }
       
        return visitedTiles.Count.ToString();
    }

    public string Part2(string inputFile)
    {
        var data = Calendar.LoadInput(inputFile);
        var matrix = ParseInput(data);
        var potentialObstacles = new HashSet<Vector2>();

        for (var row = 0; row < matrix.Height; row++)
        {
            for (var col = 0; col < matrix.Width; col++)
            {
                if (Obstructions.Contains(matrix.Tiles[row][col]))
                    continue;
                
                var newMatrix = new Grid<char>(matrix.Tiles);
                newMatrix.Tiles[row][col] = '#';
                
                var guard = FindGuard(matrix);
                if (!guard.HasValue)
                    continue;
                
                var visitedTiles = new HashSet<Vector2>();
                while (true)
                {
                    if (!visitedTiles.Add(guard.Value.Position))
                    {
                        potentialObstacles.Add(guard.Value.Position);
                        break;
                    }

                    var (nextX, nextY) = NextPosition(guard.Value.Position.X, guard.Value.Position.Y, guard.Value.Direction);
                    if(!matrix.TryGetTile(nextY, nextX, out var tile))
                        break;
                    
                    if (!Obstructions.Contains(tile.Value))
                    {
                        guard = guard.Value with { Position = tile.Coordinates };
                        continue;
                    }
                    
                    guard = guard.Value with { Direction = GetNextDirection(guard.Value.Direction) };
                }
            }
        }
        
        return potentialObstacles.Count.ToString();
    }

    private static Grid<char> ParseInput(string input)
    {
        var data = input.Split(Environment.NewLine)
            .Select(line => line.ToList())
            .ToList();
        
        return new Grid<char>(data);
    }

    private static (float, float) NextPosition(float x, float y, Direction direction)
    {
        return direction switch
        {
            Direction.North => (x, y - 1),
            Direction.East => (x + 1, y),
            Direction.South => (x, y + 1),
            Direction.West => (x - 1, y),
            _ => (x, y)
        };
    }

    private static Guard? FindGuard(Grid<char> grid)
    {
        var guardChar = grid.Tiles.SelectMany(row => row)
            .Where(cell => !Obstructions.Contains(cell) && cell != Step)
            .ToArray();

        if (guardChar.Length == 0)
            return null;
        
        var position = grid.SearchForValues(guardChar[0]).First();
        var direction = position.Value switch
        {
            '^' => Direction.North,
            '>' => Direction.East,
            '<' => Direction.West,
            _ => Direction.South
        };
        
        return new Guard
        {
            Position = position.Coordinates,
            Direction = direction
        };
    }

    private static Direction GetNextDirection(Direction direction)
    {
        return direction switch
        {
            Direction.North => Direction.East,
            Direction.East => Direction.South,
            Direction.South => Direction.West,
            _ => Direction.North
        };
    }
    
    private struct Guard
    {
        public Direction Direction { get; set; }
        public Vector2 Position { get; set; }
    }
}