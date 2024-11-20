using System.Numerics;

namespace AdventOfCode.Base.Point;

public readonly struct Point2D<T>(T x, T y) : IEquatable<Point2D<T>>
    where T : INumber<T>
{
    private T X { get; } = x;

    private T Y { get; } = y;

    public static Point2D<T> operator +(Point2D<T> lhs, Point2D<T> rhs) => 
        new(lhs.X + rhs.X, lhs.Y + rhs.Y);

    public static Point2D<T> operator -(Point2D<T> lhs, Point2D<T> rhs) => 
        new(lhs.X - rhs.X, lhs.Y - rhs.Y);

    public static Point2D<T> operator *(Point2D<T> lhs, Point2D<T> rhs) =>
        new(lhs.X * rhs.X, lhs.Y * rhs.Y);

    public static bool operator ==(Point2D<T> left, Point2D<T> right) =>
        left.Equals(right);

    public static bool operator !=(Point2D<T> left, Point2D<T> right) =>
        !(left == right);

    public bool Equals(Point2D<T> other) =>
        EqualityComparer<T>.Default.Equals(X, other.X) && EqualityComparer<T>.Default.Equals(Y, other.Y);
    
    public override int GetHashCode() =>
        X.GetHashCode() ^ Y.GetHashCode();

    public override bool Equals(object? obj)
    {
        if (obj is not Point2D<T> other)
            return false;

        return X == other.X && Y == other.Y;
    }
}