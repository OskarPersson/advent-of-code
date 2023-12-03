using System.Text.RegularExpressions;

var lines = File.ReadAllLines(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "input.txt")).ToList();

var sum = 0;
var gears = new Dictionary<string, List<int>>();

lines.Select((l, i) =>
{
    var skips = 0;
    l.ToCharArray().Select((c, j) =>
    {
        if (skips > 0)
        {
            skips--;
            return c;
        }

        if (Char.IsDigit(c))
        {
            var number = Regex.Match(l.Substring(j), @"(\d+)").Groups[1].ToString();

            number.Select((_, k) =>
            {

                if (skips > 0)
                {
                    return ' ';
                }

                var adjSymbol = false;
                foreach (var rr in new[]{-1, 0, 1})
                {
                    foreach (var cc in new[]{-1, 0, 1})
                    {
                        if (0 <= i+rr && i+rr < lines.Count  && 0 <= j+k+cc && j+k+cc < l.Length) {
                            var ch = lines[i+rr][j+k+cc];

                            if (ch != '.' && !char.IsDigit(ch))
                            {
                                adjSymbol = true;
                            }

                            if (ch == '*') {
                                if (!gears.ContainsKey($"{i+rr}x{j+k+cc}")) {
                                    gears[$"{i+rr}x{j+k+cc}"] = new List<int>();
                                }
                                gears[$"{i+rr}x{j+k+cc}"].Add(int.Parse(number));
                            }
                        }
                    }
                }

                if (adjSymbol)
                {
                    sum += int.Parse(number);
                    skips = number.Length;
                }

                return ' ';
            }).ToList();
        }

        return c;
    }).ToList();
    return l;
}).ToList();

Console.WriteLine($"part1: {sum}");
Console.WriteLine($"part2: {gears.Where(g => g.Value.Count == 2).Select(g => g.Value.Aggregate(1, (acc, val) => acc * val)).Sum()}");
