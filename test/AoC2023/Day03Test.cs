using AdventOfCode.AoC2023.Solution;

namespace Test.AoC2023;

[TestFixture]
public class Day03Test
{
    private readonly Day03 _day3 = new();

    private readonly string _inputFile = Path.Combine("AoC2023", "Data", "day_03.txt");

    [Test]
    public void TestPart1()
    {
        Assert.That(_day3.Part1(_inputFile), Is.EqualTo("4361"));
    }

    [Test]
    public void TestPart2()
    {
        Assert.That(_day3.Part2(_inputFile), Is.EqualTo("467835"));
    }
}