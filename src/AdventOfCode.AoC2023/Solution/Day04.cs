using System.Text.RegularExpressions;
using AdventOfCode.Core;

namespace AdventOfCode.AoC2023.Solution;

public partial class Day04 : ISolution
{
    [GeneratedRegex(@"\d+", RegexOptions.IgnoreCase)]
    private static partial Regex NumberRegex(); 
    
    public string Name => "Scratchcards";
    public string Part1(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var games = input.Split(Environment.NewLine, StringSplitOptions.RemoveEmptyEntries)
            .Select(GenerateGame)
            .ToArray();

        var sum = 0;
        foreach (var game in games)
        {
            var matching = game.ScratchNumbers.Count(s => game.WinningNumbers.Contains(s));
            
            sum += matching > 0 
                ? (int)Math.Pow(2, Math.Max(matching - 1, 0)) 
                : 0;
        }
        
        return sum.ToString();
    }

    public string Part2(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var games = input.Split(Environment.NewLine, StringSplitOptions.RemoveEmptyEntries)
            .Select(GenerateGame)
            .ToArray();
        
        var count = Enumerable.Repeat(1, games.Length).ToArray();
        for (var i = 0; i < games.Length; i++)
        {
            var game = games[i];
            var matching = game.ScratchNumbers.Count(s => game.WinningNumbers.Contains(s));

            foreach (var j in Enumerable.Range((i + 1), matching))
                count[j] += count[i];
        }
        
        return count.Sum().ToString();
    }

    private Game GenerateGame(string line)
    {
        var gameInfo = line.Split(':', count: 2);
        var numbers = gameInfo.Last().Split(" | ", count: 2);
        var id = int.Parse(NumberRegex().Match(numbers.First()).Value);
        
        var winningNumbers = numbers.First()
            .Split(' ', StringSplitOptions.RemoveEmptyEntries)
            .Select(int.Parse)
            .ToArray();
        
        var scratchNumbers = numbers.Last()
            .Split(' ', StringSplitOptions.RemoveEmptyEntries)
            .Select(int.Parse)
            .ToArray();
        
        return new Game { Id = id, WinningNumbers = winningNumbers, ScratchNumbers = scratchNumbers };
    }
    
    private struct Game
    {
        public int Id;
        public int[] WinningNumbers;
        public int[] ScratchNumbers;
    }
}

