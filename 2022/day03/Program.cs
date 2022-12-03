var lines = File.ReadAllLines(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "input.txt"));

int Priority(char c)
{
   int i = c;
   if (char.IsLower(c)) return i - 96;
   return i - 38;
}

char Duplicate2(char[] first, char[] second)
{
   foreach (var f in first)
   {
      if (second.Contains(f)) return f;
   }
   throw new NotImplementedException($"{first} not in {second}");
}

char Duplicate3(char[] first, char[] second, char[] third)
{
   foreach (var f in first)
   {
      if (second.Contains(f) && third.Contains(f)) return f;
   }
   throw new NotImplementedException($"{first} not in {second} && not in {third}");
}

int Part1(IEnumerable<string> lines)
{
   var sums = new List<int>();
   foreach (var line in lines)
   {
      var chars = line.ToCharArray();
      var first = chars.Take(chars.Length / 2).ToArray();
      var second = chars.TakeLast(chars.Length / 2).ToArray();
      var dup = Duplicate2(first, second);
      sums.Add(Priority(dup));
   }
   return sums.Sum();
}



int Part2(IEnumerable<string> lines)
{
   var sums = new List<int>();
   var grps = lines
      .Select((x, i) => new {Index = i, Value = x})
      .GroupBy(x => x.Index / 3)
      .Select(x => x.Select(v => v.Value).ToList())
      .ToList();;
   foreach (var grp in grps)
   {

      var grp1 = grp[0].ToCharArray();
      var grp2 = grp[1].ToCharArray();
      var grp3 = grp[2].ToCharArray();

      var dup = Duplicate3(grp1, grp2, grp3);
      sums.Add(Priority(dup));
   }
   return sums.Sum();
}

Console.WriteLine($"part1: {Part1(lines)}");
Console.WriteLine($"part2: {Part2(lines)}");
