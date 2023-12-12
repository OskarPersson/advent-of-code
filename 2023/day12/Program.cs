using System.Text.RegularExpressions;

var lines = File.ReadAllLines(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "input.txt")).ToList();

var sum = 0;
for (var lineidx = 0; lineidx < lines.Count; lineidx++)
{
    var line = lines[lineidx];
    var records = line.Split(" ")[0].ToCharArray().ToList();
    var groups = line.Split(" ")[1].Split(",").Select(s => int.Parse(s)).ToList();


    var pattern = @"(#+)";
    var matches = Regex.Matches(line, pattern);
    foreach (Match match in matches)
    {
        for (int i = 0; i < groups.Count; i++)
        {
            if (groups[i] == match.Length)
            {
                groups.RemoveAt(i);
                line = $"{line.Substring(0, match.Index)}{new string('.', match.Length)}{line.Substring(match.Index + match.Length)}";
                break;
            }
        }
    }


    var arrs = 1;

    var n = 0;
    for (int ci = 0; ci < records.Count; ci++)
    {
        var c = records[ci];
        if (c is '.' or '#')
        {
            if (n == 0)
            {
                if (c == '#')
                {
                    if (groups.Count > 0)
                    {
                        groups[0] = groups[0] - 1;
                        if (groups[0] == 0)
                        {
                            groups.RemoveAt(0);
                        }
                    }
                }

                continue;
            }

            var subgrp = new List<int>();

            while (subgrp.Sum() < n - 1)
            {
                if (c == '.')
                {
                    if (subgrp.Sum() + groups[0] <= n - 1)
                    {
                        subgrp.Add(groups[0]);
                        groups.RemoveAt(0);
                    }
                    else
                    {
                        break;
                    }
                }
                else if (c == '#')
                {
                    subgrp.Add(1);
                }
            }

            if (subgrp.Count > 0)
            {
                arrs *= n / subgrp.Sum();
            }
            else
            {
                arrs *= 1;
            }

            n = 0;
            continue;
        }

        if (c == '?')
        {
            n++;
        }
    }

    Console.WriteLine($"{line} => {arrs}");
    sum += arrs;
}


Console.WriteLine();
Console.WriteLine(sum);
