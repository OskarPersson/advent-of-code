var lines = File.ReadAllLines(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "input.txt")).Select(l => l.ToCharArray()).ToList();

var load = 0;
for (var r = 0; r < lines.Count; r++)
{
    for (var c = 0; c < lines[0].Length; c++)
    {
        var ch = lines[r][c];
        
        if (ch != 'O')
        {
            continue;
        }

        var rr = r;

        while (rr >= 0)
        {
            if (rr == 0)
            {
                break;
            }

            var chAbove = lines[rr-1][c];
            if (chAbove == '.')
            {
                lines[rr-1][c] = 'O';
                lines[rr][c] = '.';
                rr -= 1;
            }
            else
            {
                break;
            }
        }
        load += lines.Count - rr;
    }
}

Console.WriteLine($"part1: {load}");
