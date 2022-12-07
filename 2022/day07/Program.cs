using System.Runtime.InteropServices;
using System.Text.RegularExpressions;

var lines = File.ReadAllLines(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "input.txt")).ToList();

Dictionary<string, Dir> BuildFilesystem(List<string> lines)
{

   var cdPattern = @"\$ cd (.+)";
   var lsPattern = @"\$ ls";
   var filePattern = @"(\d+) .+";
   
   var pwd = new List<string>();
   var fs =  new Dictionary<string, Dir> { { "/", new Dir() } };

   pwd.Add("/");
      
   for (int i = 1; i < lines.Count; i++)
   {
      var line = lines[i];
      
      var cdMatch = Regex.Match(line, cdPattern);
      var lsMatch = Regex.Match(line, lsPattern);
      if (cdMatch.Success)
      {
         var dst = cdMatch.Groups[1].ToString();
         if (dst == "..")
         {
            pwd.RemoveAt(pwd.Count - 1);
         }
         else
         {
            pwd.Add(dst);
            var path = string.Join("/", pwd);
            if (!fs.ContainsKey(path))
            {
               fs.Add(path, new Dir());
            }
         }
      }
      else if (lsMatch.Success)
      {
         var pwdPath = string.Join("/", pwd);
         var lsFiles = lines.Skip(i + 1).TakeWhile(l => !l.StartsWith("$")).ToList();
         var lsDirs = lsFiles.Where(l => l.StartsWith("d"));
         
         foreach (var lsFile in lsFiles.Where(l => !l.StartsWith("d")))
         {
            var fileMatch = Regex.Match(lsFile, filePattern);
            var fileSize = int.Parse(fileMatch.Groups[1].ToString());
            fs[pwdPath].Size += fileSize;
         }

         foreach (var lsdir in lsDirs)
         {
            var lsdirName = new string(lsdir.ToCharArray().Skip(4).ToArray());
            var subpath = $"{string.Join("/", pwd)}/{lsdirName}";
            fs[pwdPath].Children.Add(subpath);
         }

         i += lsFiles.Count();
      }
   }

   return fs;
}

int GetSize(Dir dir, Dictionary<string, Dir> fs)
{
   return dir.Size + dir.Children.Sum(c => GetSize(fs[c], fs));
}

int Part1(List<string> lines)
{
   var fs = BuildFilesystem(lines);
   return fs.Values.Select(d => GetSize(d, fs)).Where(s => s < 100000).Sum();
}

int Part2(List<string> lines)
{
   var fs = BuildFilesystem(lines);

   var sizes = fs.Values.Select(d => GetSize(d, fs)).ToList();
   var totalSize = sizes[0];
   var needed = 30000000 - (70000000 - totalSize);

   return sizes.Skip(1).Where(s => s >= needed).Min();
}

Console.WriteLine($"part1: {Part1(lines)}");
Console.WriteLine($"part2: {Part2(lines)}");

class Dir
{
   public Dir()
   {
      Children = new List<string>();
      Size = 0;
   }

   public List<string> Children { get; set; }
   public int Size { get; set; }
}
