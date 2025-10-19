using AdventOfCode.Core;

namespace AdventOfCode.AoC2023.Solution;

public class Day06 : ISolution
{
    public string Name => "Wait For It";
    
    public string Part1(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var races = ParseRaces(input);
        var winCounts = new List<int>();

        for (var i = 0; i < races.Item1.Count; i++)
        {
            var raceTime =  int.Parse(races.Item1[i]);
            var distanceRecord = int.Parse(races.Item2[i]);
            var winCount = 0;

            for (var j = raceTime - 1; j >= 0; j--)
            {
                var distance = (raceTime - j) * j;
                if (distance > distanceRecord)
                    winCount++;
            }
            
            winCounts.Add(winCount);
        }
        
        return winCounts.Aggregate(1, (current, next) => current * next)
            .ToString();
    }

    public string Part2(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var races = ParseRaces(input);
        var winCount = 0;
        
        var time = long.Parse(string.Join("", races.Item1));
        var distanceRecord = long.Parse(string.Join("", races.Item2));
        
        for (var j = time - 1; j >= 0; j--)
        {
            var distance = (time - j) * j;
            if (distance > distanceRecord)
                winCount++;
        }
        
        return winCount.ToString();
    }

    private (List<string>, List<string>) ParseRaces(string input)
    {
         var parts = input.Split(Environment.NewLine, StringSplitOptions.RemoveEmptyEntries);
         
         var times = parts.First()
             .Split(':', 2, StringSplitOptions.TrimEntries).Last()
             .Split(' ',  StringSplitOptions.RemoveEmptyEntries)
             .ToList();
         
         var distances = parts.Last()
             .Split(':', 2, StringSplitOptions.TrimEntries).Last()
             .Split(' ', StringSplitOptions.RemoveEmptyEntries)
             .ToList();
         
         return (times, distances);
    }
}