using AdventOfCode.AoC2024.Solution;

namespace Test.AoC2024;

[TestFixture]
public class Day12Test
{
    private readonly Day12 _day12 = new();

    private readonly string _inputFile = Path.Combine("AoC2024", "Data", "day_12.txt");

    [Test]
    public void TestPart1()
    {
        Assert.That(_day12.Part1(_inputFile), Is.EqualTo("1930"));
    }
    
    [Test]
    public void TestPart2()
    {
        Assert.That(_day12.Part2(_inputFile), Is.EqualTo("1206"));
    }
}