using AdventOfCode.Core.Point;

namespace AdventOfCode.Core;

public readonly struct Matrix<T>
{
    /// <summary>
    /// The items inside the matrix.
    /// </summary>
    private List<List<T>> Tiles { get; }
    
    /// <summary>
    /// How many columns the matrix contains.
    /// </summary>
    private int Width { get; }

    /// <summary>
    /// How many rows the matrix contains.
    /// </summary>
    private int Height { get; }

    public Matrix(List<List<T>> tiles)
    {
        Tiles = tiles;
        Width = Tiles[0].Count;
        Height = Tiles.Count;
    }

    /// <summary>
    /// Tries to get a tile at the given location
    /// </summary>
    /// <param name="row">The row to search in.</param>
    /// <param name="column">The column to search in.</param>
    /// <param name="tile">The found tile.</param>
    /// <returns>If the tile has been found.</returns>
    public bool TryGetTile(int row, int column, out T tile)
    {
        try
        {
            tile = Tiles[row][column];
            return true;
        }
        catch (IndexOutOfRangeException e)
        {
            tile = default!;
            return false;   
        }
    }
}

public readonly struct Cell<T>(T value, Point2D<uint> coordinates) : IEquatable<Cell<T>>
{
    /// <summary>
    /// The value of the cell.
    /// </summary>
    private T Value { get; } = value;
    
    /// <summary>
    /// The coordinates of the cell.
    /// </summary>
    private Point2D<uint> Coordinates { get; } = coordinates;

    public override int GetHashCode() =>
        Value.GetHashCode() ^ Coordinates.GetHashCode();

    public override bool Equals(object obj)
    {
        if (obj is not Cell<T> cell)
            return false;
        
        return cell.Value.Equals(Value)
               && cell.Coordinates == Coordinates;
    }
    
    public bool Equals(Cell<T> other) =>
        EqualityComparer<T>.Default.Equals(Value, other.Value) && Coordinates.Equals(other.Coordinates);

    public static bool operator ==(Cell<T> left, Cell<T> right) =>
        left.Equals(right);

    public static bool operator !=(Cell<T> left, Cell<T> right) =>
        !(left == right);
}