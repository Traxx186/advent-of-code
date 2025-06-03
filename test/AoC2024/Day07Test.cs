using AdventOfCode.AoC2024.Solution;

namespace Test.AoC2024;

[TestFixture]
public class Day07Test
{
    private readonly Day07 _day7 = new();

    private readonly string _inputFile = Path.Combine("AdventOfCode.AoC2024", "Data", "day_07.txt");

    [Test]
    public void TestPart1()
    {
        Assert.That(_day7.Part1(_inputFile), Is.EqualTo("3749"));
    }

    [Test]
    public void TestPart2()
    {
        Assert.That(_day7.Part2(_inputFile), Is.EqualTo("11387"));
    }
}