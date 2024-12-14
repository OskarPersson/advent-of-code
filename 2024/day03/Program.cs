using System.Text.RegularExpressions;

var lines = File.ReadAllLines(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "input.txt"));

int part1(IEnumerable<string> lines)
{
    return Regex.Matches(string.Join("", lines.ToList()), @"mul\((\d{1,3}),(\d{1,3})\)")
        .Select(m => int.Parse(m.Groups[1].Value) * int.Parse(m.Groups[2].Value)).Sum();
}

int part2(IEnumerable<string> lines)
{
    var str = string.Join("", lines.ToList());
    return Regex.Matches(str, @"mul\((\d{1,3}),(\d{1,3})\)")
        .Select(m =>
        {
            var substr = str.Substring(0, m.Index);
            var conds = Regex.Matches(substr, @"(do\(\))|(don't\(\))");
            if (conds.Count > 0 && conds.Last().Value == "don't()")
                return 0;
            return int.Parse(m.Groups[1].Value) * int.Parse(m.Groups[2].Value);
        }).Sum();
}


Console.WriteLine($"part1: {part1(lines)}");
Console.WriteLine($"part2: {part2(lines)}");
