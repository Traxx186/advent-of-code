using AdventOfCode.AoC2023.Solution;

namespace Test.AoC2023;

[TestFixture]
public class Day05Test
{
    private readonly Day05 _day5 = new();

    private readonly string _inputFile = Path.Combine("AoC2023", "Data", "day_05.txt");

    [Test]
    public void TestPart1()
    {
        Assert.That(_day5.Part1(_inputFile), Is.EqualTo("35"));
    }

    [Test]
    public void TestPart2()
    {
        Assert.That(_day5.Part2(_inputFile), Is.EqualTo("46"));
    }
}