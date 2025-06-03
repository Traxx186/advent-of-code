using AdventOfCode.AoC2024.Solution;

namespace Test.AoC2024;

[TestFixture]
public class Day10Test
{
    private readonly Day10 _day10 = new();

    private readonly string _inputFile = Path.Combine("AdventOfCode.AoC2024", "Data", "day_10.txt");

    [Test]
    public void TestPart1()
    {
        Assert.That(_day10.Part1(_inputFile), Is.EqualTo("36"));
    }

    [Test]
    public void TestPart2()
    {
        Assert.That(_day10.Part2(_inputFile), Is.EqualTo("81"));
    }
}