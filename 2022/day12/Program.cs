
int Elevation(char c)
{
   return c switch
   {
      'S' => 'a',
      'E' => 'z',
      _ => c
   };
}


List<(int, int)> Neighbours(int x, int y, List<string> ls)
{
   var width = ls[0].Length;
   var height = ls.Count;
   var u = (x, y);
   return new List<(int, int)>()
   {
      (u.Item1+1, u.Item2),
      (u.Item1, u.Item2+1),
      (u.Item1-1, u.Item2),
      (u.Item1, u.Item2-1),
   }
      .Where(xx => xx.Item1 >= 0 && xx.Item1 < width && xx.Item2 >= 0 && xx.Item2 < height)
      .Where(xx => Elevation(ls[xx.Item2][xx.Item1]) <= Elevation(ls[u.Item2][u.Item1]) + 1).ToList();
}


int ShortestPath((int, int) start, (int, int) end, List<string> ls)
{
   var dists = new Dictionary<(int, int), int>();
   var queue = new PriorityQueue<(int, int), int>();
   
   dists[start] = 0;
   queue.Enqueue(start, 0);

   while (queue.Count > 0)
   {
      var u = queue.Dequeue();
      var neighbours = Neighbours(u.Item1, u.Item2, ls);
      foreach (var v in neighbours)
      {
         if (!dists.ContainsKey(v))
         {
            var alt = dists[u] + 1;
            dists[v] = alt;
            queue.Enqueue(v, alt);
         }
      }
   }

   return !dists.ContainsKey(end) ? int.MaxValue : dists[end];
}

int Part1(List<string> ls)
{
   var start = (0,0);
   var end = (0,0);

   for (var y = 0; y < ls.Count; y++)
   {
      var line = ls[y];
      for (var x = 0; x < line.Length; x++)
      {
         if (line[x] == 'S')
         {
            start = (x, y);
         }
         else if (line[x] == 'E')
         {
            end = (x, y);
         }
      }
   }

   return ShortestPath(start, end, ls.ToList());
}

int Part2(List<string> ls)
{
   var starts = new List<(int, int)>();
   var end = (0,0);

   for (var y = 0; y < ls.Count; y++)
   {
      var line = ls[y];
      for (var x = 0; x < line.Length; x++)
      {
         if (line[x] == 'S' || line[x] == 'a')
         {
            starts.Add((x, y));
         }
         else if (line[x] == 'E')
         {
            end = (x, y);
         }
      }
   }

   ls = ls.Select(l => l.Replace('S', 'a')).ToList();
   return starts.Select((start) => ShortestPath(start, end, ls)).Min();
}

var lines = File.ReadAllLines(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "input.txt")).ToList();
Console.WriteLine($"part1: {Part1(lines.ToList())}");
Console.WriteLine($"part2: {Part2(lines.ToList())}");
