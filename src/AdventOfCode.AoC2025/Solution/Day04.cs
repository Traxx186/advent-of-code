using AdventOfCode.Core;

namespace AdventOfCode.AoC2025.Solution;

public class Day04 : ISolution
{
    private const char PaperRoll = '@';

    public string Name => "Printing Department";

    public string Part1(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var warehouse = ParseInput(input);
        var totalRollsMoved = MoveRolls(warehouse);
        
        return totalRollsMoved.ToString();
    }

    public string Part2(string inputFile)
    {
        var input = Calendar.LoadInput(inputFile);
        var warehouse = ParseInput(input);
        var totalRollsMoved = 0;
        var removed = 0;

        do
        {
            removed = MoveRolls(warehouse);
            totalRollsMoved += removed;
        } while (removed > 0);

        return totalRollsMoved.ToString();
    }

    private static Grid<char> ParseInput(string input)
    {
        var tiles = input.Split(Environment.NewLine, StringSplitOptions.RemoveEmptyEntries)
            .Select(line => line.ToCharArray().ToList())
            .ToList();

        return new Grid<char>(tiles);
    }

    private static int MoveRolls(Grid<char> warehouse)
    {
        var removedRolls = new List<(int, int)>();
        var rolls = 0;
        
        for (var i = 0; i < warehouse.Tiles.Count; i++)
        {
            var row = warehouse.Tiles[i];

            for (var j = 0; j < row.Count; j++)
            {
                var item = row[j];
                if (item != PaperRoll)
                    continue;

                if (!MovableRoll(warehouse, i, j))
                    continue;

                removedRolls.Add((i, j));
                rolls++;
            }
        }
        
        foreach (var removed in removedRolls)
            warehouse.Tiles[removed.Item1][removed.Item2] = '.';
        
        return rolls;
    }
    
    private static bool MovableRoll(Grid<char> grid, int row, int col)
    {
        var surroundings = new List<char>();
        var searchRows = new[] { -1, 0, 1 };
        var searchCols = new[] { -1, 0, 1 };

        foreach (var searchRow in searchRows)
        {
            var rowToSelect =  row + searchRow;

            // Check is row search is out of bounds, if true skip search on current row
            if (rowToSelect < 0 || rowToSelect > grid.Height - 1)
                continue;
            
            foreach (var searchCol in searchCols)
            {
               var colToSelect = col + searchCol;
                
                // Check is col search is out of bounds, if true skip search on current col
                if (colToSelect < 0 || colToSelect > grid.Width - 1)
                    continue;
                
                // Do not add source item
                if (rowToSelect == row && colToSelect == col)
                    continue;
                
                surroundings.Add(grid.Tiles[rowToSelect][colToSelect]);
            }
        }

        return surroundings.Count(t => t == PaperRoll) < 4;
    }
}