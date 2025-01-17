using System.Numerics;

namespace AdventOfCode.Core;

public readonly struct Grid<T>
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

    public Grid(List<List<T>> tiles)
    {
        Tiles = tiles;
        Width = Tiles[0].Count;
        Height = Tiles.Count;
    }

    /// <summary>
    /// Searches in the matrix for any occuring item based on the search value.
    /// </summary>
    /// <param name="searchValue">item to search for.</param>
    /// <returns>List of occuring items</returns>
    public Cell<T>[] SearchForValues(T searchValue)
    {
        var values = new List<Cell<T>>();
        for (var y = 0; y < Height; y++)
        {
            for (var x = 0; x < Width; x++)
            {
                var value = Tiles[y][x];
                if (value.Equals(searchValue))
                    values.Add(new Cell<T>(value, new Vector2(x, y)));
            }
        }
        
        return values.ToArray();
    }
    
    /// <summary>
    /// Tries to get a tile at the given location
    /// </summary>
    /// <param name="row">The row to search in.</param>
    /// <param name="column">The column to search in.</param>
    /// <param name="tile">The found tile.</param>
    /// <returns>If the tile has been found.</returns>
    public bool TryGetTile(float row, float column, out Cell<T> tile)
    {
        if ((uint)row >= Height || (uint)column >= Width)
        {
            tile = default!;
            return false;
        }
        
        tile = new Cell<T>(Tiles[(int)row][(int)column], new Vector2(column, row));
        return true;
    }
}

public readonly struct Cell<T>(T value, Vector2 coordinates) : IEquatable<Cell<T>>
{
    /// <summary>
    /// The value of the cell.
    /// </summary>
    public T Value { get; } = value;
    
    /// <summary>
    /// The coordinates of the cell.
    /// </summary>
    public Vector2 Coordinates { get; } = coordinates;

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