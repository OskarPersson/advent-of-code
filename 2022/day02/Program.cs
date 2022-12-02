var lines = File.ReadAllLines(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "input.txt"));

int CharPoint(char c)
{
   switch (c)
   {
      case 'A':
         return 1;
      case 'B':
         return 2;
      case 'C':
         return 3;
      case 'X':
         return 1;
      case 'Y':
         return 2;
      case 'Z':
         return 3;
        
      default:
         return 0;
   }
}

int Part1(IEnumerable<string> lines)
{
   var sums = new List<int>();
   foreach (var l in lines)
   {
      var chars = l.ToCharArray();
      var char1 = chars[0];
      var char2 = chars[2];

      if (char1 == 'A')
      {
         if (char2 == 'X')
         {
            sums.Add(CharPoint(char2) + 3);
         }

         if (char2 == 'Y')
         {
            sums.Add(CharPoint(char2) + 6);
         }

         if (char2 == 'Z')
         {
            sums.Add(CharPoint(char2) + 0);
         }
      }

      if (char1 == 'B')
      {

         if (char2 == 'X')
         {
            sums.Add(CharPoint(char2) + 0);
         }

         if (char2 == 'Y')
         {
            sums.Add(CharPoint(char2) + 3);
         }

         if (char2 == 'Z')
         {
            sums.Add(CharPoint(char2) + 6);
         }
      }

      if (char1 == 'C')
      {
         if (char2 == 'X')
         {
            sums.Add(CharPoint(char2) + 6);
         }

         if (char2 == 'Y')
         {
            sums.Add(CharPoint(char2) + 0);
         }

         if (char2 == 'Z')
         {
            sums.Add(CharPoint(char2) + 3);
         }
      }
   }
   return sums.Sum();
}

int Part2(IEnumerable<string> lines)
{
   var sums = new List<int>();
   foreach (var l in lines)
   {
      var chars = l.ToCharArray();
      var char1 = chars[0];
      var char2 = chars[2];

      if (char1 == 'A')
      {
         if (char2 == 'X')
         {
            sums.Add(CharPoint('Z') + 0);
         }

         if (char2 == 'Y')
         {
            sums.Add(CharPoint('X') + 3);
         }

         if (char2 == 'Z')
         {
            sums.Add(CharPoint('Y') + 6);
         }
      }

      if (char1 == 'B')
      {

         if (char2 == 'X')
         {
            sums.Add(CharPoint('A') + 0);
         }

         if (char2 == 'Y')
         {
            sums.Add(CharPoint('Y') + 3);
         }

         if (char2 == 'Z')
         {
            sums.Add(CharPoint('Z') + 6);
         }
      }

      if (char1 == 'C')
      {
         if (char2 == 'X')
         {
            sums.Add(CharPoint('Y') + 0);
         }

         if (char2 == 'Y')
         {
            sums.Add(CharPoint('Z') + 3);
         }

         if (char2 == 'Z')
         {
            sums.Add(CharPoint('X') + 6);
         }
      }
   }
   return sums.Sum();
}

Console.WriteLine($"part1: {Part1(lines)}");
Console.WriteLine($"part2: {Part2(lines)}");
