var lines = File.ReadAllLines(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "input.txt")).ToList();


for (int i = 0; i < lines.Count; i++)
{
    lines[i] = $".{lines[i]}.";
}

lines.Insert(0, new string('.', lines[0].Length));
lines.Add(new string('.', lines[0].Length));

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

var v = new List<Pos>() { new() { i = starti, j = startj, c = 'S' } };

const int steps = 0;
var ans = StepsToStart(starti, startj, steps, v);

Console.WriteLine($"part1: {ans.Count/2}");

var noPathToTarget = 0;

for (int i = 0; i < lines.Count; i++)
{
    for (int j = 0; j < lines[i].Length; j++)
    {
        if (j == lines[i].Length - 1)
        {
            continue;
        }
        
        var ch = lines[i][j];
        var nextch = lines[i][j + 1];
        
        if (!(ans.Any(x => x.i == i && x.j == j) && ans.Any(x => x.i == i && x.j == j + 1)))
        {
            continue;
        }
        

        if ((ch == '|' && nextch == '|')
            || (ch == '|' && nextch == 'S')
            || (ch == '|' && nextch == 'F')
            || (ch == '|' && nextch == 'L')
            || (ch == 'S' && nextch == '|')
            || (ch == 'S' && nextch == 'F')
            || (ch == 'S' && nextch == 'L')
            || (ch == '7' && nextch == 'S')
            || (ch == '7' && nextch == '|')
            || (ch == '7' && nextch == 'L')
            || (ch == '7' && nextch == 'F')
            || (ch == 'J' && nextch == 'S')
            || (ch == 'J' && nextch == '|')
            || (ch == 'J' && nextch == 'L')
            || (ch == 'J' && nextch == 'F')
           )
        {
            for (var k = 0; k < lines.Count; k++)
            {
                
                if (ans.Any(x => x.i == k && x.j == j) && ans.Any(x => x.i == k && x.j == j + 1))
                {
                    if ((lines[k][j] == 'S' && lines[k][j + 1] == '-')
                        || (lines[k][j] == 'S' && lines[k][j + 1] == 'J')
                        || (lines[k][j] == 'S' && lines[k][j + 1] == '7')
                        || (lines[k][j] == '-' && lines[k][j + 1] == '-')
                        || (lines[k][j] == '-' && lines[k][j + 1] == 'S')
                        || (lines[k][j] == 'L' && lines[k][j + 1] == '-')
                        || (lines[k][j] == 'L' && lines[k][j + 1] == 'S')
                        || (lines[k][j] == 'F' && lines[k][j + 1] == '-')
                        || (lines[k][j] == 'F' && lines[k][j + 1] == 'S')
                        || (lines[k][j] == '-' && lines[k][j + 1] == 'J')
                        || (lines[k][j] == '-' && lines[k][j + 1] == '7')
                        || (lines[k][j] == 'L' && lines[k][j + 1] == '7')
                        || (lines[k][j] == 'L' && lines[k][j + 1] == 'J')
                        || (lines[k][j] == 'F' && lines[k][j + 1] == '7')
                        || (lines[k][j] == 'F' && lines[k][j + 1] == 'J'))
                    {
                        ans = ans.Select(x =>
                        {
                            if (x.i == k && x.j >= j + 1)
                            {
                                return new Pos {i = k, j = x.j + 1, c = x.c};
                            }

                            return x;
                        }).ToList();
                        ans.Add(new Pos { i = k, j = j + 1, c = '-'});
                        lines[k] = lines[k].Insert(j + 1, "-");
                    }
                    else
                    {
                        ans = ans.Select(x =>
                        {
                            if (x.i == k && x.j >= j + 1)
                            {
                                return new Pos {i = k, j = x.j + 1, c = x.c};
                            }

                            return x;
                        }).ToList();
                        lines[k] = lines[k].Insert(j + 1, "X");
                    }
                }
                else
                {
                    ans = ans.Select(x =>
                    {
                        if (x.i == k && x.j >= j + 1)
                        {
                            return new Pos {i = k, j = x.j + 1, c = x.c};
                        }

                        return x;
                    }).ToList();
                    lines[k] = lines[k].Insert(j + 1, "X");
                }
            }
        }
    }
}

