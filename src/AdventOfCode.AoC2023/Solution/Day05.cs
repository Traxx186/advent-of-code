using AdventOfCode.Core;
using AdventOfCode.Core.Numerics;

namespace AdventOfCode.AoC2023.Solution;

public class Day05 : ISolution
{
    public string Name => "If You Give A Seed A Fertilizer";

    public string Part1(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var almanac = ParseInput(input);

        var sources = almanac.Maps
            .Select(m => (m.Source, m))
            .ToDictionary(m => m.Source, m => m.m);

        var minLocation = almanac.Seeds.Select(seed => FindLocation(sources, seed))
            .Prepend(int.MaxValue)
            .Min();

        return minLocation.ToString();
    }

    public string Part2(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var almanac = ParseInput(input);

        var newSeeds = new List<long>();
        foreach (var chunk in almanac.Seeds.Chunk(2))
        {
            var chunkRange = new RangeLong(chunk[0], chunk[0] + chunk[1]);
            newSeeds.AddRange(chunkRange);
        }
        
        almanac.Seeds = newSeeds;
        var sources = almanac.Maps
            .Select(m => (m.Source, m))
            .ToDictionary(m => m.Source, m => m.m);
        
        var minLocation = almanac.Seeds.Select(seed => FindLocation(sources, seed))
            .Prepend(int.MaxValue)
            .Min();

        return minLocation.ToString();
    }

    private Almanac ParseInput(string input)
    {
        var lines =  input.Split(Environment.NewLine, StringSplitOptions.TrimEntries);
        var seeds = ParseSeeds(lines.First());
        var maps = ParseMaps(lines);
        
        return new Almanac(seeds, maps);
    }

    private static List<long> ParseSeeds(string input)
    {
        return input.Replace("seeds: ", string.Empty)
            .Split(" ")
            .Select(long.Parse)
            .ToList();
    }

    private static List<Map> ParseMaps(string[] input)
    {
        var maps = new List<Map>();
        var chunks = new List<string>();

        foreach (var line in input.Skip(2))
        {
            if (string.IsNullOrWhiteSpace(line))
            {
                maps.Add(new Map(chunks));
                chunks.Clear();
            }
            else
            {
                chunks.Add(line);
            }
        }

        if (chunks.Count != 0)
            maps.Add(new Map(chunks));
        
        return maps;
    }

    private long FindLocation(Dictionary<string, Map> sourceMaps, long seed)
    {
        var currentMap = sourceMaps["seed"];
        var currentValue = seed;

        while (!currentMap.Destination.Equals("location", StringComparison.InvariantCultureIgnoreCase))
        {
            currentValue = currentMap.Convert(currentValue);
            currentMap = sourceMaps[currentMap.Destination];
        }
        
        return currentMap.Convert(currentValue);
    }

    private record Almanac(List<long> Seeds, List<Map> Maps)
    {
        public List<long> Seeds = Seeds; 
        public List<Map> Maps = Maps;
    }
    
    private class Map
    {
        public string Source;
        public string Destination;
        public List<MapRange> Ranges = [];

        public Map(List<string> chunks)
        {
            var nameLine = chunks.First()
                .Replace("map:", string.Empty)
                .Trim();
            
            var names =  nameLine.Split("-to-");
            
            foreach (var line in chunks.Skip(1))
            {
                var data = line.Split(' ')
                    .Select(long.Parse)
                    .ToArray();
                
                var range = new RangeLong(data[1], data[1] + data[2]);
                Ranges.Add(new MapRange(range, data[0] - data[1]));
            }
            
            Source = names.First();
            Destination = names.Last();
        }

        public long Convert(long source)
        {
            var destination = source;
            
            foreach (var range in Ranges)
            {
                if (range.Range.Contains(source))
                    destination = source + range.DesOffset;
            }
            
            return destination;
        }
    }

    private record MapRange(RangeLong Range, long DesOffset);
}