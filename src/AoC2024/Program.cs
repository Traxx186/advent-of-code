using AdventOfCode.AoC2024;
using AdventOfCode.AoC2024.Solution;
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
    new Day09(),
    new Day10(),
    new Day11(),
    new Day12(),
]; 

var aoc = new Calendar(2024, solutions);
aoc.Run();