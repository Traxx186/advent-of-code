using AdventOfCode.AoC2023.Solution;

namespace Test.AoC2023;

[TestFixture]
public class Day01Test
{
    private readonly Day01 _day1 = new();

    private readonly string _inputFile = Path.Combine("AoC2023", "Data", "day_01.txt");

    [Test]
    public void TestPart1()
    {
        Assert.That(_day1.Part1(_inputFile), Is.EqualTo("142"));
    }

    [Test]
    public void TestPart2()
    {
        //NOTE: different test data is required for this test
        Assert.That(_day1.Part2(_inputFile), Is.EqualTo("281"));
    }
}