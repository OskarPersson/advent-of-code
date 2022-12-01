var lines = File.ReadAllLines(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "input.txt"));


int Part1(List<int> sums)
{
   return sums.Max();
}

int Part2(List<int> sums)
{
   sums.Sort();
   return sums.TakeLast(3).Sum();
}

var sums = new List<int>();
var sum = 0;

foreach (var l in lines)
{
   if (l == "")
   {
      sums.Add(sum);
      sum = 0;
      continue;
   }
   sum += int.Parse(l);
}

Console.WriteLine($"part1: {Part1(sums)}");
Console.WriteLine($"part2: {Part2(sums)}");
