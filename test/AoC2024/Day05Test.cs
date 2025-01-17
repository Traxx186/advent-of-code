using AdventOfCode.AoC2024.Solution;

namespace Test.AoC2024;

[TestFixture]
public class Day05Test
{
    private readonly Day05 _day5 = new();

    private readonly string _inputFile = Path.Combine("AoC2024", "Data", "day_05.txt");

    [Test]
    public void TestPart1()
    {
        Assert.That(_day5.Part1(_inputFile), Is.EqualTo("143"));
    }

    [Test]
    public void TestPart2()
    {
        Assert.That(_day5.Part2(_inputFile), Is.EqualTo("123"));
    }
}