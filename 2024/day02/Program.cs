var lines = File.ReadAllLines(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "input.txt"));

bool safetyCheck(List<int> levels)
{
    var increase = levels[1] - levels[0] > 0;
    for (var i = 0; i < levels.Count - 1; i++)
    {
        var j = i + 1;
        if (i > 0 && levels[j] > levels[i] != increase) return false;

        if (levels[i] == levels[j] || Math.Abs(levels[i] - levels[j]) > 3)
        {
            return false;
        }
    }

    return true;
}

int part1(IEnumerable<string> lines)
{
    return lines
        .Select(l =>
        {
            var levels = l.Split(' ').Select(int.Parse).ToList();
            return safetyCheck(levels);
        }).Count(l => l);
}

int part2(IEnumerable<string> lines)
{
    return lines
        .Select(l =>
        {
            var levels = l.Split(' ').Select(int.Parse).ToList();
            if (safetyCheck(levels)) return true;

            for (int i = 0; i < levels.Count; i++)
            {
                var newList = new List<int>(levels);
                newList.RemoveAt(i);
                if (safetyCheck(newList)) return true;
            }

            return false;
        }).Count(l => l);
}


Console.WriteLine($"part1: {part1(lines)}");
Console.WriteLine($"part2: {part2(lines)}");
