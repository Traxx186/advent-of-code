using AdventOfCode.AoC2024.Solution;

namespace Test.AoC2024;

[TestFixture]
public class Day15Test
{
    private readonly Day15 _day15 = new();

    private readonly string _inputFile = Path.Combine("AoC2024", "Data", "day_15.txt");

    [Test]
    public void TestPart1()
    {
        Assert.That(_day15.Part1(_inputFile), Is.EqualTo("10092"));
    }
    
    [Test]
    public void TestPart2()
    {
        Assert.That(_day15.Part2(_inputFile), Is.EqualTo("9021"));
    }
}