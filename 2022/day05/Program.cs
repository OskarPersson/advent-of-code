using System.Text.RegularExpressions;

var lines = File.ReadAllLines(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "input.txt"));
const string pattern = @"move (\d+) from (\d) to (\d)";


(List<List<char>>, int) Setup(IEnumerable<string> lines)
{
   var stacks = new List<List<char>>();
   var stackLine = 0;
   foreach (var line in lines)
   {
      stackLine++;
      var isNumeric = int.TryParse(line.Replace(" ", ""), out _);
      if (isNumeric)
      {
         for (int i = 0; i < line.Replace(" ", "").Length; i++)
         {
            stacks.Add(new List<char>());
         }
         break;
      }
   }

   foreach (var line in lines)
   {
      var isNumeric = int.TryParse(line.Replace(" ", ""), out _);
      if (isNumeric) break;

      var chunks = line.Chunk(4).Select(c => new string(c.Take(3).ToArray())).ToList();

      var idx = 0;
      foreach (var chunk in chunks)
      {
         if (!string.IsNullOrWhiteSpace(chunk))
         {
            stacks[idx].Add(chunk[1]);
         }
         idx += 1;
      }
   }
   return (stacks, stackLine);
}

string Part1(List<List<char>> stacks, List<string> lines)
{
   foreach (var line in lines)
   {
      var match = Regex.Match(line, pattern);
      var num = int.Parse(match.Groups[1].ToString());
      var from = int.Parse(match.Groups[2].ToString());
      var to = int.Parse(match.Groups[3].ToString());

      for (var i = 0; i < num; i++)
      {
         stacks[to - 1].Insert(0, stacks[from - 1][0]);
         stacks[from - 1].RemoveAt(0);
      }
   }

   return new string(stacks.Where(s => s.Any()).Select(s => s[0]).ToArray());
}

string Part2(List<List<char>> stacks, List<string> lines)
{
   foreach (var line in lines)
   {
      var match = Regex.Match(line, pattern);
      var num = int.Parse(match.Groups[1].ToString());
      var from = int.Parse(match.Groups[2].ToString());
      var to = int.Parse(match.Groups[3].ToString());

      for (int i = num; i > 0; i--)
      {
         stacks[to - 1].Insert(0, stacks[from - 1][i-1]);
         stacks[from - 1].RemoveAt(i-1);
      }
   }

   var res = new List<char>();
   
   foreach (var s in stacks)
   {
      if (s.Any()) res.Add(s[0]);
   }
   return new string(stacks.Where(s => s.Any()).Select(s => s[0]).ToArray());
}

var (stacks, stackLine) = Setup(lines);
var (stacks2, _) = Setup(lines);

Console.WriteLine($"part1: {Part1(stacks, lines.Skip(stackLine+1).ToList())}");
Console.WriteLine($"part2: {Part2(stacks2, lines.Skip(stackLine+1).ToList())}");
