namespace AdventOfCode.Base;

public interface ISolution
{
    /// <summary>
    /// The display name of the assignment.
    /// </summary>
    string Name { get; }
    
    /// <summary>
    /// Executes the first part of the days assignment.
    /// </summary>
    /// <param name="inputFilePath">Path to the input file.</param>
    /// <returns>Result of the executed assignment.</returns>
    string Part1(string inputFilePath);
    
    /// <summary>
    /// Executes the first part of the days assignment.
    /// </summary>
    /// <param name="inputFilePath">Path to the input file.</param>
    /// <returns>Result of the executed assignment.</returns>
    string Part2(string inputFilePath);
}