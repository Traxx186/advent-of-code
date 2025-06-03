using System.Numerics;
using System.Text.RegularExpressions;
using AdventOfCode.Core;

namespace AdventOfCode.AoC2024.Solution;

public partial class Day13 : ISolution
{
    public string Name => "Claw Contraption";
    
    [GeneratedRegex(@"-?\d{1,}", RegexOptions.Compiled, "en-US")]
    private static partial Regex NumberRegex();
    
    public string Part1(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var games = ParseInput(input);
        var total = 0;

        foreach (var game in games)
        {
            var y = (game.Prize.Y * game.ButtonA.X - game.Prize.X * game.ButtonA.Y) 
                    / (game.ButtonB.Y * game.ButtonA.X - game.ButtonB.X * game.ButtonA.Y);
            
            var x = (game.Prize.X - y * game.ButtonB.X) / game.ButtonA.X;
            
            if (x % 1 != 0 || y % 1 != 0)
                continue;
            
            total += (int)x * 3 + (int)y;
        }
        
        return total.ToString();
    }

    public string Part2(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var games = ParseInput(input);
        var total = 0L;
        
        foreach (var game in games)
        {
            var px = (long)game.Prize.X + 10_000_000_000_000;
            var py = (long)game.Prize.Y + 10_000_000_000_000;
            
            var y = (py * (double)game.ButtonA.X - px * (double)game.ButtonA.Y) 
                    / (game.ButtonB.Y * (double)game.ButtonA.X - game.ButtonB.X * (double)game.ButtonA.Y);
            
            var x = (px - y * game.ButtonB.X) / game.ButtonA.X;
            
            if (x % 1 != 0 || y % 1 != 0)
                continue;
            
            total += (long)x * 3 + (long)y;
        }
        
        return total.ToString();
    }
    
    private static List<Game> ParseInput(string input)
    {
        var gameInput = input.Split(Environment.NewLine + Environment.NewLine).ToArray();
        var games = new List<Game>();
        
        foreach (var line in gameInput)
        {
            var parts = line.Split(Environment.NewLine).ToArray();
            var buttonA = NumberRegex().Matches(parts[0]).Select(match => int.Parse(match.Value)).ToArray();
            var buttonB = NumberRegex().Matches(parts[1]).Select(match => int.Parse(match.Value)).ToArray();
            var prize = NumberRegex().Matches(parts[2]).Select(match => int.Parse(match.Value)).ToArray();
            
            games.Add(new Game
            {
                ButtonA = new Vector2(buttonA[0], buttonA[1]),
                ButtonB = new Vector2(buttonB[0], buttonB[1]),
                Prize = new Vector2(prize[0], prize[1]),
            });
        }

        return games;
    }
    
    private struct Game
    {
        public Vector2 Prize;
        public Vector2 ButtonA;
        public Vector2 ButtonB;
    }
}