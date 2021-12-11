type Grid = Vec<Vec<usize>>;

fn parse_input(contents: &str) -> Grid {
    contents
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn get_point(grid: &Grid, x: usize, y: usize) -> Option<usize> {
    if let Some(e) = grid.get(x) {
        return e.get(y).map(|ee| *ee);
    }
    None
}

fn get_adjacent(grid: &Grid, x: usize, y: usize) -> Vec<(usize, usize, usize)> {
    let mut points = vec![];

    if let Some(v) = get_point(&grid, x, y + 1) {
        points.push((x, y + 1, v));
    }
    if let Some(v) = get_point(&grid, x, y - 1) {
        points.push((x, y - 1, v))
    }
    if let Some(v) = get_point(&grid, x + 1, y) {
        points.push((x + 1, y, v))
    }
    if let Some(v) = get_point(&grid, x - 1, y) {
        points.push((x - 1, y, v))
    }

    points
}

fn flash(grid: &mut Grid, pos: (usize, usize, &mut usize), flashes: &mut i32) {}

fn part1(contents: &str) -> i32 {
    let mut grid = parse_input(contents);
    let mut flashes = 0;

    for _ in 0..100 {
        for (i, x) in grid.iter_mut().enumerate() {
            for (j, y) in x.iter_mut().enumerate() {
                *y += 1;
                if y > &mut 9 {
                    flash(&mut grid, (i, j, y), &mut flashes);
                }
            }
        }
    }
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
