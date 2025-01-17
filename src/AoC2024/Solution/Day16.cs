using System.Numerics;
using AdventOfCode.Core;
using AdventOfCode.Core.Point;

namespace AdventOfCode.AoC2024.Solution;

public class Day16 : ISolution
{
    private const char EmptyPlot = '.';
    private const char Wall = '#';
    
    public string Name => "Day 16";
    
    public string Part1(string inputFile)
    {
        var grid = ParseInput(Calendar.LoadInput(inputFile));
        var start = grid.SearchForValues('S').First();
        var goal = grid.SearchForValues('E').First();
        var startState = new State(start.Coordinates, Directions[Direction.East]);
        var path = FindPathInMaze(grid, goal.Coordinates);
        
        return path[startState].ToString();
    }

    public string Part2(string inputFile)
    {
        var grid = ParseInput(Calendar.LoadInput(inputFile));
        var start = grid.SearchForValues('S').First();
        var goal = grid.SearchForValues('E').First();
        var path = FindPathInMaze(grid, goal.Coordinates);
        
        var startState = new State(start.Coordinates, Directions[Direction.East]);
        var queue = new PriorityQueue<State, int>();
        queue.Enqueue(startState, path[startState]);

        var bestSpots = new HashSet<State> { startState };
        while (queue.TryDequeue(out var state, out var remainingScore))
        {
            foreach (var (next, score) in Steps(grid, state, true))
            {
                var nextRemainingScore = remainingScore - score;
                if (bestSpots.Contains(next) || path[next] != nextRemainingScore) 
                    continue;
                
                bestSpots.Add(next);
                queue.Enqueue(next, nextRemainingScore);
            }
        }
        
        return bestSpots.DistinctBy(state => state.Current)
            .Count()
            .ToString();
    }

    private static Dictionary<State, int> FindPathInMaze(Matrix<char> grid, Vector2 goal)
    {
        var queue = new PriorityQueue<State, int>();
        var visited = new Dictionary<State, int>();

        foreach (var direction in Directions)
        {
            queue.Enqueue(new State(goal, direction.Value), 0);
            visited[new State(goal, direction.Value)] = 0;
        }
        
        while (queue.TryDequeue(out var state, out var totalDistance))
        {
            foreach (var (next, score) in Steps(grid, state, false))
            {
                var nextCost = totalDistance + score;
                if (nextCost >= visited.GetValueOrDefault(next, int.MaxValue)) 
                    continue;
                
                var newQueue = new PriorityQueue<State, int>();
                while (queue.TryDequeue(out var val, out var priority))
                    if (val != next)
                        newQueue.Enqueue(val, priority);
                
                queue = newQueue;
                visited[next] = nextCost;
                queue.Enqueue(next, nextCost);
            }
        }
        
        return visited;
    }
    
    private static Matrix<char> ParseInput(string input)
    {
        var lines = input.Split(Environment.NewLine, StringSplitOptions.RemoveEmptyEntries);
        var map = lines.Select(line => line.ToCharArray().ToList())
            .ToList();

        return new Matrix<char>(map);
    }

    private static Dictionary<Direction, Vector2> Directions => new()
    {
        { Direction.North, new Vector2(0, -1) },
        { Direction.East, new Vector2(1, 0) },
        { Direction.South, new Vector2(0, 1) },
        { Direction.West, new Vector2(-1, 0) }
    };

    private static IEnumerable<(State, int cost)> Steps(Matrix<char> map, State state, bool forward)
    {
        foreach (var direction in Directions)
        {
            if (direction.Value == state.Direction)
            {
                var position = forward ? state.Current + direction.Value : state.Current - direction.Value;
                if (map.TryGetTile(position.Y, position.X, out var tile) && tile.Value != Wall)
                    yield return (new State(position, direction.Value), 1);
            }
            else if (direction.Value != -state.Direction)
                yield return (state with { Direction = direction.Value }, 1000);
        }
    }
    
    private readonly record struct State(Vector2 Current, Vector2 Direction);
}