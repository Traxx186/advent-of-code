using AdventOfCode.AoC2025.Solution;
using AdventOfCode.Core;

ISolution[] solutions =
[
    new Day01(),
    new Day02(),
    new Day03(),
];

var aoc = new Calendar(2025, solutions);
aoc.Run();