for (int j = 0; j < lines[0].Length; j++)
{
    for (int i = 0; i < lines.Count; i++)
    {
        if (i == lines.Count - 1)
        {
            continue;
        }
        
        if (j == lines.Count - 1)
        {
            continue;
        }
        
        var ch = lines[i][j];
        var nextch = lines[i+1][j];
        
        if (!(ans.Any(x => x.i == i && x.j == j) && ans.Any(x => x.i == i + 1 && x.j == j)))
        {
            continue;
        }

        
        if ((ch == '-' && nextch == '-')
            || (ch == '-' && nextch == 'S')
            || (ch == '-' && nextch == 'J')
            || (ch == '-' && nextch == '7')
            || (ch == 'S' && nextch == '-')
            || (ch == 'S' && nextch == 'J')
            || (ch == 'S' && nextch == '7')
            || (ch == 'L' && nextch == 'S')
            || (ch == 'L' && nextch == '-')
            || (ch == 'L' && nextch == 'J')
            || (ch == 'L' && nextch == '7')
            || (ch == 'F' && nextch == 'S')
            || (ch == 'F' && nextch == '-')
            || (ch == 'F' && nextch == 'J')
            || (ch == 'F' && nextch == '7')
           )
        {
            var newline = new string('X', lines[0].Length);
            for (var k = 0; k < lines[0].Length; k++)
            {
                if (ans.Any(x => x.i == i && x.j == k) && ans.Any(x => x.i == i + 1 && x.j == k))
                {

                    if ((lines[i][k] == 'S' && lines[i + 1][k] == '|')
                        || (lines[i][k] == 'S' && lines[i + 1][k] == 'L')
                        || (lines[i][k] == 'S' && lines[i + 1][k] == 'J')
                        || (lines[i][k] == '|' && lines[i + 1][k] == '|')
                        || (lines[i][k] == '|' && lines[i + 1][k] == 'L')
                        || (lines[i][k] == '|' && lines[i + 1][k] == 'J')
                        || (lines[i][k] == '|' && lines[i + 1][k] == 'S')
                        || (lines[i][k] == '7' && lines[i + 1][k] == '|')
                        || (lines[i][k] == '7' && lines[i + 1][k] == 'S')
                        || (lines[i][k] == 'F' && lines[i + 1][k] == '|')
                        || (lines[i][k] == 'F' && lines[i + 1][k] == 'S')
                        || (lines[i][k] == 'F' && lines[i + 1][k] == 'J')
                        || (lines[i][k] == 'F' && lines[i + 1][k] == 'L')
                        || (lines[i][k] == '7' && lines[i + 1][k] == 'J')
                        || (lines[i][k] == '7' && lines[i + 1][k] == 'L'))
                    {
                        newline = ReplaceAt(newline, k, "|");
                    }
                }
                else
                {
                    newline = ReplaceAt(newline, k, "X");
                }

            }
            
            for (int k = 0; k < newline.Length; k++)
            {
                ans = ans.Select(x =>
                {
                    if (x.i >= i + 1 && x.j == k)
                    {
                        return new Pos { i = x.i + 1, j = k, c = x.c };
                    }
                    return x;
                }).ToList();
            }

            for (int k = 0; k < newline.Length; k++)
            {
                if (newline[k] != 'X')
                {
                    ans.Add(new Pos { i = i + 1, j = k, c = '|' });
                }
            }
            
            lines.Insert(i+1, newline);
        }
    }
}

var visited = new bool[lines.Count, lines[0].Length];

// Flood fill from the outside
// If the cell is on the border, not a pipe and not visited, perform flood fill
FloodFill(0, 0);

// Count the number of non-pipe tiles that are not filled
int notFilledCount = 0;
for (int i = 0; i < lines.Count; i++)
{
    for (int j = 0; j < lines[0].Length; j++)
    {
        if (!ans.Any(x => x.i == i && x.j == j) && !visited[i, j] && lines[i][j] != 'X')
        {
            notFilledCount++;
        }
    }
}

Console.WriteLine($"part2: {notFilledCount}");

void FloodFill(int i, int j)
{
    // If the cell is out of bounds, a wall or already visited, return
    if (i < 0 || i >= lines.Count || j < 0 || j >= lines[0].Length || ans.Any(x => x.i == i && x.j == j) || visited[i, j])
    {
        return;
    }

    // Mark the cell as visited
    visited[i, j] = true;

    // Perform flood fill for the neighboring cells
    FloodFill(i - 1, j); // Up
    FloodFill(i + 1, j); // Down
    FloodFill(i, j - 1); // Left
    FloodFill(i, j + 1); // Right
}



List<Pos> StepsToStart(int starti, int startj, int startSteps, List<Pos> startVisited)
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

                        if ((c == '|' && rr != 0 && cc == 0)
                            || (c == '-' && cc != 0 && rr == 0)
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
                            return visited;
                        }

                    }

                    if (visited.Any(x => x.i == i + rr && x.j == j + cc))
                    {
                        continue;
                    }

                    var pos = new Pos() { i = i + rr, j = j + cc, c = lines[i+rr][j+cc] };

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

    return new List<Pos>();
}


string ReplaceAt(string input, int index, string newChar)
{
    char[] chars = input.ToCharArray();
    chars[index] = newChar.ToCharArray()[0];
    return new string(chars);
}


struct Pos
{
    public int i;
    public int j;
    public char c;
}
