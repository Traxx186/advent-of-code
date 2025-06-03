using System.Numerics;
using AdventOfCode.Core;

namespace AdventOfCode.AoC2024.Solution;

public class Day15 : ISolution
{
    private const char EmptyPlot = '.';
    private const char Wall = '#';
    private const char Robot = '@';
    private const char Box = 'O';
    
    public string Name => "Day 15";

    public string Part1(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var (map, moves) = ParseInput(input);

        var robot = map.SearchForValues(Robot).First();

        foreach (var move in moves)
        {
            var direction = GetDirection(move);
            var nextPosition = robot.Coordinates + direction;

            while (map.TryGetTile(nextPosition.Y, nextPosition.X, out var tile) && tile.Value == Box)
                nextPosition += direction;

            if (!map.TryGetTile(nextPosition.Y, nextPosition.X, out var nextTile) || nextTile.Value != EmptyPlot)
                continue;
            
            while (nextPosition != robot.Coordinates)
            {
                var prev = nextPosition - direction;
                map.Tiles[(int)nextPosition.Y][(int)nextPosition.X] = map.Tiles[(int)prev.Y][(int)prev.X];
                nextPosition = prev; 
            }

            map.Tiles[(int)nextPosition.Y][(int)nextPosition.X] = EmptyPlot;
            
            var nextRobotPosition = robot.Coordinates + direction;
            map.TryGetTile(nextRobotPosition.Y, nextRobotPosition.X, out robot);
        }
        
        var total = map.Tiles.SelectMany((rows, y) => rows.Select((_, x) => new Vector2(x, y)))
            .Where(point => map.TryGetTile(point.Y, point.X, out var tile) && tile.Value == Box)
            .Select(point => 100 * point.Y + point.X)
            .Sum();
        
        return total.ToString();
    }

    public string Part2(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var (originalMap, moves) = ParseInput(input);
        var map = DoubleMap(originalMap);
        var scratchMap = DoubleMap(originalMap);

        var robot = map.SearchForValues(Robot).First();

        foreach (var move in moves)
        {
            var direction = GetDirection(move);
            var surroundings = SearchLargeMap(map, robot.Coordinates, direction);
            if (surroundings.Count == 0)
                continue;

            foreach (var surrounding in surroundings)
                scratchMap.Tiles[(int)surrounding.Y][(int)surrounding.X] = map.Tiles[(int)surrounding.Y][(int)surrounding.X];

            foreach (var surrounding in surroundings)
                map.Tiles[(int)surrounding.Y][(int)surrounding.X] = EmptyPlot;

            foreach (var surrounding in surroundings)
            {
                var newPosition = surrounding + direction;
                map.Tiles[(int)newPosition.Y][(int)newPosition.X] = scratchMap.Tiles[(int)surrounding.Y][(int)surrounding.X];
            }

            var nextRobotPosition = robot.Coordinates + direction;
            map.TryGetTile(nextRobotPosition.Y, nextRobotPosition.X, out robot);
        }

        var total = map.Tiles.SelectMany((rows, y) => rows.Select((_, x) => new Vector2(x, y)))
            .Where(point => map.TryGetTile(point.Y, point.X, out var tile) && tile.Value == '[')
            .Select(point => 100 * point.Y + point.X)
            .Sum();

        return total.ToString();
    }

    private static (Grid<char>, char[]) ParseInput(string input)
    {
        var parts = input.Split(Environment.NewLine + Environment.NewLine, StringSplitOptions.RemoveEmptyEntries);
        var map = parts.First()
            .Split(Environment.NewLine)
            .Select(line => line.ToCharArray().ToList())
            .ToList();

        return (new Grid<char>(map), parts.Last().ToCharArray());
    }

    private static Grid<char> DoubleMap(Grid<char> map)
    {
        var newMap = new List<List<char>>(map.Height);

        for (var y = 0; y < map.Height; y++)
        {
            newMap.Add(new List<char>(map.Width * 2));
            for (var x = 0; x < map.Width; x++)
            {
                var current = map.Tiles[y][x];
                char[] newTiles = current switch
                {
                    Wall => ['#', '#'],
                    Box => ['[', ']'],
                    Robot => ['@', '.'],
                    _ => ['.', '.']
                };
                
                newMap[y].Insert(x * 2, newTiles[0]);
                newMap[y].Insert(x * 2 + 1, newTiles[1]);
            }
        }
        
        return new Grid<char>(newMap);
    }

    private static HashSet<Vector2> SearchLargeMap(Grid<char> map, Vector2 robotPosition, Vector2 direction)
    {
        var surrounding = new HashSet<Vector2>([robotPosition]);
        var possiblePositions = new Stack<Vector2>([robotPosition]);
        var left = new Vector2(-1, 0);
        var right = new Vector2(1, 0);

        while (possiblePositions.TryPop(out var current))
        {
            var nextPosition = current + direction;
            if (surrounding.Contains(nextPosition))
                continue;
            
            map.TryGetTile(nextPosition.Y, nextPosition.X, out var nextTile);
            if (nextTile.Value == Wall)
                return [];
            
            if (nextTile.Value != ']' && nextTile.Value != '[')
                continue;
            
            possiblePositions.Push(nextPosition);
            surrounding.Add(nextPosition);

            if (direction.Y == 0)
                continue;
            
            var other = nextTile.Value == ']' ? nextPosition + left : nextPosition + right;
            possiblePositions.Push(other);
            surrounding.Add(other);
        }
        
        return surrounding;
    }
    
    private static Vector2 GetDirection(char direction) => direction switch
    {
        '^' => new Vector2(0, -1),
        '>' => new Vector2(1, 0),
        'v' => new Vector2(0, 1),
        '<' => new Vector2(-1, 0),
        _ => new Vector2(0, 0),
    };
}