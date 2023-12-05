var lines = File.ReadAllLines(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "input.txt")).ToList();


//number = Regex.Match(l.Substring(j), @"(\d+)").Groups[1].ToString();
var seeds = lines[0].Split(" ", 2)[1].Split(" ").Select(x =>
{
    return Int64.Parse(x);
}).ToList();


var zz = File.ReadAllText(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "input.txt")).Split("\n\n")
    .Where(x => !String.IsNullOrWhiteSpace(x)).Skip(1).ToList();
var ll = zz.Select(x => x.Split("\n").Skip(1).ToList()).Select(x => x.Select(y => y.Split(" ").Where(x => !String.IsNullOrWhiteSpace(x)).Select(z => Int64.Parse(z)).ToList()).Where(x => x.Count > 0).ToList() ).ToList();


var locations = new List<Int64>();

var seedGrps = seeds.Chunk(2).ToList();

for (int seedidx = 0; seedidx < seeds.Count; seedidx++)
{
    var seed = seeds[seedidx];
    var n = seed;

    for (int mapidx = 0; mapidx < ll.Count; mapidx++)
    {
        var map = ll[mapidx];

        for (int maplidx = 0; maplidx < map.Count; maplidx++)
        {
            var mapl = map[maplidx];

            var destRangeStart = mapl[0];
            var sourceRangeStart = mapl[1];
            var rangeLen = mapl[2];


            if (n >= sourceRangeStart && n < sourceRangeStart + rangeLen)
            {
                var nPlacement = n - sourceRangeStart;
                n = destRangeStart + nPlacement;
                break;
            }
        }

        if (mapidx == ll.Count - 1)
        {
            locations.Add(n);
        }
    }
}


Console.WriteLine($"part1: {locations.Min()}");

locations = new List<long>();

for (int seedgrpidx = 0; seedgrpidx < seedGrps.Count; seedgrpidx++)
{
    for (var seed = seedGrps[seedgrpidx][0]; seed < seedGrps[seedgrpidx][0] + seedGrps[seedgrpidx][1]; seed++)
    {
        var n = seed;

        for (int mapidx = 0; mapidx < ll.Count; mapidx++)
        {
            var map = ll[mapidx];

            for (int maplidx = 0; maplidx < map.Count; maplidx++)
            {
                var mapl = map[maplidx];

                var destRangeStart = mapl[0];
                var sourceRangeStart = mapl[1];
                var rangeLen = mapl[2];


                if (n >= sourceRangeStart && n < sourceRangeStart + rangeLen)
                {
                    var nPlacement = n - sourceRangeStart;
                    n = destRangeStart + nPlacement;
                    break;
                }
            }

            if (mapidx == ll.Count - 1)
            {
                locations.Add(n);
            }
        }
    }
}

Console.WriteLine($"part2: {locations.Min()}");
