var lines = File.ReadAllLines(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "input.txt")).ToList();

var rl = lines[0]!.ToCharArray();

var nodes = new Dictionary<string, List<string>>();

foreach (var l in lines.Skip(2))
{

    var key = l.Split(" = ")[0];
    var val = l.Split(" = ")[1].Split(", ");
    var val1 = val[0].Split("(")[1];
    var val2 = val[1].Split(")")[0];
    nodes.Add(key, new List<string>() { val1, val2 });
}


var k = "AAA";
var instidx = 0;

var steps = 0;

while (k != "ZZZ")
{
    steps++;
    var inst = rl[instidx];

    if (inst == 'L')
    {
        k = nodes[k][0];
    }
    else if (inst == 'R')
    {
        k = nodes[k][1];
    }


    if (instidx == rl.Length - 1)
    {
        instidx = 0;
    }
    else
    {
        instidx++;
    }

}

Console.WriteLine($"part 1: {steps}");

var starts = nodes.Where(kv => kv.Key.EndsWith('A')).ToList();

var stepsToEndingZ = new List<long>();
foreach (var start in starts)
{
    stepsToEndingZ.Add(0);
    var node = start.Key;
    steps = 0;
    instidx = 0;

    while (!node.EndsWith('Z'))
    {
        steps++;
        var inst = rl[instidx];

        if (inst == 'L')
        {
            node = nodes[node][0];
        }
        else if (inst == 'R')
        {
            node = nodes[node][1];
        }

        if (instidx == rl.Length - 1)
        {
            instidx = 0;
        }
        else
        {
            instidx++;
        }
    }

    if (node.EndsWith('Z'))
    {
        stepsToEndingZ[^1] = steps;
    }
}

// Print least common multiplier of numbers in nums:
var lcm = stepsToEndingZ.Aggregate((a, b) => a * b / GCD(a, b));
Console.WriteLine($"part 2: {lcm}");

long GCD(long a, long b)
{
    while (b != 0)
    {
        var t = b;
        b = a % b;
        a = t;
    }

    return a;
}
