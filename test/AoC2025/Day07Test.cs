using AdventOfCode.AoC2025.Solution;

namespace Test.AoC2025;

[TestFixture]
public class Day07Test
{
    private readonly Day07 _day7 = new();

    private readonly string _inputFile = Path.Combine("AoC2025", "Data", "day_07.txt");

    [Test]
    public void TestPart1()
    {
        Assert.That(_day7.Part1(_inputFile), Is.EqualTo("21"));
    }

    [Test]
    public void TestPart2()
    {
        Assert.That(_day7.Part2(_inputFile), Is.EqualTo("40"));
    }
}