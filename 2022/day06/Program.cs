var lines = File.ReadAllLines(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "input.txt")).ToList();

int FindMarker(string line, int n)
{
   for (int i = n; i < line.Length-1; i++)
   {
      var chunk = line.Take(new Range(i - n, i));
      var dup = false;
      foreach (var c in chunk.ToList())
      {
         dup = (chunk.Count(cc => cc==c) > 1);
         if (dup) break;
      }
      if (!dup) return i;
   }
   return 0;
}

int Part1(string line)
{
   return FindMarker(line, 4);
}


int Part2(string line)
{
   return FindMarker(line, 14);
}
Console.WriteLine($"part1: {Part1(lines[0])}");
Console.WriteLine($"part2: {Part2(lines[0])}");
