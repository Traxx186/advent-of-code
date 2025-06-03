using AdventOfCode.AoC2024.Solution;

namespace Test.AoC2024;

[TestFixture]
public class Day13Test
{
    private readonly Day13 _day13 = new();

    private readonly string _inputFile = Path.Combine("AdventOfCode.AoC2024", "Data", "day_13.txt");

    [Test]
    public void TestPart1()
    {
        Assert.That(_day13.Part1(_inputFile), Is.EqualTo("480"));
    }
    
    [Test]
    public void TestPart2()
    {
        Assert.That(_day13.Part2(_inputFile), Is.EqualTo("875318608908"));
    }
}