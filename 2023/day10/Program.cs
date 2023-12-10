var lines = File.ReadAllLines(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "input.txt")).ToList();

var starti = 0;
var startj = 0;

for (int i = 0; i < lines.Count; i++)
{
    var line = lines[i];
    for (int j = 0; j < line.Length; j++)
    {
        if (line[j] == 'S')
        {
            starti = i;
            startj = j;
            break;
        }
    }
}



var v = new List<Pos>() { new() { i = starti, j = startj } };

const int steps = 0;
var ans = StepsToStart(starti, startj, steps, v);
Console.WriteLine($"part1: {ans/2}");


int? StepsToStart(int starti, int startj, int startSteps, List<Pos> startVisited)
{
    var stack = new Stack<(int i, int j, int steps, List<Pos> visited)>();
    stack.Push((starti, startj, startSteps, startVisited));

    int? minSteps = null;

    while (stack.Count > 0)
    {
        var (i, j, steps, visited) = stack.Pop();

        foreach (var rr in new[] { -1, 0, 1 })
        {
            foreach (var cc in new[] { -1, 0, 1 })
            {
                if (0 <= i + rr && i + rr < lines.Count && 0 <= j + cc && j + cc < lines[0].Length)
                {
                    if (cc != 0 && rr != 0)
                    {
                        continue;
                    }

                    var ch = lines[i + rr][j + cc];

                    if (steps > 1 && ch == 'S')
                    {
                        var c = lines[i][j];

                        if ((c == '|' && rr == 0 && cc != 0)
                            || (c == '-' && cc == 0 && rr != 0)
                            || (c == 'L' && cc == 0 && rr == -1)
                            || (c == 'L' && cc == 1 && rr == 0)
                            || (c == 'J' && cc == 0 && rr == -1)
                            || (c == 'J' && cc == -1 && rr == 0)
                            || (c == '7' && cc == -1 && rr == 0)
                            || (c == '7' && cc == 0 && rr == 1)
                            || (c == 'F' && cc == 1 && rr == 0)
                            || (c == 'F' && cc == 0 && rr == 1)
                            )
                        {
                            minSteps = minSteps == null ? steps + 1 : Math.Min(minSteps.Value, steps + 1);
                            return minSteps;
                        }

                    }

                    if (visited.Any(x => x.i == i + rr && x.j == j + cc))
                    {
                        continue;
                    }

                    var pos = new Pos() { i = i + rr, j = j + cc };

                    if (rr == 0 && cc == 0)
                    {
                        continue;
                    }

                    var v2 = visited.ToList();
                    v2.Add(pos);

                    if ((cc == 1 && rr == 0 && ch is '-' or 'J' or '7') ||
                        (cc == -1 && rr == 0 && ch is '-' or 'L' or 'F') ||
                        (cc == 0 && rr == -1 && ch is '|' or '7' or 'F') ||
                        (cc == 0 && rr == 1 && ch is '|' or 'J' or 'L'))
                    {
                        
                        //Console.WriteLine(lines[i + rr][j + cc]);
                        stack.Push((i + rr, j + cc, steps + 1, v2));
                    }
                }
            }
        }
    }

    return minSteps;
}




struct Pos
{
    public int i;
    public int j;
}
