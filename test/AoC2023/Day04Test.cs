using AdventOfCode.AoC2023.Solution;

namespace Test.AoC2023;

[TestFixture]
public class Day04Test
{
    private readonly Day04 _day4 = new();

    private readonly string _inputFile = Path.Combine("AoC2023", "Data", "day_04.txt");

    [Test]
    public void TestPart1()
    {
        Assert.That(_day4.Part1(_inputFile), Is.EqualTo("13"));
    }

    [Test]
    public void TestPart2()
    {
        Assert.That(_day4.Part2(_inputFile), Is.EqualTo("30"));
    }
}