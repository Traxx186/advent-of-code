using AdventOfCode.AoC2024.Solution;

namespace Test.AoC2024;

[TestFixture]
public class Day06Test
{
    private readonly Day06 _day6 = new();

    private readonly string _inputFile = Path.Combine("AoC2024", "Data", "day_06.txt");

    [Test]
    public void TestPart1()
    {
        Assert.That(_day6.Part1(_inputFile), Is.EqualTo("41"));
    }

    [Test]
    public void TestPart2()
    {
        Assert.That(_day6.Part2(_inputFile), Is.EqualTo("6"));
    }
}