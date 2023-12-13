var patterns = File.ReadAllText(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "input.txt")).Split("\n\n")
    .Select(p => p.Split("\n").Where(p => !string.IsNullOrWhiteSpace(p)).ToArray()).ToList();

var sum = 0;
foreach (var p in patterns)
{
    for (var r = 0; r < p.Length - 1; r++)
    {
        var reflected = false;
        if (p[r] == p[r + 1])
        {
            reflected = true;
            var offset = 1;
            while (r - offset >= 0 && r + 1 + offset < p.Length)
            {
                if (p[r - offset] == p[r + 1 + offset])
                {
                    offset++;
                }
                else
                {
                    reflected = false;
                    break;
                }
            }
        }

        if (reflected)
        {
            sum += (r + 1) * 100;
            break;
        }
    }
    
    for (var c = 0; c < p[0].Length - 1; c++)
    {
        var reflected = false;
        
        var col = string.Join("", p.Select(r => r[c]).ToList());
        var nextCol = string.Join("", p.Select(r => r[c + 1]).ToList());
        if (col == nextCol)
        {
            reflected = true;
            
            var offset = 1;
            while (c - offset >= 0 && c + 1 + offset < p[0].Length)
            {
                
                col = string.Join("", p.Select(r => r[c - offset]).ToList());
                nextCol = string.Join("", p.Select(r => r[c + 1 + offset]).ToList());
                
                if (col == nextCol)
                {
                    offset++;
                }
                else
                {
                    reflected = false;
                    break;
                }
            }
        }
        
        
        if (reflected)
        {
            sum += c + 1;
            break;
        }
    }
}

Console.WriteLine($"part1: {sum}");

sum = 0;
foreach (var p in patterns)
{
    var good = false;
    var reflected = false;
    for (var r = 0; r < p.Length - 1; r++)
    {
        reflected = false;
        var l1 = p[r];
        var l2 = p[r + 1];

        var xored = xor(l1, l2);
        var smudged = xored.Count(c => c=='1') == 1;

        if (smudged || p[r] == p[r + 1])
        {
            reflected = true;
            var offset = 1;

            while (r - offset >= 0 && r + 1 + offset < p.Length)
            {
                l1 = p[r - offset];
                l2 = p[r + 1 + offset];
                xored = xor(l1, l2);

                if (l1 == l2)
                {
                    offset++;
                } else if (!smudged && xored.Count(c => c=='1') == 1)
                {
                    smudged = true;
                    offset++;
                }
                else
                {
                    reflected = false;
                    break;
                }
            }
        }

        if (reflected && smudged)
        {
            sum += (r + 1) * 100;
            good = true;
            break;
        }
    }

    if (good)
    {
        continue;
    }

    for (var c = 0; c < p[0].Length - 1; c++)
    {
        reflected = false;

        var col = string.Join("", p.Select(r => r[c]).ToList());
        var nextCol = string.Join("", p.Select(r => r[c + 1]).ToList());

        var xored = xor(col, nextCol);
        var smudged = xored.Count(c => c=='1') == 1;

        if (smudged || col == nextCol)
        {
            reflected = true;

            var offset = 1;
            while (c - offset >= 0 && c + 1 + offset < p[0].Length)
            {
                col = string.Join("", p.Select(r => r[c - offset]).ToList());
                nextCol = string.Join("", p.Select(r => r[c + 1 + offset]).ToList());
                xored = xor(col, nextCol);

                if (col == nextCol)
                {
                    offset++;
                } else if (!smudged && xored.Count(c => c=='1') == 1)
                {
                    smudged = true;
                    offset++;
                }
                else
                {
                    reflected = false;
                    break;
                }
            }
        }

        if (reflected && smudged)
        {
            sum += c + 1;
            break;
        }
    }
}

Console.WriteLine($"part2: {sum}");

string xor(string str1, string str2)
{
    var n1 = Convert.ToInt32(str1.Replace(".", "0").Replace("#", "1"), 2);
    var n2 = Convert.ToInt32(str2.Replace(".", "0").Replace("#", "1"), 2);

    var xorRes = n1 ^ n2;
    var xorResultBinary = Convert.ToString(xorRes, 2).PadLeft(str1.Length, '0');

    return xorResultBinary;
}
