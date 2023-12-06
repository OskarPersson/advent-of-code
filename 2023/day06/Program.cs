using System.Text.RegularExpressions;

var lines = File.ReadAllLines(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "input.txt")).ToList();

long Calc(List<long> times, List<long> distances)
{
    long s = 1;
    for (var i = 0; i < times.Count; i++)
    {
        var t = times[i];
        var d = distances[i];

        var mid = t / 2;
        if (t % 2 == 1)
        {
            mid++;
        }

        var good = 0;

        for (var j = mid; j <= t; j++)
        {
            var traveled = j * (t-j);
            if (traveled > d)
            {
                good += 2;
            }
            else
            {
                if (t % 2 == 0) good--;
                
                break;
            }
        }

        s *= good;
    }

    return s;
}

var times = Regex.Matches(lines[0], @"(\d+)").Select(m => long.Parse(m.Value)).ToList();
var distances = Regex.Matches(lines[1], @"(\d+)").Select(m => long.Parse(m.Value)).ToList();

Console.WriteLine($"part1: {Calc(times, distances)}");

times = new List<long> { Int64.Parse(String.Join("", Regex.Matches(lines[0], @"(\d+)"))) };
distances = new List<long> { Int64.Parse(String.Join("", Regex.Matches(lines[1], @"(\d+)"))) };

Console.WriteLine($"part2: {Calc(times, distances)}");
