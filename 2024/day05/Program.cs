using System.Formats.Asn1;

var lines = File.ReadAllLines(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "input.txt"));

int part1(IEnumerable<string> lines)
{
    var rules = lines.TakeWhile(l => !string.IsNullOrWhiteSpace(l))
        .Select(r => r
            .Split("|")
            .Select(int.Parse)
            .ToList())
        .ToList();

    var updates = lines.SkipWhile(l => l.Contains('|') || string.IsNullOrWhiteSpace(l))
        .Select(u => u
            .Split(',')
            .Select(int.Parse)
            .ToList())
        .ToList();

    var validMiddles = 0;

    foreach (var update in updates)
    {
        var valid = true;
        for (int i = 0; i < update.Count - 1; i++)
        {
            var a = update[i];
            var b = update[i + 1];

            if (rules.Any(r => r[0] == b && r[1] == a))
            {
                valid = false;
                break;
            }
        }

        if (valid)
        {
            validMiddles += update[update.Count / 2];
        }
    }

    return validMiddles;
}

List<int> fix(List<int> update, List<List<int>> rules)
{
    var valid = true;
    var newUpdate = new List<int>();
    for (int i = 0; i < update.Count - 1; i++)
    {
        var a = update[i];
        var b = update[i + 1];

        if (rules.Any(r => r[0] == b && r[1] == a))
        {
            if (i > 0)
            {
                newUpdate.AddRange(update.Take(i));
            }
            newUpdate.Add(b);
            newUpdate.Add(a);
            newUpdate.AddRange(update.Skip(i+2));
            valid = false;
            break;
        }
    }


    if (!valid)
    {
        return fix(newUpdate, rules);
    }

    return update;
}

int part2(IEnumerable<string> lines)
{
    var rules = lines.TakeWhile(l => !string.IsNullOrWhiteSpace(l))
        .Select(r => r
            .Split("|")
            .Select(int.Parse)
            .ToList())
        .ToList();

    var updates = lines.SkipWhile(l => l.Contains('|') || string.IsNullOrWhiteSpace(l))
        .Select(u => u
            .Split(',')
            .Select(int.Parse)
            .ToList())
        .ToList();

    var validMiddles = 0;

    foreach (var update in updates)
    {
        var valid = true;
        for (int i = 0; i < update.Count - 1; i++)
        {
            var a = update[i];
            var b = update[i + 1];

            if (rules.Any(r => r[0] == b && r[1] == a))
            {
                valid = false;
                break;
            }
        }

        if (valid)
        {
            continue;
        }

        var fixedUpdate = fix(update, rules);
        validMiddles += fixedUpdate[fixedUpdate.Count / 2];
    }

    return validMiddles;
}


Console.WriteLine($"part1: {part1(lines)}");
Console.WriteLine($"part2: {part2(lines)}");
