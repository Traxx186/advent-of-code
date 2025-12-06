using AdventOfCode.AoC2025.Solution;

namespace Test.AoC2025;

[TestFixture]
public class Day05Test
{
    private readonly Day05 _day5 = new();

    private readonly string _inputFile = Path.Combine("AoC2025", "Data", "day_05.txt");

    [Test]
    public void TestPart1()
    {
        Assert.That(_day5.Part1(_inputFile), Is.EqualTo("3"));
    }

    [Test]
    public void TestPart2()
    {
        Assert.That(_day5.Part2(_inputFile), Is.EqualTo("14"));
    }
}