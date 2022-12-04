using System.Text.RegularExpressions;

var lines = File.ReadAllLines(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "input.txt"));

int Part1(IEnumerable<string> lines)
{
   var sum = 0;

   var pattern = @"(\d+)-(\d+),(\d+)-(\d+)";
   foreach (var line in lines)
   {
      var match = Regex.Match(line, pattern);
      var x1 = int.Parse(match.Groups[1].ToString());
      var x2 = int.Parse(match.Groups[2].ToString());
      var y1 = int.Parse(match.Groups[3].ToString());
      var y2 = int.Parse(match.Groups[4].ToString());
      

      if (x1 <= y1 && y2 <= x2)
      {
         sum += 1;
      }

      else if (y1 <= x1 && x2 <= y2)
      {
         sum += 1;
      }

   }


   return sum;
}


int Part2(IEnumerable<string> lines)
{
   var sum = 0;

   var pattern = @"(\d+)-(\d+),(\d+)-(\d+)";
   foreach (var line in lines)
   {
      var match = Regex.Match(line, pattern);
      var x1 = int.Parse(match.Groups[1].ToString());
      var x2 = int.Parse(match.Groups[2].ToString());
      var y1 = int.Parse(match.Groups[3].ToString());
      var y2 = int.Parse(match.Groups[4].ToString());
      

      if (x1 <= y1 && y1 <= x2 || x1 <= y2 && y2 <= x2 )
      {
         sum += 1;
      }
      else if (y1 <= x1 && x1 <= y2 || y1 <= x2 && x2 <= y2 )
      {
         sum += 1;
      }

   }

   return sum;
}

Console.WriteLine($"part1: {Part1(lines)}");
Console.WriteLine($"part2: {Part2(lines)}");
