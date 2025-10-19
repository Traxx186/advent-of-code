using AdventOfCode.AoC2024.Solution;

namespace Test.AoC2024;

[TestFixture]
public class Day14Test
{
    private readonly Day14 _day14 = new();

    private readonly string _inputFile = Path.Combine("AoC2024", "Data", "day_14.txt");

    [Test]
    public void TestPart1()
    {
        Assert.That(_day14.Part1(_inputFile), Is.EqualTo("21"));
    }
}