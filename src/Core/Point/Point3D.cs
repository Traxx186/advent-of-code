using System.Numerics;

namespace AdventOfCode.Core.Point;

public struct Point3D<T>(T x, T y, T z)
    where T : INumber<T>
{
    private T X { get; } = x;

    private T Y { get; } = y;

    private T Z { get; } = z;

    public static Point3D<T> operator +(Point3D<T> lhs, Point3D<T> rhs) => 
        new(lhs.X + rhs.X, lhs.Y + rhs.Y, lhs.Z + rhs.Z);
    
    public static Point3D<T> operator -(Point3D<T> lhs, Point3D<T> rhs) =>
        new(lhs.X - rhs.X, lhs.Y - rhs.Y, lhs.Z - rhs.Z);
    
    public static Point3D<T> operator *(Point3D<T> lhs, Point3D<T> rhs) => 
        new(lhs.X * rhs.X, lhs.Y * rhs.Y, lhs.Z * rhs.Z);
}