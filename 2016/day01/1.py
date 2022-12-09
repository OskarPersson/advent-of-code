import sys
from collections import defaultdict

infile = sys.argv[1] if len(sys.argv)>1 else 'input.txt'
data = open(infile).read().strip()

cmds = [x for x in data.split(", ")]
DIR = [(0,1), (1,0), (0,-1), (-1,0)]

d = 0
x = 0
y = 0

visited = [(0,0)];
dup = None;

for c in cmds:
    rot = c[0]
    n = int(c[1:])

    if rot == 'R':
        d += 1
        if d > 3:
            d = 0
    else:
        d -= 1
        if d < 0:
            d = 3

    for _ in range(n):
        x = x + DIR[d][0]
        y = y + DIR[d][1]
        if dup is None and (x, y) in visited:
            dup = abs(x) + abs(y)
        visited.append((x,y))


print("Part 1:", abs(x) + abs(y))
print("Part 2:", dup)
