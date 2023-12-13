using System.Text.RegularExpressions;

var lines = File.ReadAllLines(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "input.txt")).ToList();

var sum = 0;
for (var lineidx = 0; lineidx < lines.Count; lineidx++)
{
    var localsum = 1;
    var line = lines[lineidx];

    var line1 = line.Split(" ")[0];
    var line2 = line.Split(" ")[1];

    var line1list = new List<string>();
    var line2list = new List<string>();
    for (var i = 0; i < 5; i++)
    {
        line1list.Add(line1);
        line2list.Add(line2);
    }
    Console.WriteLine(line);
    //line = $"{string.Join("?", line1list)} {string.Join(",", line2list)}";
    //Console.WriteLine(line);

    var records = line.Split(" ")[0].ToCharArray().ToList();
    var groups = line.Split(" ")[1].Split(",").Select(s => int.Parse(s)).ToList();

    var l = line1;
    
    var j = 0;
    var grpidx = 0;

    var pattern = @"^[^#.]*(#+)";
    var pattern2 = @"^[^#.]*(#+\?+)";

    while (groups.Count() > 0)
    {
        var match = Regex.Match(l, pattern);
        if (match.Groups.Count == 2 && match.Groups[1].Length == groups[0])
        {
            // if the first set of # matches the length of the next group, skip those # characters.
            // Also, skip the next coming character to leave a space for the next group
            l = l.Substring(match.Groups[1].Index + match.Groups[1].Length + 1);
            groups.RemoveAt(0);
            continue;
        }
        else
        {
            break;
        }

        /*
        var match2 = Regex.Match(l, pattern2);

        if (match2.Groups.Count == 2 && match2.Groups[1].Length == groups[grpidx])
        {
            l = l.Substring(match2.Groups[1].Index + match2.Groups[1].Length + 1);
            grpidx++;
            continue;
        }
        */
        

    }
    
    
    localsum += GetCombinations(groups, l);

    int GetCombinations(List<int> groups, string substring)
    {
        var offset = 0;
        var matchSuccess = false;

        if (groups.Count == 0)
        {
            return 0;
        }

        if (groups.Count == 1)
        {
            while (offset == 0 || matchSuccess)
            {
                var ll = substring.Substring(offset);
                offset++;
                
                
                var pattern3 = $"^([#,\\?]{{{groups[0]}}})";
                var match3 = Regex.Match(ll, pattern3);
                matchSuccess = match3.Success;
            }
            return Math.Max(offset - 1, 0);
        }

        var combos = 0;
        var subSum = 0;

        while (offset == 0 || combos > 1)
        {

            var ll = substring.Substring(offset);
            offset++;
            
            var pattern3 = $"^([#,\\?]{{{groups[0]}}})";
            var match3 = Regex.Match(ll, pattern3);
            if (match3.Success)
            {
                var g2 = groups.Skip(1).ToList();
                combos = GetCombinations(g2, ll.Substring(match3.Index + match3.Length + 1));
                subSum += combos;
            }
            else
            {
                break;
            }
        }

        return offset * subSum;
    }

    /*
    while (grpidx < groups.Count())
    {
        var grp= groups[grpidx];

        var record = records[j];
        while (record == '.')
        {
            j++;
            record = records[j];
        }

    }
    */
    
    
    Console.WriteLine();
    sum += localsum;
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
