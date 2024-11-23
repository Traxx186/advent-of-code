namespace AdventOfCode.Base;

public class Calendar(int year, ISolution[] solutions)
{
    /// <summary>
    /// Runs the CLI for the given Advent Calendar year.
    /// </summary>
    public void Run()
    {
        Console.WriteLine($"\ud83c\udf84 Advent of Code {year} Solutions \ud83c\udf84");
        Console.WriteLine("       Justin van der Kruit       ");
        Console.WriteLine("Type 'exit' to stop the program \n");

        for (var i = 0; i < solutions.Length; i++)
            Console.WriteLine($"[{i + 1}] {solutions[i].Name}");

        while (true)
        {
            Console.Write("\nDay > ");
            var dayInput = Console.ReadLine();
            if (null == dayInput || dayInput.Equals("exit", StringComparison.CurrentCultureIgnoreCase))
                break;
            
            int day = 0, part = 0;
            try
            {
                day = int.Parse(dayInput);
                if (day > solutions.Length)
                {
                    Console.WriteLine($"[*] The program only goes to day {solutions.Length}");
                    continue;
                }
                
                Console.Write("Part (1 / 2) > ");
                var partInput = Console.ReadLine();
                part = int.Parse(partInput ?? string.Empty);
            }
            catch (Exception e)
            {
                Console.WriteLine("That is no number :/");
            }
            
            RunDay(day, part);
        }
    }

    /// <summary>
    /// Run the assignment part of the given day.
    /// </summary>
    /// <param name="day">The day to fetch.</param>
    /// <param name="part">The part to run.</param>
    private void RunDay(int day, int part)
    {
        var solution = solutions[day - 1];
        var output = part switch
        {
            1 => solution.Part1(),
            2 => solution.Part2(),
            _ => null
        };

        if (output == null)
        {
            Console.WriteLine("[-] Invalid part");
            return;
        }
        
        Console.WriteLine($"[+] OUT: {output}");
    }
    
    /// <summary>
    /// Reads the contents of the given input file.
    /// </summary>
    /// <param name="fileName">Name of file to read.</param>
    /// <returns>Contents of the file.</returns>
    public static string LoadInput(string fileName)
    {
        var currentDir = Environment.CurrentDirectory;
        var inputPath = Path.Combine(currentDir, "Data", fileName);
        
        return File.ReadAllText(inputPath);
    }
}