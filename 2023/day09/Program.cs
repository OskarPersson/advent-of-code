var lines = File.ReadAllLines(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "input.txt")).ToList();

var sum = 0;

foreach (var line in lines)
{
    var differences = new List<List<int>>();
    var nums = line.Split(" ").Select(int.Parse).ToList();
    
    differences.Add(new List<int>());
    for (int i = 0; i < nums.Count - 1; i++)
    {
        differences.Last().Add(nums[i + 1] - nums[i]);
    }
    
    while (differences.Last().Any(x => x != 0))
    {
        var lastRow = differences.Last();
        differences.Add(new List<int>());
        for (int i = 0; i < lastRow.Count - 1; i++)
        {
           differences.Last().Add(lastRow[i + 1] - lastRow[i]); 
        }
    }


    var rev = differences.AsEnumerable().Reverse().ToList();
    rev.Add(nums);
    for (int i = 0; i < rev.Count-1; i++)
    {
        var row = rev[i];
        var last = row.Last();
        
        
        var nextRow = rev[i+1];
        var nextRowLast = nextRow.Last();
        rev[i+1].Add(nextRowLast + last);
    }
    sum += rev.Last().Last();
}

Console.WriteLine($"part1: {sum}");

sum = 0;

foreach (var line in lines)
{
    var differences = new List<List<int>>();
    var nums = line.Split(" ").Select(int.Parse).ToList();
    
    differences.Add(new List<int>());
    for (int i = 0; i < nums.Count - 1; i++)
    {
        differences.Last().Add(nums[i + 1] - nums[i]);
    }
    
    while (differences.Last().Any(x => x != 0))
    {
        var lastRow = differences.Last();
        differences.Add(new List<int>());
        for (int i = 0; i < lastRow.Count - 1; i++)
        {
           differences.Last().Add(lastRow[i + 1] - lastRow[i]); 
        }
    }

    var rev = differences.AsEnumerable().Reverse().ToList();
    rev.Add(nums);
    for (int i = 0; i < rev.Count-1; i++)
    {
        var row = rev[i];
        var first = row.First();
        
        
        var nextRow = rev[i+1];
        var nextRowFirst = nextRow.First();
        rev[i+1].Insert(0, nextRowFirst - first);
    }
    sum += rev.Last().First();
}

Console.WriteLine($"part2: {sum}");
