using AdventOfCode.Core.Point;

namespace AdventOfCode.Core;

public readonly struct Matrix<T>
where T : IEquatable<T>
{
    /// <summary>
    /// The items inside the matrix.
    /// </summary>
    public List<List<T>> Tiles { get; }

    /// <summary>
    /// How many columns the matrix contains.
    /// </summary>
    public int Width { get; }

    /// <summary>
    /// How many rows the matrix contains.
    /// </summary>
    public int Height { get; }

    public Matrix(List<List<T>> tiles)
    {
        Tiles = tiles;
        Width = Tiles[0].Count;
        Height = Tiles.Count;
    }

    public Cell<T>? SearchForValue(T searchValue)
    {
        for (var y = 0; y < Height; y++)
        {
            for (var x = 0; x < Width; x++)
            {
                var value = Tiles[y][x];
                if (value.Equals(searchValue))
                    return new Cell<T>(value, new Point2D<int>(x, y));
            }
        }
        
        return null;
    }
    
    /// <summary>
    /// Tries to get a tile at the given location
    /// </summary>
    /// <param name="row">The row to search in.</param>
    /// <param name="column">The column to search in.</param>
    /// <param name="tile">The found tile.</param>
    /// <returns>If the tile has been found.</returns>
    public bool TryGetTile(int row, int column, out Cell<T> tile)
    {
        if ((uint)column >= Height || (uint)row >= Width)
        {
            tile = default!;
            return false;
        }
        
        tile = new Cell<T>(Tiles[row][column], new Point2D<int>(column, row));
        return true;
    }
}

public readonly struct Cell<T>(T value, Point2D<int> coordinates) : IEquatable<Cell<T>>
{
    /// <summary>
    /// The value of the cell.
    /// </summary>
    public T Value { get; } = value;
    
    /// <summary>
    /// The coordinates of the cell.
    /// </summary>
    public Point2D<int> Coordinates { get; } = coordinates;

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