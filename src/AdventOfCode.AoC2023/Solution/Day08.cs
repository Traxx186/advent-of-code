using AdventOfCode.Core;
using AdventOfCode.Core.Numerics;

namespace AdventOfCode.AoC2023.Solution;

public class Day08 : ISolution
{
    public string Name => "Haunted Wasteland";

    public string Part1(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var parts = input.Split(Environment.NewLine + Environment.NewLine, count: 2, StringSplitOptions.RemoveEmptyEntries);

        var instructions = parts[0].Select(ParseInputLine).ToList();
        var nodes = parts[1].Split(Environment.NewLine, StringSplitOptions.RemoveEmptyEntries)
            .Select(l => new Node(l))
            .ToList();

        var startNode = nodes.Single(n => n.Id == "AAA");
        var steps = CountSteps(instructions, nodes, startNode, s => s == "ZZZ");

        return steps.ToString();
    }

    public string Part2(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var parts = input.Split(Environment.NewLine + Environment.NewLine, count: 2, StringSplitOptions.RemoveEmptyEntries);

        var instructions = parts[0].Select(ParseInputLine).ToList();
        var nodes = parts[1].Split(Environment.NewLine, StringSplitOptions.RemoveEmptyEntries)
            .Select(l => new Node(l))
            .ToList();

        var steps = nodes.Where(n => n.Id.EndsWith('A'))
            .Select(n => CountSteps(instructions, nodes, n, s => s.EndsWith('Z')))
            .ToList();
        
        return steps.Aggregate(0, (current, next) => current * next / Integer.GreatestCommonDivisor(current, next))
            .ToString();
    }

    private static int CountSteps(
        List<Instruction> instructions,
        List<Node> nodes,
        Node startNode,
        Predicate<string> endCondition
    )
    {
        var steps = 0;
        var node = startNode;

        for (var i = 0; i < instructions.Count ; i++)
        {
            var instruction = instructions[i];
            var target = instruction switch
            {
                Instruction.Left => node.Left,
                Instruction.Right => node.Right,
                _ => throw new ArgumentOutOfRangeException(nameof(instruction), instruction, null)
            };
            
            steps++;
            if (endCondition(target))
                break;
            
            node = nodes.Single(n => n.Id == target);
            
            // To loop again over the instruction list, reset the index
            if (instructions.Count == (i + 1))
                i = -1;
        }
        
        return steps;
    }

    private class Node
    {
        public string Id;
        public string Left;
        public string Right;

        public Node(string line)
        {
            var parts = line.Split(" = ", count: 2, StringSplitOptions.RemoveEmptyEntries);
            var directionParts = parts[1].Replace("(", string.Empty)
                .Replace(")", string.Empty)
                .Split(", ");

            Id = parts[0];
            Left = directionParts[0];
            Right = directionParts[1];
        }
    }

    private enum Instruction
    {
        Left,
        Right
    }

    private static Instruction ParseInputLine(char instruction)
    {
        return instruction switch
        {
            'L' => Instruction.Left,
            'R' => Instruction.Right,
            _ => throw new ArgumentOutOfRangeException(nameof(instruction), instruction, null)
        };
    }
}