using AdventOfCode.Aoc2024.Solution;

namespace Test.AoC2024;

[TestFixture]
public class Day1Test
{
    private readonly Day1 _day1;

    public Day1Test()
    {
        _day1 = new Day1();
    }
    
    [Test]
    public void TestPart1()
    {
        var inputFile = Path.Combine("AoC2024", "Data", "day_01.txt");
        Assert.That(_day1.Part1(inputFile), Is.EqualTo("Part 1"));
    }
}