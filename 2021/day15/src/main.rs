use std::collections::HashMap;

fn parse_input(contents: &str) -> Vec<Vec<u32>> {
    contents
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn find_shortest_path(
    grid: &Vec<Vec<u32>>,
    cache: &mut HashMap<(usize, usize), i32>,
    r: usize,
    c: usize,
) -> Option<i32> {
    if let Some(cached) = cache.get(&(c, r)) {
        return Some(*cached);
    }

    let rmax = grid.len() as i32;
    let cmax = grid.get(0).unwrap().len() as i32;

    let mut paths = vec![];
    let val = grid.get(r).unwrap().get(c).map(|v| *v as i32);
    if r == (rmax as usize) - 1 && c == (cmax as usize) - 1 {
        return val;
    } else {
        for dr in [0, 1] {
            for dc in [0, 1] {
                let rr = (r as i32) + dr;
                let cc = (c as i32) + dc;
                if (dr != 0 && dc != 0) || (dr == 0 && dc == 0) {
                    continue;
                }
                if 0 <= rr && rr < rmax && 0 <= cc && cc < cmax {
                    if let Some(p) = find_shortest_path(grid, cache, rr as usize, cc as usize) {
                        let pv = val.unwrap() + p;
                        paths.push(pv);
                    }
                }
            }
        }
    }
    let min = paths.iter().min().map(|v| *v);

    if let Some(m) = min {
        cache.insert((c, r), m);
    }
    min
}

fn part1(contents: &str) -> i32 {
    let mut grid = parse_input(contents);
    let start = grid.get_mut(0).unwrap().get_mut(0).unwrap();
    *start = 0;

    let mut cache: HashMap<(usize, usize), i32> = HashMap::new();
    find_shortest_path(&grid, &mut cache, 0 as usize, 0 as usize).unwrap()
}

fn part2(contents: &str) -> i32 {
    let grid = parse_input(contents);
    let mut biggergrid: Vec<Vec<u32>> = vec![];
    let rmax = grid.len() as i32;
    let cmax = grid.get(0).unwrap().len() as i32;

    for r in 0..5 {
        for rr in 0..rmax {
            let mut row = vec![];
            for c in 0..5 {
                for cc in 0..cmax {
                    let mut v =
                        (*grid.get(rr as usize).unwrap().get(cc as usize).unwrap() + r + c) % 9;
                    if v == 0 {
                        v = 9;
                    }
                    row.push(v);
                }
            }
            biggergrid.push(row);
        }
    }

    let start = biggergrid.get_mut(0).unwrap().get_mut(0).unwrap();
    *start = 0;

    let mut cache: HashMap<(usize, usize), i32> = HashMap::new();
    find_shortest_path(&biggergrid, &mut cache, 0 as usize, 0 as usize).unwrap()
}

fn main() {
    let contents = include_str!("../input.txt");

    let part1 = part1(contents);
    println!("part1: {}", part1);

    let part2 = part2(contents);
    println!("part2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_minimal_1() {
        let contents = "11
13";
        assert_eq!(part1(contents), 4);
    }

    #[test]
    fn test_part_1_minimal_2() {
        let contents = "116
138
213";
        assert_eq!(part1(contents), 7);
    }

    #[test]
    fn test_minimal_3() {
        let contents = "118
818
188
111";
        assert_eq!(part1(contents), 6);
    }

    #[test]
    fn test_part_1() {
        let contents = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
        assert_eq!(part1(contents), 40);
    }

    #[test]
    fn test_part_2() {
        let contents = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
        assert_eq!(part2(contents), 315);
    }
}
