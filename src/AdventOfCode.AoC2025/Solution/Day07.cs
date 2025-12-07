using AdventOfCode.Core;

namespace AdventOfCode.AoC2025.Solution;

public class Day07 : ISolution
{
    public string Name => "Laboratories";
    
    public string Part1(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var tachyonManifold = ParseTachyonManifold(input);
        var splits = RunManifold(tachyonManifold).splits;
        
        return splits.ToString();
    }

    public string Part2(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var tachyonManifold = ParseTachyonManifold(input);
        var timelines = RunManifold(tachyonManifold).timeLines;

        return timelines.ToString();
    }

    private static Grid<char> ParseTachyonManifold(string input)
    {
        var manifold = input.Split(Environment.NewLine, StringSplitOptions.RemoveEmptyEntries)
            .Select(line => line.ToCharArray().ToList())
            .ToList();
        
        return new Grid<char>(manifold);
    }

    private static (int splits, long timeLines) RunManifold(Grid<char> manifold)
    {
        var splits = 0;
        var timelines = new long[manifold.Width];

        for (var x = 0; x < manifold.Height; x++)
        {
            var newTimelines = new long[manifold.Width];
            for (var y = 0; y < manifold.Width; y++)
            {
                if (!manifold.TryGetTile(x, y, out var tile))
                    continue;

                switch (tile.Value)
                {
                    case 'S':
                        newTimelines[y] = 1;
                        break;
                    case '^':
                        splits += timelines[y] > 0 ? 1 : 0;
                        newTimelines[y - 1] += timelines[y];
                        newTimelines[y + 1] += timelines[y];
                        break;
                    default:
                        newTimelines[y] += timelines[y];
                        break;
                }
            }
            
            timelines = newTimelines;
        }
        
        return (splits, timelines.Sum());
    }
}