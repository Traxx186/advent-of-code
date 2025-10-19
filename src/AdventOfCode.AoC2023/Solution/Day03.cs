using AdventOfCode.Core;

namespace AdventOfCode.AoC2023.Solution;

public class Day03 : ISolution
{
    public string Name => "Gear Ratios";
    
    public string Part1(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var lines = input.Split(Environment.NewLine, StringSplitOptions.RemoveEmptyEntries);

        var partNumbers = new List<int>();
        for (var i = 0; i < lines.Length; i++)
        {
            var currentLine = lines[i];
            var parts = GetPartsFromLine(currentLine);

            foreach (var part in parts)
            {
                var left = Math.Max(part.Start - 1, 0);
                var right = (part.End + 1) >= currentLine.Length 
                    ? currentLine.Length - 1 
                    : part.End + 1;
                
                var hasSymbolLeft = IsSymbol(currentLine[left]);
                var hasSymbolRight = IsSymbol(currentLine[right]);
                
                var hasSymbolTop = lines[Math.Max(i - 1, 0)]
                    .Substring(left, right - left + 1)
                    .Any(IsSymbol);
                
                var hasSymbolBottom = lines[Math.Min(i + 1, lines.Length - 1)]
                    .Substring(left, right - left + 1)
                    .Any(IsSymbol);
                
                if (hasSymbolLeft || hasSymbolRight || hasSymbolTop || hasSymbolBottom)
                    partNumbers.Add(part.Value);
            }
        }
        
        return partNumbers.Sum().ToString();
    }

    public string Part2(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var lines = input.Split(Environment.NewLine, StringSplitOptions.RemoveEmptyEntries);
        var parts = lines.Select(GetPartsFromLine).ToArray();
        var sum = 0;

        for (var y = 0; y < lines.Length; y++)
        {
            var currentLine = lines[y];

            for (var x = 0; x < currentLine.Length; x++)
            {
                var currentChar = currentLine[x];
                if (!IsSymbol(currentChar))
                    continue;

                var foundParts = new List<PartNumber>();
                var left = Math.Max(x - 1, 0);
                var right = (x + 1) >= currentLine.Length 
                    ? currentLine.Length - 1 
                    : x + 1;
                
                var top = Math.Max(y - 1, 0);
                var bottom = (y + 1) >= lines.Length 
                    ? lines.Length - 1 
                    : y + 1;

                foundParts.AddRange([
                    parts[y].FirstOrDefault(p => p.End == left),
                    parts[y].FirstOrDefault(p => p.Start == right),
                    parts[top].FirstOrDefault(p => Enumerable.Range(p.Start, p.End - p.Start + 1).Contains(x)),
                    parts[top].FirstOrDefault(p => p.End == left),
                    parts[top].FirstOrDefault(p =>  p.Start == right),
                    parts[bottom].FirstOrDefault(p => Enumerable.Range(p.Start, p.End - p.Start + 1).Contains(x)),
                    parts[bottom].FirstOrDefault(p => p.End == left),
                    parts[bottom].FirstOrDefault(p =>  p.Start == right),
                ]);
                
                var adjacentParts = foundParts.Where(part => part.Value != 0)
                    .ToArray();
                
                if (adjacentParts.Length < 2)
                    continue;

                sum += adjacentParts.Select(p => p.Value)
                    .Aggregate(1, (a, b) => a * b);
            }
        }
        
        return sum.ToString();
    }

    private PartNumber[] GetPartsFromLine(string line)
    {
        var result = new List<PartNumber>();
        var numberString = string.Empty;
        var start = 0;
        var end = 0;

        for (var i = 0; i < line.Length; i++)
        {
            var c = line[i];
            if (char.IsNumber(c))
            {
                if (string.IsNullOrWhiteSpace(numberString))
                    start = i;
                
                end = i;
                numberString += c;
            }
            else if (!string.IsNullOrWhiteSpace(numberString))
            {
                var value = int.Parse(numberString);
                
                result.Add(new PartNumber { Start = start, End = end, Value = value });
                numberString = string.Empty;
                start = 0;
                end = 0;
            }
        }

        if (string.IsNullOrWhiteSpace(numberString))
            return result.ToArray();
        
        result.Add(new PartNumber { Start = start, End = end, Value = int.Parse(numberString) });

        return result.ToArray();
    }

    private bool IsSymbol(char c) => !char.IsNumber(c) && c != '.';
    
    private struct PartNumber
    {
        public int Start;
        public int End;
        public int Value;
    };
}
