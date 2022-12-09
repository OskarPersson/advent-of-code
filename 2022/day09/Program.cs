using System.Text.RegularExpressions;

var lines = File.ReadAllLines(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "input.txt")).ToList();
var pattern = @"([A-Z]) (\d+)";

((int, int), (int, int)) MoveIt((int, int) k1, (int, int) k2)
{
   if (k2.Item1 < k1.Item1 - 1)
   {
      k2.Item1 += 1;
      if (k2.Item2 == k1.Item2) return (k1, k2);
      
      if (k2.Item2 < k1.Item2) k2.Item2 += 1;
      else k2.Item2 -= 1;
   }
   else if (k2.Item1 > k1.Item1 + 1)
   {
      k2.Item1 -= 1;
      if (k2.Item2 == k1.Item2) return (k1, k2);
      
      if (k2.Item2 < k1.Item2) k2.Item2 += 1;
      else k2.Item2 -= 1;
   }
   else if (k2.Item2 < k1.Item2 - 1)
   {
      k2.Item2 += 1;
      if (k2.Item1 == k1.Item1) return (k1, k2);
      
      if (k2.Item1 < k1.Item1) k2.Item1 += 1;
      else k2.Item1 -= 1;
   }
   else if (k2.Item2 > k1.Item2 + 1)
   {
      k2.Item2 -= 1;
      if (k2.Item1 == k1.Item1) return (k1, k2);
      
      if (k2.Item1 < k1.Item1) k2.Item1 += 1;
      else k2.Item1 -= 1;
   }
      
   return (k1, k2);
}

int Part1(List<string> lines)
{
   var H = (0, 0);
   var T = (0, 0);

   var tPos = new List<(int, int)> { (0, 0) };

   foreach (var line in lines)
   {
      var match = Regex.Match(line, pattern);
      var dir = match.Groups[1].ToString().ToCharArray()[0];
      var steps = int.Parse(match.Groups[2].ToString());
      
      for (int i = 0; i < steps; i++)
      {
         switch (dir)
         {
            case 'U':
               H.Item2 += 1;
               break;
            case 'R':
               H.Item1 += 1;
               break;
            case 'D':
               H.Item2 -= 1;
               break;
            case 'L':
               H.Item1 -= 1;
               break;
         }
         
         (H, T) = MoveIt(H, T);

         if (!tPos.Contains(T)) tPos.Add(T);
      }
      
   }

   return tPos.Count();
}

int Part2(List<string> lines)
{
   var H = (0, 0);
   var T = new List<(int, int)>
   {
      (0, 0),
      (0, 0),
      (0, 0),
      (0, 0),
      (0, 0),
      (0, 0),
      (0, 0),
      (0, 0),
      (0, 0),
   };

   var tPos = new List<(int, int)> { (0, 0) };

   foreach (var line in lines)
   {
      var match = Regex.Match(line, pattern);
      var dir = match.Groups[1].ToString().ToCharArray()[0];
      var steps = int.Parse(match.Groups[2].ToString());
      
      for (int i = 0; i < steps; i++)
      {
         switch (dir)
         {
            case 'U':
               H.Item2 += 1;
               break;
            case 'R':
               H.Item1 += 1;
               break;
            case 'D':
               H.Item2 -= 1;
               break;
            case 'L':
               H.Item1 -= 1;
               break;
         }

         for (int j = 0; j < 9; j++)
         {
            var k1 = H;
            if (j > 0) k1 = T[j - 1];
            var k2 = T[j];

            (k1, k2) = MoveIt(k1, k2);
            
            if (j > 0) T[j - 1] = k1;
            T[j] = k2;
         
            if (!tPos.Contains(T[8])) tPos.Add(T[8]);
         }
      }
   }

   return tPos.Count();
}

Console.WriteLine($"part1: {Part1(lines)}");
Console.WriteLine($"part2: {Part2(lines)}");
