// Heavy inspiration taken from
// * https://doc.rust-lang.org/std/collections/binary_heap/index.html
// * https://github.com/AxlLind/AdventOfCode2021/blob/main/src/bin/15.rs
//
// Biggest mistake when trying to implement Dijsktra based on the BinaryHeap docs was putting the
// position as the first element of each BinaryHeap entry instead of the cost. This lead to the
// postions being compared on each pop, instead of the cost. Negative costs are used here because
// of the BinaryHeap by default being a max-heap.

use std::collections::{BinaryHeap, HashMap};

fn parse_input(contents: &str) -> Vec<Vec<u32>> {
    contents
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn dijkstra(grid: &Vec<Vec<u32>>, r: usize, c: usize) -> Option<i32> {
    let rmax = grid.len() as i32;
    let cmax = grid.get(0).unwrap().len() as i32;
    let goal = (rmax - 1, cmax - 1);

    let mut dist: HashMap<(i32, i32), i32> = HashMap::new();

    for (i, r) in grid.iter().enumerate() {
        for (j, _) in r.iter().enumerate() {
            dist.insert((i as i32, j as i32), i32::MAX);
        }
    }

    let mut heap = BinaryHeap::new();
    heap.push((0, (r as i32, c as i32)));

    while let Some((cost, position)) = heap.pop() {
        if position == goal {
            return Some(-cost);
        }

        if -cost > *dist.get(&position).unwrap() {
            continue;
        }

        for dc in [0, 1, -1] {
            for dr in [0, 1, -1] {
                let rr = (position.0 as i32) + dr;
                let cc = (position.1 as i32) + dc;
                if (dr != 0 && dc != 0) || (dr == 0 && dc == 0) {
                    continue;
                }
                if rr < 0 || rr >= rmax || cc < 0 || cc >= cmax {
                    continue;
                }

                let edge_pos = (rr, cc);

                let edge_cost =
                    -cost + *grid.get(rr as usize).unwrap().get(cc as usize).unwrap() as i32;

                if (edge_cost as i32) < *dist.get(&edge_pos).unwrap() {
                    let dep = dist.get_mut(&edge_pos).unwrap();
                    *dep = edge_cost;
                    heap.push((-edge_cost, edge_pos));
                }
            }
        }
    }

    // Goal not reachable
    None
}

fn part1(contents: &str) -> i32 {
    let mut grid = parse_input(contents);
    let start = grid.get_mut(0).unwrap().get_mut(0).unwrap();
    *start = 0;

    dijkstra(&grid, 0, 0).unwrap()
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

    dijkstra(&biggergrid, 0, 0).unwrap()
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
    fn test_part_1_minimal_3() {
        let contents = "118
818
118
181
111
";
        assert_eq!(part1(contents), 8);
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
