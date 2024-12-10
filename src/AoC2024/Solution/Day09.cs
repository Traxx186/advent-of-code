using AdventOfCode.Core;

namespace AdventOfCode.AoC2024.Solution;

public class Day09 : ISolution
{
    public string Name => "Day 9";
    
    public string Part1(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var fileSystem = ParseInput(input);

        var start = 0;
        var end = fileSystem.Length - 1;
        var spaceNeeded = fileSystem[end];
        var index = 0;
        var checksum = 0L;
        
        while (start < end)
        {
            for (var i = 0; i < fileSystem[start]; i++)
            {
                checksum += (long)(start / 2) * index;
                index++;
            }
            
            start++;
            for (var i = 0; i < fileSystem[start]; i++)
            {
                if (spaceNeeded == 0)
                {
                    end -= 2;
                    if (end <= start)
                        break;
                    spaceNeeded = fileSystem[end];
                }
                
                checksum += (long)(end / 2) * index;
                index++;
                spaceNeeded--;
            }
            
            start++;
        }

        for (var i = 0; i < spaceNeeded; i++)
        {
            checksum += (long)(end / 2) * index;
            index++;
        }
        
        return checksum.ToString();
    }

    public string Part2(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var fileSystem = ParseInput(input);
        
        var openPlaces = new int[fileSystem.Length];
        openPlaces[0] = 0;
        for (var i = 1; i < fileSystem.Length; i++)
            openPlaces[i] = openPlaces[i - 1] + fileSystem[i - 1];
        
        var checksum = 0L;
        for (var end = fileSystem.Length - 1; end >= 0; end -= 2)
        {
            var found = false;
            for (var start = 1; start < end; start += 2)
            {
                if (fileSystem[start] < fileSystem[end]) continue;
                
                for (var i = 0; i < fileSystem[end]; i++)
                    checksum += (long)(end / 2) * (openPlaces[start] + i);
                    
                fileSystem[start] -= fileSystem[end];
                openPlaces[start] += fileSystem[end];
                found = true;
                break;
            }
            
            if (found) 
                continue;
            
            for (var i = 0; i < fileSystem[end]; i++)
                checksum += (long)(end / 2) * (openPlaces[end] + i);
        }
        
        return checksum.ToString();
    }

    private static int[] ParseInput(string input)
    {
        return input.Select(c => c - '0')
            .ToArray();
    }
}