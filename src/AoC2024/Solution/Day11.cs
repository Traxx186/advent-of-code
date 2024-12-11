using AdventOfCode.Core;

namespace AdventOfCode.AoC2024.Solution;

public class Day11 : ISolution
{
    public string Name => "Day 11";
    
    public string Part1(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var cache = new Dictionary<long, List<long>>();
        var stones = input.Split(' ')
            .Select(long.Parse)
            .ToDictionary(num => num, _ => 1L);

        for (var i = 0; i < 25; i++)
        {
            var blink = new Dictionary<long, long>();
            foreach (var (engraving, multiplier) in stones)
            {
                var existingStones = ChangeStones(engraving, cache)
                    .Where(newStone => !blink.TryAdd(newStone, multiplier));
                
                foreach (var existingStone in existingStones)
                    blink[existingStone] += multiplier;
            }
            
            stones = blink;
        }

        var total = stones.Sum(stone => stone.Value);
        return total.ToString();
    }

    public string Part2(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var cache = new Dictionary<long, List<long>>();
        var stones = input.Split(' ')
            .Select(long.Parse)
            .ToDictionary(num => num, _ => 1L);

        for (var i = 0; i < 75; i++)
        {
            var blink = new Dictionary<long, long>();
            foreach (var (engraving, multiplier) in stones)
            {
                var existingStones = ChangeStones(engraving, cache)
                    .Where(newStone => !blink.TryAdd(newStone, multiplier));
                
                foreach (var existingStone in existingStones)
                    blink[existingStone] += multiplier;
            }
            
            stones = blink;
        }

        var total = stones.Sum(stone => stone.Value);
        return total.ToString();
    }

    private static List<long> ChangeStones(long engraving, Dictionary<long, List<long>> cache)
    {
        if (cache.TryGetValue(engraving, out var cached))
            return cached;
        
        var newStones = new List<long>();
        if (engraving == 0)
            newStones.Add(1);
        else if (Math.Floor(Math.Log10(engraving) + 1) % 2 == 0)
        {
            var numString = engraving.ToString();
            var left = numString[..(numString.Length / 2)];
            var right = numString[(numString.Length / 2)..];
                    
            newStones.Add(long.Parse(left));
            newStones.Add(long.Parse(right)); 
        }
        else
            newStones.Add(engraving * 2024);
        
        cache.Add(engraving, newStones);
        return newStones;
    }
}