using AdventOfCode.Core;

namespace AdventOfCode.AoC2025.Solution;

public class Day03 : ISolution
{
    public string Name => "Lobby";

    public string Part1(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var banks = input.Split(Environment.NewLine, StringSplitOptions.RemoveEmptyEntries);
        var outputJoltage = banks.Select(bank => CalculateJolt(bank, 2)).Sum();
        
        return outputJoltage.ToString();
    }

    public string Part2(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var banks = input.Split(Environment.NewLine, StringSplitOptions.RemoveEmptyEntries);
        var outputJoltage = banks.Select(bank => CalculateJolt(bank, 12)).Sum();
        
        return outputJoltage.ToString();
    }

    private static long CalculateJolt(string bank, int numBatteries)
    {
        var joltage = 0L;
        for (; numBatteries > 0; numBatteries--)
        {
            var maxJolt = bank[..^(numBatteries - 1)].Max();
            bank = bank[(bank.IndexOf(maxJolt) + 1)..];
            joltage = 10 * joltage + (maxJolt - '0');
        }
        
        return joltage;
    }
}