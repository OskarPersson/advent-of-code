var lines = File.ReadAllLines(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "input.txt")).ToList();

Console.WriteLine($"part1: {Solve(2)}");
Console.WriteLine($"part2: {Solve(1000000)}");

long Solve(int n)
{

    var galaxies = new List<Pos>();

    for (int i = 0; i < lines.Count; i++)
    {
        var line = lines[i];
        for (int j = 0; j < line.Length; j++)
        {
            if (line[j] == '#')
            {
                galaxies.Add(new Pos { i = i, j = j });
            }
        }
    }

    var rows = (long)lines.Count;
    for (var i = 0; i < rows; i++)
    {
        var hasGalaxy = galaxies.Any(g => g.i == i);
        if (!hasGalaxy)
        {
            galaxies = galaxies.Select(g => g.i > i ? new Pos { i = g.i + n - 1, j = g.j } : g).ToList();
            rows += n - 1;
            i += n - 1;
        }
    }

    var cols = (long)lines[0].Length;

    for (int j = 0; j < cols; j++)
    {
        var hasGalaxy = galaxies.Any(g => g.j == j);

        if (!hasGalaxy)
        {
            galaxies = galaxies.Select(g => g.j > j ? new Pos { i = g.i, j = g.j + n - 1 } : g).ToList();
            cols += n - 1;
            j += n - 1;
        }
    }

    var pairs = galaxies.SelectMany((x, i) => galaxies.Skip(i + 1), (x, y) => Tuple.Create(x, y));

    var distances = new List<long>();

    foreach (var pair in pairs)
    {
        var distance = Math.Abs(pair.Item1.i - pair.Item2.i) + Math.Abs(pair.Item1.j - pair.Item2.j);
        distances.Add(distance);
    }

    return distances.Sum();
}

struct Pos
{
    public long i { get; set; }
    public long j { get; set; }
}
