using AdventOfCode.AoC2023.Solution;
using AdventOfCode.Core;

ISolution[] solutions =
[
    new Day01(),
    new Day02(),
    new Day03(),
    new Day04(),
    new Day05(),
    new Day06(),
    new Day07(),
    new Day08(),
];

var aoc = new Calendar(2023, solutions);
aoc.Run();