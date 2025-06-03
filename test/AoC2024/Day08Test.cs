using AdventOfCode.AoC2024.Solution;

namespace Test.AoC2024;

[TestFixture]
public class Day08Test
{
    private readonly Day08 _day8 = new();

    private readonly string _inputFile = Path.Combine("AdventOfCode.AoC2024", "Data", "day_08.txt");

    [Test]
    public void TestPart1()
    {
        Assert.That(_day8.Part1(_inputFile), Is.EqualTo("14"));
    }

    [Test]
    public void TestPart2()
    {
        Assert.That(_day8.Part2(_inputFile), Is.EqualTo("34"));
    }
}