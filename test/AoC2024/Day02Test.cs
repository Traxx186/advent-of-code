using AdventOfCode.AoC2024.Solution;

namespace Test.AoC2024;

[TestFixture]
public class Day02Test
{
    private readonly Day02 _day2 = new();
    
    private readonly string _inputFile = Path.Combine("AoC2024", "Data", "day_02.txt");

    [Test]
    public void TestPart1()
    {
        Assert.That(_day2.Part1(_inputFile), Is.EqualTo("2"));
    }

    [Test]
    public void TestPart2()
    {
        Assert.That(_day2.Part2(_inputFile), Is.EqualTo("4"));
    }
}
