using AdventOfCode.AoC2024.Solution;

namespace Test.AoC2024;

[TestFixture]
public class Day16Test
{
    private readonly Day16 _day16 = new();

    private readonly string _inputFile = Path.Combine("AdventOfCode.AoC2024", "Data", "day_16.txt");

    [Test]
    public void TestPart1()
    {
        Assert.That(_day16.Part1(_inputFile), Is.EqualTo("7036"));
    }
    
    [Test]
    public void TestPart2()
    {
        Assert.That(_day16.Part2(_inputFile), Is.EqualTo("45"));
    }
}