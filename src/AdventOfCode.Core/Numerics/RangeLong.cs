using System.Collections;

namespace AdventOfCode.Core.Numerics;

/// <summary>
/// Represents a <see cref="Range"/>, but a <see cref="Int64"/> data type is used instead of a <see cref="Int32"/>.
/// </summary>
public readonly struct RangeLong : IEquatable<RangeLong>, IEnumerable<long>
{
    /// <summary>
    /// Represent the inclusive start index of the Range.
    /// </summary>
    public long Start { get; }
    
    /// <summary>
    /// Represent the inclusive start index of the Range.
    /// </summary>
    public long End { get; }
    
    /// <summary>Construct a Range object using the start and end indexes.</summary>
    /// <param name="start">Represent the inclusive start index of the range.</param>
    /// <param name="end">Represent the exclusive end index of the range.</param>
    public RangeLong(long start, long end)
    {
        Start = start;
        End = end;
    }
    
    /// <summary>
    /// Checks if the given value is contained with in the range.
    /// </summary>
    /// <param name="value">The value to check if it's within the range.</param>
    /// <returns>If the value is contained with in the range.</returns>
    public bool Contains(long value)
    {
        return value >= Start && value <= End;
    }

    /// <summary>
    /// Indicates whether the current Range object is equal to another Range object.
    /// </summary>
    /// <param name="other">An object to compare with this object</param>
    public bool Equals(RangeLong other)
    {
        return Start == other.Start && End == other.End;
    }

    public IEnumerator<long> GetEnumerator()
    {
        return new RangeLongEnumerator(this);
    }

    /// <summary>
    /// Indicates whether the current Range object is equal to another object of the same type.
    /// </summary>
    /// <param name="value">An object to compare with this object</param>
    public override bool Equals(object? value)
    {
        return value is RangeLong other && Equals(other);
    }

    /// <summary>
    /// Returns the hash code for this instance.
    /// </summary>
    public override int GetHashCode()
    {
        return HashCode.Combine(Start, End);
    }

    IEnumerator IEnumerable.GetEnumerator()
    {
        return GetEnumerator();
    }

    public static bool operator ==(RangeLong left, RangeLong right)
    {
        return left.Equals(right);
    }

    public static bool operator !=(RangeLong left, RangeLong right)
    {
        return !(left == right);
    }
}

public class RangeLongEnumerator : IEnumerator<long>
{
    private readonly RangeLong _rangeLong;
    private long _current;

    public RangeLongEnumerator(RangeLong rangeLong)
    {
        _rangeLong = rangeLong;
        _current = _rangeLong.Start - 1;
    }
    
    public bool MoveNext()
    {
        if  (_current >= _rangeLong.End)
            return false;

        _current++;
        return true;
    }

    public void Reset()
    {
        _current = _rangeLong.Start;
    }

    public void Dispose()
    {
        GC.SuppressFinalize(this);
    }

    long IEnumerator<long>.Current => _current;

    object? IEnumerator.Current => _current;
}