using System.Numerics;
using AdventOfCode.Core;

namespace AdventOfCode.AoC2025.Solution;

public class Day08 : ISolution
{
    public string Name => "Playground";
    
    public string Part1(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var junctionBoxes = ParseJunctionBoxes(input);
        var circuits = junctionBoxes.ToDictionary(box => box, box => new HashSet<Vector3>([box]));
        
        foreach (var (a, b) in PairJunctionBoxes(junctionBoxes).Take(1000))
        {
            if (circuits[a] == circuits[b])
                continue;
            
            Connect(a, b, circuits);
        }
        
        return circuits.Values
            .Distinct()
            .OrderByDescending(circuit => circuit.Count)
            .Take(3)
            .Aggregate(1, (a, b) => a * b.Count)
            .ToString();
    }

    public string Part2(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var junctionBoxes = ParseJunctionBoxes(input);
        var circuits = junctionBoxes.ToDictionary(box => box, box => new HashSet<Vector3>([box]));
        var junctionsCount = junctionBoxes.Length;

        var result = 0m;
        foreach (var (a, b) in PairJunctionBoxes(junctionBoxes).TakeWhile(_ => junctionsCount > 1))
        {
            if (circuits[a] == circuits[b])
                continue;
            
            Connect(a, b, circuits);
            result = (decimal)a.X * (decimal)b.X;
            junctionsCount--;
        }
        
        return result.ToString("0");
    }

    private static Vector3[] ParseJunctionBoxes(string input)
    {
        return input.Split(Environment.NewLine, StringSplitOptions.RemoveEmptyEntries)
            .Select(line => line.Split(','))
            .Select(split => new Vector3(float.Parse(split[0]), float.Parse(split[1]), float.Parse(split[2])))
            .ToArray();
    }

    private static (Vector3 a, Vector3 b)[] PairJunctionBoxes(Vector3[] junctionBoxes)
    {
        return junctionBoxes.SelectMany(a => junctionBoxes
                .Where(b => (a.X, a.Y, a.Z).CompareTo((b.X, b.Y, b.Z)) < 0)
                .Select(b => (a, b))
            )
            .OrderBy(t => Vector3.Distance(t.a, t.b))
            .ToArray();
    }

    private static void Connect(Vector3 a, Vector3 b, Dictionary<Vector3, HashSet<Vector3>> circuits)
    {
        circuits[a].UnionWith(circuits[b]);
        foreach (var pair in circuits[b])
            circuits[pair] = circuits[a];
    }
}