using AdventOfCode.AoC2024.Solution;

namespace Test.AoC2024;

[TestFixture]
public class Day09Test
{
    private readonly Day09 _day9 = new();

    private readonly string _inputFile = Path.Combine("AdventOfCode.AoC2024", "Data", "day_09.txt");

    [Test]
    public void TestPart1()
    {
        Assert.That(_day9.Part1(_inputFile), Is.EqualTo("1928"));
    }

    [Test]
    public void TestPart2()
    {
        Assert.That(_day9.Part2(_inputFile), Is.EqualTo("2858"));
    }
}