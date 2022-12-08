var lines = File.ReadAllLines(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "input.txt")).ToList();

List<int> LeftNeighbours(List<string> l, int row, int col)
{
   var left = new List<int>();
   for (int x = 0; x <= col-1; x++) left.Add(l[row][x] - '0');
   return left;
}

List<int> RightNeighbours(List<string> l, int row, int col)
{
   var right = new List<int>();
   for (int x = col+1; x < l[0].Length; x++) right.Add(l[row][x] - '0');
   return right;
}
List<int> UpNeighbours(List<string> l, int row, int col)
{
   var up = new List<int>();
   for (int y = 0; y <= row-1; y++) up.Add(l[y][col] - '0');
   return up;
}

List<int> DownNeighbours(List<string> l, int row, int col)
{
   var down = new List<int>();
   for (int y = row+1; y < l.Count; y++) down.Add(l[y][col] - '0');
   return down;
}

int CheckNeighbours(List<string> l, int row, int col)
{
   int startHeight = l[row][col] - '0';
   int visible = 0;
   
   // left
   var left = LeftNeighbours(l, row, col);
   if (left.Max() <= startHeight-1) visible += 1;
   
   // right
   var right = RightNeighbours(l, row, col);
   if (right.Max() <= startHeight-1) visible += 1;
   
   // up
   var up = UpNeighbours(l, row, col);
   if (up.Max() <= startHeight-1) visible += 1;
   
   // down
   var down = DownNeighbours(l, row, col);
   if (down.Max() <= startHeight-1) visible += 1;

   return int.Min(visible, 1);
}

int CheckNeighboursScore(List<string> l, int row, int col)
{
   int startHeight = l[row][col] - '0';
   
   // left
   var left = LeftNeighbours(l, row, col);
   left.Reverse();
   var leftScore = left.TakeWhile(lft => lft < startHeight).Count();
   if (left.Max() >= startHeight) leftScore += 1;

   // right
   var right = RightNeighbours(l, row, col);
   var rightScore = right.TakeWhile(lft => lft < startHeight).Count();
   if (right.Max() >= startHeight) rightScore += 1;
   
   // up
   var up = UpNeighbours(l, row, col);
   up.Reverse();
   var upScore= up.TakeWhile(lft => lft < startHeight).Count();
   if (up.Max() >= startHeight) upScore += 1;
   
   // down
   var down = DownNeighbours(l, row, col);
   var downScore = down.TakeWhile(lft => lft < startHeight).Count();
   if (down.Max() >= startHeight) downScore += 1;

   return leftScore * rightScore * upScore * downScore;
}

int Part1(List<string> lines)
{
   var visible = 0;
   for (int row = 1; row < lines.Count-1; row++)
   {
      for (int col = 1; col < lines[row].Length-1; col++)
      {
         var v = CheckNeighbours(lines, row, col);
         visible += v;
      }
   }

   return visible + (lines.Count * 2) + (lines[0].Length * 2) - 4;
}

int Part2(List<string> lines)
{
   var visible = 0;
   for (int row = 1; row < lines.Count-1; row++)
   {
      for (int col = 1; col < lines[row].Length-1; col++)
      {
         var v = CheckNeighboursScore(lines, row, col);
         visible = Int32.Max(visible, v);
      }
   }

   return visible;
}

Console.WriteLine($"part1: {Part1(lines)}");
Console.WriteLine($"part2: {Part2(lines)}");
