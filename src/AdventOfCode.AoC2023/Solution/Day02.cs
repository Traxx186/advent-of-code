using AdventOfCode.Core;

namespace AdventOfCode.AoC2023.Solution;

public class Day02 : ISolution
{
    public string Name => "Cube Conundrum";
    
    public string Part1(string inputFile)
    {
        var bag = new Dictionary<Color, int>
        {
            { Color.Blue, 14 },
            { Color.Green, 13 },
            { Color.Red, 12 },
        };
        
        var input = Calendar.LoadInput(inputFile);
        var games = input.Split(Environment.NewLine, StringSplitOptions.RemoveEmptyEntries)
            .Select(l => IsGamePossible(new Game(l), bag))
            .Sum();
        
        return games.ToString();
    }

    public string Part2(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var games = input.Split(Environment.NewLine, StringSplitOptions.RemoveEmptyEntries)
            .Select(l => new Game(l))
            .ToArray();

        var total = games.Select(GetSmallestBag)
            .Select(bag => bag.Values.Aggregate(1, (a, b) => a * b))
            .Sum();

        return total.ToString();
    }

    private int IsGamePossible(Game game, Dictionary<Color, int> bag)
    {
        var gamePossible = game.Reveals
            .SelectMany(r => r)
            .All(r => r.Value <= bag[r.Key]);

        return gamePossible ? game.Id : 0;
    }

    private Dictionary<Color, int> GetSmallestBag(Game game)
    {
        var bag = new Dictionary<Color, int>();
        var reveals = game.Reveals.SelectMany(r => r).ToArray();

        foreach (var reveal in reveals)
        {
            if (!bag.TryGetValue(reveal.Key, out var max))
            {
                bag.Add(reveal.Key, reveal.Value);
                max = reveal.Value;
            }
            
            if (max < reveal.Value)
                bag[reveal.Key] = reveal.Value;
        }
        
        return bag;
    }
    
    [Flags]
    private enum Color
    {
        Red,
        Green,
        Blue
    }

    private class Game
    {
        public int Id;
        public List<Dictionary<Color, int>> Reveals;

        public Game(string line)
        {
            var parts = line.Split(':', count: 2);
        
            Id = int.Parse(new string(parts.First().Where(char.IsAsciiDigit).ToArray()));
            Reveals = parts.Last()
                .Split(';')
                .Select(r => r.Split(',')
                    .Select(ParseColorReveal)
                    .ToDictionary(c => c.Item1, c => c.Item2)
                )
                .ToList();
        }

        private (Color, int) ParseColorReveal(string reveal)
        {
            var parts = reveal.Trim().Split(' ', count: 2);
        
            return (Enum.Parse<Color>(parts.Last(), true), int.Parse(parts.First()));
        }
    }
}

