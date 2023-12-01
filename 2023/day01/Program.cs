var lines = File.ReadAllLines(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "input.txt"));

var numbers = new Dictionary<String, int>()
{
    {"one", 1},
    {"two", 2},
    {"three", 3},
    {"four", 4},
    {"five", 5},
    {"six", 6},
    {"seven", 7},
    {"eight", 8},
    {"nine", 9},
};

int AddLastAndFirstDigit(IEnumerable<string> lines)
{
    return lines.Select(l =>
    {

        var c1 = l.ToCharArray().First(char.IsDigit);
        var c2 = l.ToCharArray().Last(char.IsDigit);
        return int.Parse($"{c1}{c2}");
    }).Sum();
}


int part1(IEnumerable<string> lines)
{
    return AddLastAndFirstDigit(lines);
}

int part2(IEnumerable<string> lines)
{
    var solution = lines.Where(l => !string.IsNullOrWhiteSpace(l)).Select(
        l =>
        {
            try
            {
                var subindexFirst = numbers.Select(k => l.IndexOf(k.Key)).Where(i => i >= 0).Min();
                var subindexFirstVal = numbers.First(k => l.Substring(subindexFirst).StartsWith(k.Key)).Key;

                l = l.Substring(0, subindexFirst) + numbers[subindexFirstVal] + l.Substring(subindexFirst + 1);
            }
            catch (Exception)
            {
                // No number found, and therefore i is never larger than 0. We get here because Min doesn't like empty lists.
            }

            var subindexLast = numbers.Select(k => l.LastIndexOf(k.Key)).Max();
            if (subindexLast >= 0)
            {
                var subindexLastVal = numbers.Last(k => l.Substring(subindexLast).StartsWith(k.Key)).Key;

                l = l.Substring(0, subindexLast) + numbers[subindexLastVal] + l.Substring(subindexLast + 1);
            }

            return l;
        }
    );
    return AddLastAndFirstDigit(solution);
}


Console.WriteLine($"part1: {part1(lines)}");
Console.WriteLine($"part2: {part2(lines)}");
