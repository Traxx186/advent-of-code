using System.Numerics;
using System.Text;
using System.Text.RegularExpressions;
using AdventOfCode.Core;

namespace AdventOfCode.AoC2024.Solution;

public partial class Day14 : ISolution
{
    private const float Width = 101f;
    private const float Height = 103f;
    
    public string Name => "Restroom Redoubt";
    
    [GeneratedRegex(@"-?\d{1,}", RegexOptions.Compiled, "en-US")]
    private static partial Regex NumberRegex();
    
    public string Part1(string inputFile)
    {
        const int seconds = 100;

        var input = Calendar.LoadInput(inputFile);
        var robots = ParseInput(input);
        var center = new Vector2(MathF.Floor(Width / 2), MathF.Floor(Height / 2));
        var q1 = 0;
        var q2 = 0;
        var q3 = 0;
        var q4 = 0;

        foreach (var (position, velocity) in robots)
        {
            var positionX = (position.X + velocity.X * seconds + Width * seconds) % Width;
            var positionY = (position.Y + velocity.Y * seconds + Height * seconds) % Height;

            if (center.X == positionX || center.Y == positionY)
                continue;

            if (positionX < center.X && positionY < center.Y)
                q1++;
            else if (positionX > center.X && positionY < center.Y)
                q2++;
            else if (positionX < center.X && positionY > center.Y)
                q3++;
            else if (positionX > center.X && positionY > center.Y)
                q4++;
        }
        
        return (q1 * q2 * q3 * q4).ToString();
    }

    public string Part2(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var robots = ParseInput(input);
        var treeVissible = false;
        var seconds = 0;

        while (!treeVissible)
        {
            seconds++;
            
            var positions = new HashSet<Vector2>();
            foreach (var (position, velocity) in robots)
            {
                var positionX = (position.X + velocity.X * seconds + Width * seconds) % Width;
                var positionY = (position.Y + velocity.Y * seconds + Height * seconds) % Height;
               positions.Add(new Vector2(positionX, positionY));
            }

            if (positions.Count != robots.Count)
                continue;

            var output = new StringBuilder();
            for (var y = 0; y < Height; y++)
            {
                for (var x = 0; x < Width; x++)
                {
                    var search = new Vector2(x, y);
                    output.Append(positions.Contains(search) ? '#' : '.');
                }
                
                output.AppendLine();
            }
            
            Console.WriteLine(output.ToString());
            Console.Write("Tree visible? (Y/N): ");
            var answer = Console.ReadLine();
            treeVissible = answer != null && answer.ToLower().StartsWith('y');
        }

        return seconds.ToString();
    }

    private static List<(Vector2, Vector2)> ParseInput(string input)
    {
        var lines = input.Split(Environment.NewLine, StringSplitOptions.RemoveEmptyEntries);
        
        return lines.Select(line => NumberRegex()
                .Matches(line)
                .Select(m => int.Parse(m.Value))
                .ToArray())
            .Select(matches => (new Vector2(matches[0], matches[1]), new Vector2(matches[2], matches[3])))
            .ToList();
    }
}