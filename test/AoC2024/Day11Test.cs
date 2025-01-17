using AdventOfCode.AoC2024.Solution;

namespace Test.AoC2024;

[TestFixture]
public class Day11Test
{
    private readonly Day11 _day11 = new();

    private readonly string _inputFile = Path.Combine("AoC2024", "Data", "day_11.txt");

    [Test]
    public void TestPart1()
    {
        Assert.That(_day11.Part1(_inputFile), Is.EqualTo("55312"));
    }
}