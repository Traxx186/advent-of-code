using AdventOfCode.AoC2025.Solution;

namespace Test.AoC2025;

[TestFixture]
public class Day02Test
{
    private readonly Day02 _day2 = new();

    private readonly string _inputFile = Path.Combine("AoC2025", "Data", "day_02.txt");

    [Test]
    public void TestPart1()
    {
        Assert.That(_day2.Part1(_inputFile), Is.EqualTo("1227775554"));
    }

    [Test]
    public void TestPart2()
    {
        Assert.That(_day2.Part2(_inputFile), Is.EqualTo("4174379265"));
    }
}