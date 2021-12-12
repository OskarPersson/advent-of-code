use std::collections::HashMap;

type Grid = HashMap<usize, HashMap<usize, usize>>;

fn parse_input(contents: &str) -> Grid {
    let mut map: Grid = HashMap::new();
    for (x, l) in contents.lines().enumerate() {
        for (y, c) in l.chars().enumerate() {
            let d = c.to_digit(10).unwrap() as usize;
            if let Some(e) = map.get_mut(&x) {
                e.insert(y, d);
            } else {
                let mut xmap: HashMap<usize, usize> = HashMap::new();
                xmap.insert(y, d);
                map.insert(x, xmap);
            }
        }
    }
    map
}

fn get_point(grid: &Grid, x: usize, y: usize) -> Option<usize> {
    if let Some(e) = grid.get(&x) {
        return e.get(&y).map(|ee| *ee);
    }
    None
}

fn get_adjacent(grid: &Grid, x: usize, y: usize) -> Vec<(usize, usize, usize)> {
    let mut points = vec![];

    if let Some(v) = get_point(&grid, x, y + 1) {
        points.push((x, y + 1, v));
    }
    if let Some(v) = get_point(&grid, x + 1, y) {
        points.push((x + 1, y, v))
    }
    if let Some(v) = get_point(&grid, x + 1, y + 1) {
        points.push((x + 1, y + 1, v))
    }
    if x > 0 && y > 0 {
        if let Some(v) = get_point(&grid, x - 1, y - 1) {
            points.push((x - 1, y - 1, v))
        }
    }
    if x > 0 {
        if let Some(v) = get_point(&grid, x - 1, y) {
            points.push((x - 1, y, v))
        }
        if let Some(v) = get_point(&grid, x - 1, y + 1) {
            points.push((x - 1, y + 1, v))
        }
    }
    if y > 0 {
        if let Some(v) = get_point(&grid, x, y - 1) {
            points.push((x, y - 1, v))
        }
        if let Some(v) = get_point(&grid, x + 1, y - 1) {
            points.push((x + 1, y - 1, v))
        }
    }

    points
}

fn flash(grid: &mut Grid, x: usize, y: usize, flashes: &mut i32) {
    *flashes += 1;
    for adj in get_adjacent(grid, x, y) {
        increase_energy(grid, adj.0, adj.1, flashes);
    }
}

fn increase_energy(grid: &mut Grid, x: usize, y: usize, flashes: &mut i32) -> () {
    let point = grid.get_mut(&x).unwrap().get_mut(&y).unwrap();
    *point += 1;

    if *point == 10 {
        flash(grid, x, y, flashes);
        let p = grid.get_mut(&x).unwrap().get_mut(&y).unwrap();
        *p += 1;
    }
}

fn run_steps(grid: &mut Grid, steps: i32) -> i32 {
    let mut flashes = 0;
    let grid_size = 10;
    for _ in 0..steps {
        for i in 0..grid_size {
            for j in 0..grid_size {
                increase_energy(grid, i, j, &mut flashes)
            }
        }
        for (_, x) in grid.iter_mut() {
            for (_, y) in x.iter_mut() {
                if *y > 9 {
                    *y = 0;
                }
            }
        }
    }
    flashes
}

fn part1(contents: &str) -> i32 {
    let mut grid = parse_input(contents);
    let flashes = run_steps(&mut grid, 100);
    flashes
}

fn main() {
    let contents = include_str!("../input.txt");

    let part1 = part1(contents);
    println!("part1: {}", part1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let contents = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        assert_eq!(part1(contents), 1656);
    }
}
