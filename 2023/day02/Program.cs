using System.Text.RegularExpressions;

var lines = File.ReadAllLines(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "input.txt"));

var gamepattern = @"Game (\d+)";
var redpattern = @"(\d+) red";
var greenpattern = @"(\d+) green";
var bluepattern = @"(\d+) blue";


int part1(IEnumerable<string> lines)
{

    var gameids = 0;

    foreach (var line in lines)
    {
        var gameid = int.Parse(Regex.Match(line, gamepattern).Groups[1].ToString());
        var red = Regex.Matches(line, redpattern).Select(m => int.Parse(m.Groups[1].ToString()));
        var green = Regex.Matches(line, greenpattern).Select(m => int.Parse(m.Groups[1].ToString()));
        var blue = Regex.Matches(line, bluepattern).Select(m => int.Parse(m.Groups[1].ToString()));
        
        if (red.All(r => r <= 12) && green.All(r => r <= 13) && blue.All(r => r <= 14))
        {
            gameids += gameid;
        }
    }

    return gameids;
}

int part2(IEnumerable<string> lines)
{
    var producs = 0;

    foreach (var line in lines)
    {
        var red = Regex.Matches(line, redpattern).Select(m => int.Parse(m.Groups[1].ToString())).Max();
        var green = Regex.Matches(line, greenpattern).Select(m => int.Parse(m.Groups[1].ToString())).Max();
        var blue = Regex.Matches(line, bluepattern).Select(m => int.Parse(m.Groups[1].ToString())).Max();

        producs += red * green * blue;

    }

    return producs;
}


Console.WriteLine($"part1: {part1(lines)}");
Console.WriteLine($"part2: {part2(lines)}");
