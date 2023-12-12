using System.Text.RegularExpressions;

var lines = File.ReadAllLines(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "input.txt")).ToList();

var sum = 0;
for (var lineidx = 0; lineidx < lines.Count; lineidx++)
{
    var localsum = 0;
    var line = lines[lineidx];

    var line1 = line.Split(" ")[0];
    var line2 = line.Split(" ")[1];

    var line1list = new List<string>();
    for (var i = 0; i < 5; i++)
    {
        line1list.Add(line1);
    }
    line = $"{string.Join("?", line1list)} {line2}";

    var records = line.Split(" ")[0].ToCharArray().ToList();
    var groups = line.Split(" ")[1].Split(",").Select(s => int.Parse(s)).ToList();

    var pattern = "^\\.*";

    foreach (var grp in groups)
    {
        pattern += $"#{{{grp}}}\\.+";
    }

    pattern += "$";

    pattern = pattern.Replace("+$", "*$");


    var vars = GetVariations(line.Split(" ")[0]);

    foreach (var v in vars)
    {
        if (Regex.IsMatch(v, pattern))
        {
            localsum++;
            sum++;
        }
    }
}


Console.WriteLine();
Console.WriteLine(sum);


List<string> GetVariations(string input)
{
    // Get all variations of the input string where each ? is replaced with either . or #

    var variations = new List<string>();
    var questionMarks = input.Count(c => c == '?');
    var max = Math.Pow(2, questionMarks);
    for (var i = 0; i < max; i++)
    {
        var binary = Convert.ToString(i, 2).PadLeft(questionMarks, '0');
        var variation = input;
        foreach (var bit in binary)
        {
            variation = ReplaceFirst(variation, "?", bit == '0' ? "." : "#");
        }
        variations.Add(variation);
    }
    return variations;

}

string ReplaceFirst(string text, string search, string replace)
{
    int pos = text.IndexOf(search);
    if (pos < 0)
    {
        return text;
    }
    return text.Substring(0, pos) + replace + text.Substring(pos + search.Length);
}
