using AdventOfCode.Aoc2024.Solution;
using AdventOfCode.AoC2024.Solution;
using AdventOfCode.Core;

ISolution[] solutions =
[
    new Day01(),
    new Day02()
]; 

var aoc = new Calendar(2024, solutions);
aoc.Run();