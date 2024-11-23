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
    /// <returns>Result of the executed assignment.</returns>
    string Part1();
    
    /// <summary>
    /// Executes the first part of the days assignment.
    /// </summary>
    /// <returns>Result of the executed assignment.</returns>
    string Part2();
}