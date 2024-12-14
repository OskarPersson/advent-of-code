var lines = File.ReadAllLines(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "input.txt"));

int part1(IEnumerable<string> lines)
{
    var rows = lines.Count();
    var cols = lines.First().Length;

    var sum = 0;

    for (int i = 0; i < rows; i++)
    {
        for (int j = 0; j < cols; j++)
        {
            var c = lines.ElementAt(i)[j];
            if (c == 'X')
            {
                if (j <= cols - 4 && lines.ElementAt(i)[j + 1] == 'M' && lines.ElementAt(i)[j + 2] == 'A' &&
                    lines.ElementAt(i)[j + 3] == 'S')
                {
                    // right
                    sum++;
                }

                if (i <= rows - 4 && lines.ElementAt(i + 1)[j] == 'M' && lines.ElementAt(i + 2)[j] == 'A' &&
                    lines.ElementAt(i + 3)[j] == 'S')
                {
                    // down
                    sum++;
                }

                if (j >= 3 && lines.ElementAt(i)[j - 1] == 'M' && lines.ElementAt(i)[j - 2] == 'A' &&
                    lines.ElementAt(i)[j - 3] == 'S')
                {
                    // left
                    sum++;
                }

                if (i >= 3 && lines.ElementAt(i - 1)[j] == 'M' && lines.ElementAt(i - 2)[j] == 'A' &&
                    lines.ElementAt(i - 3)[j] == 'S')
                {
                    // up
                    sum++;
                }

                if (i <= rows - 4 && j <= cols - 4 && lines.ElementAt(i + 1)[j + 1] == 'M' && lines.ElementAt(i + 2)[j + 2] == 'A' &&
                    lines.ElementAt(i + 3)[j + 3] == 'S')
                {
                    // down right
                    sum++;
                }

                if (i >= 3 && j >= 3 && lines.ElementAt(i - 1)[j - 1] == 'M' && lines.ElementAt(i - 2)[j - 2] == 'A' &&
                    lines.ElementAt(i - 3)[j - 3] == 'S')
                {
                    // up left
                    sum++;
                }

                if (i >= 3 && j <= cols - 4 && lines.ElementAt(i - 1)[j + 1] == 'M' && lines.ElementAt(i - 2)[j + 2] == 'A' &&
                    lines.ElementAt(i - 3)[j + 3] == 'S')
                {
                    // up right
                    sum++;
                }

                if (i <= rows - 4 && j >= 3 && lines.ElementAt(i + 1)[j - 1] == 'M' &&
                    lines.ElementAt(i + 2)[j - 2] == 'A' &&
                    lines.ElementAt(i + 3)[j - 3] == 'S')
                {
                    // down left
                    sum++;
                }
            }
        }
    }

    return sum;
}

int part2(IEnumerable<string> lines)
{
    var rows = lines.Count();
    var cols = lines.First().Length;

    var sum = 0;

    for (int i = 1; i < rows - 1; i++)
    {
        for (int j = 1; j < cols - 1; j++)
        {
            var c = lines.ElementAt(i)[j];
            if (c == 'A')
            {
                var downRight = (i <= rows && j <= cols && lines.ElementAt(i - 1)[j - 1] == 'M' &&
                                 lines.ElementAt(i + 1)[j + 1] == 'S');
                var downLeft = (i <= rows && j >= 1 && lines.ElementAt(i - 1)[j + 1] == 'M' &&
                                lines.ElementAt(i + 1)[j - 1] == 'S');

                var upRight = (i >= 1 && j <= cols && lines.ElementAt(i + 1)[j - 1] == 'M' &&
                               lines.ElementAt(i - 1)[j + 1] == 'S');

                var upLeft = (i >= 1 && j >= 1 && lines.ElementAt(i + 1)[j + 1] == 'M' &&
                              lines.ElementAt(i - 1)[j - 1] == 'S');

                if (downRight && (upRight || downLeft)) sum++;
                if (upRight && (downRight || upLeft)) sum++;
                if (downLeft && (upLeft || downRight)) sum++;
                if (upLeft && (downLeft || upRight)) sum++;
            }
        }
    }
    return sum / 2;
}


Console.WriteLine($"part1: {part1(lines)}");
Console.WriteLine($"part2: {part2(lines)}");
