using AdventOfCode.Core;

namespace AdventOfCode.AoC2023.Solution;

public class Day01 : ISolution
{
    public string Name => "Trebuche?!";
    
    private Dictionary<string, int> _numbersMap = new()
    {
        {"one", 1},
        {"two", 2},
        {"three", 3},
        {"four", 4},
        {"five", 5},
        {"six", 6},
        {"seven", 7},
        {"eight", 8}, 
        {"nine", 9},
    };
    
    public string Part1(string inputFile)
    {
        var data = Calendar.LoadInput(inputFile);
        var result = data.Split(Environment.NewLine, StringSplitOptions.RemoveEmptyEntries)
            .Select(line =>
            {
                var firstNumber = line.First(char.IsAsciiDigit) - 48;
                var secondNumber = line.Last(char.IsAsciiDigit) - 48;
                return firstNumber * 10 + secondNumber;
            })
            .Sum();
        
        return result.ToString();
    }

    public string Part2(string inputFile)
    {
        var data = Calendar.LoadInput(inputFile);
        var result = data.Split(Environment.NewLine, StringSplitOptions.RemoveEmptyEntries)
            .Select(line =>
            {
                var firstNumber = FindFirstNumber(line);
                var secondNumber = FindLastNumber(line);
                return firstNumber * 10 + secondNumber;
            })
            .Sum();
        
        return result.ToString();
    }

    private int FindFirstNumber(string line)
    {
        var searchedString = string.Empty;
        
        foreach (var character in line)
        {
            if (char.IsAsciiDigit(character))
                return character - 48;
            
            searchedString += character;
            foreach (var number in _numbersMap)
            {
                if (searchedString.Contains(number.Key))
                    return number.Value;
            }
        }
        
        return 0;
    }
    
    private int FindLastNumber(string line)
    {
        var searchedString = string.Empty;
        
        foreach (var character in line.Reverse())
        {
            if (char.IsAsciiDigit(character))
                return character - 48;

            searchedString = searchedString.Insert(0, character.ToString());
            foreach (var number in _numbersMap)
            {
                if (searchedString.Contains(number.Key))
                    return number.Value;
            }
        }
        
        return 0;
    }
}