var lines = File.ReadAllLines(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "input.txt"));

int part1(IEnumerable<string> lines)
{
    var firstList = lines.Select(l => int.Parse(l.Split(" ", 2)[0])).Order();
    var secondList = lines.Select(l => int.Parse(l.Split(" ").Reverse().ToList()[0])).Order();

    return firstList.Zip(secondList, (f, s) => Math.Abs(f-s)).Sum();
}

int part2(IEnumerable<string> lines)
{
    var firstList = lines.Select(l => int.Parse(l.Split(" ", 2)[0])).Order();
    var secondList = lines.Select(l => int.Parse(l.Split(" ").Reverse().ToList()[0])).Order();

    return firstList.Select(f =>
    {
        return f * secondList.Count(s => s == f);
    }).Sum();
}


Console.WriteLine($"part1: {part1(lines)}");
Console.WriteLine($"part2: {part2(lines)}");
