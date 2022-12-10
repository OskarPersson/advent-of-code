var lines = File.ReadAllLines(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "input.txt")).ToList();

int Part1(List<string> lines)
{
   var reg = 1;
   var cycle = 1;
   var stack = new Queue<int>();

   var relevant = new List<int> { 20, 60, 100, 140, 180, 220 };
   var score = 0;

   for (int i = 0; i < lines.Count; i++)
   {
      while (stack.Count > 0)
      {
         if (relevant.Contains(cycle)) score += cycle * reg;
         
         reg += stack.FirstOrDefault();
         stack.TryDequeue(out _);
         
         cycle++;
      }
      
      if (lines[i] != "noop")
      {
         var instr = int.Parse(lines[i].Split(' ')[1]);
         stack.Enqueue(0);
         stack.Enqueue(instr);
      }

      if (relevant.Contains(cycle)) score += cycle * reg;

      reg += stack.FirstOrDefault();
      stack.TryDequeue(out _);
      cycle++;
   }

   while (stack.Count > 0)
   {
      if (relevant.Contains(cycle)) score += cycle * reg;
      
      reg += stack.FirstOrDefault();
      stack.TryDequeue(out _);
      cycle++;
   }
   
   return score;
}

string Part2(List<string> lines)
{
   var reg = 1;
   var stack = new Queue<int>();

   var crt = "";
   var crtpos = 0;

   for (int i = 0; i < lines.Count; i++)
   {
      if (crtpos == reg - 1 || crtpos == reg || crtpos == reg + 1)
      {
         crt += "#";
      }
      else
      {
         crt += ".";
      }

      while (stack.Count > 0)
      {
         reg += stack.FirstOrDefault();
         stack.TryDequeue(out _);

         crtpos++;
         
         if (crtpos > 39)
         {
            crt += "\n";
            crtpos = 0;
         }
         
         if (crtpos == reg - 1 || crtpos == reg || crtpos == reg + 1)
         {
            crt += "#";
         }
         else
         {
            crt += ".";
         }
      }
      
      if (lines[i] != "noop")
      {
         var instr = int.Parse(lines[i].Split(' ')[1]);
         stack.Enqueue(0);
         stack.Enqueue(instr);
      }

      reg += stack.FirstOrDefault();
      stack.TryDequeue(out _);
      crtpos++;
      
      if (crtpos > 39)
      {
         crt += "\n";
         crtpos = 0;
      }
   }

   while (stack.Count > 0)
   {
      reg += stack.FirstOrDefault();
      stack.TryDequeue(out _);
   }
   
   return crt;
}

Console.WriteLine($"part1: {Part1(lines)}");
Console.WriteLine($"part2: \n{Part2(lines)}");
