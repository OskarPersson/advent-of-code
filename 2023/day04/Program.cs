var lines = File.ReadAllLines(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "input.txt")).ToList();

var part1 = lines.Select((l, i) =>
{
    var split = l.Split(" | ");
    var winners = split[0].Replace("  ", " ").Split(": ")[1].Split(" ").Select(x => int.Parse(x)).ToList();
    var mine = split[1].Replace("  ", " ").Split(" ").Where(x => !String.IsNullOrWhiteSpace(x)).Select(x => int.Parse(x)).ToList();

    var points = 0;
    foreach (var _ in mine.Where(n => winners.Contains(n)))
    {
        if (points == 0)
        {
            points++;
                
        }
        else
        {
            points *= 2;
        }
    }
    return points;
}).Sum();

var map = new Dictionary<int, int>();

int p2(List<string> lines)
{
    for (var i = 0; i < lines.Count; i++)
    {
        map[i] = 0;
    }

    for (int i = 0; i < lines.Count; i++)
    {
        var split = lines[i].Split(" | ");
        var winners = split[0].Replace("  ", " ").Split(": ")[1].Split(" ").Select(x => int.Parse(x)).ToList();
        var mine = split[1].Replace("  ", " ").Split(" ").Where(x => !String.IsNullOrWhiteSpace(x))
            .Select(x => int.Parse(x)).ToList();


        map[i] += 1;
        var ww = mine.Where(m => winners.Contains(m)).ToList();

        for (var k = 0; k < ww.Count(); k++)
        {
            map[i+k+1] += map[i];
        }
    }
    return map.Select(m => m.Value).Sum();
}


var part2 = p2(lines);

Console.WriteLine($"part1: {part1}");
Console.WriteLine($"part2: {part2}");
