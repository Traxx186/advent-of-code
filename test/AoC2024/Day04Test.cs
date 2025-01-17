using AdventOfCode.AoC2024.Solution;

namespace Test.AoC2024;

[TestFixture]
public class Day04Test
{
    private readonly Day04 _day4 = new();

    private readonly string _inputFile = Path.Combine("AoC2024", "Data", "day_04.txt");

    [Test]
    public void TestPart1()
    {
        Assert.That(_day4.Part1(_inputFile), Is.EqualTo("18"));
    }

    [Test]
    public void TestPart2()
    {
        Assert.That(_day4.Part2(_inputFile), Is.EqualTo("9"));
    }
}
