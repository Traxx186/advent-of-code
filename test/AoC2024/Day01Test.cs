using AdventOfCode.AoC2024.Solution;

namespace Test.AoC2024;

[TestFixture]
public class Day01Test
{
    private readonly Day01 _day1 = new();

    private readonly string _inputFile = Path.Combine("AoC2024", "Data", "day_01.txt");

    [Test]
    public void TestPart1()
    {
        Assert.That(_day1.Part1(_inputFile), Is.EqualTo("11"));
    }

    [Test]
    public void TestPart2()
    {
        Assert.That(_day1.Part2(_inputFile), Is.EqualTo("31"));
    }
}