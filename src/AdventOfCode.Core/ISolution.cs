namespace AdventOfCode.Core;

public interface ISolution
{
    /// <summary>
    /// The display name of the assignment.
    /// </summary>
    string Name { get; }

    /// <summary>
    /// Executes the first part of the days assignment.
    /// </summary>
    /// <param name="inputFile">Path to the input file.</param>
    /// <returns>Result of the executed assignment.</returns>
    string Part1(string inputFile);

    /// <summary>
    /// Executes the first part of the days assignment.
    /// </summary>
    /// <param name="inputFile">Path to the input file.</param>
    /// <returns>Result of the executed assignment.</returns>
    string Part2(string inputFile);
}