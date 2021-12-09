use std::collections::HashMap;

type Grid = HashMap<i32, HashMap<i32, i32>>;

fn parse_input(contents: &str) -> Grid {
    let mut map: Grid = HashMap::new();
    for (x, l) in contents.lines().enumerate() {
        for (y, c) in l.chars().enumerate() {
            let d = c.to_digit(10).unwrap() as i32;
            if let Some(e) = map.get_mut(&(x as i32)) {
                e.insert(y as i32, d);
            } else {
                let mut xmap: HashMap<i32, i32> = HashMap::new();
                xmap.insert(y as i32, d);
                map.insert(x as i32, xmap);
            }
        }
    }
    map
}

fn get_point(entries: &Grid, x: i32, y: i32) -> Option<i32> {
    if let Some(e) = entries.get(&x) {
        return e.get(&y).map(|ee| *ee);
    }
    None
}

fn adjacent_points(entries: &Grid, x: i32, y: i32) -> Vec<(i32, i32, i32)> {
    let mut points = vec![];

    if let Some(v) = get_point(&entries, x, y + 1) {
        points.push((x, y + 1, v));
    }
    if let Some(v) = get_point(&entries, x, y - 1) {
        points.push((x, y - 1, v))
    }
    if let Some(v) = get_point(&entries, x + 1, y) {
        points.push((x + 1, y, v))
    }
    if let Some(v) = get_point(&entries, x - 1, y) {
        points.push((x - 1, y, v))
    }

    points
}

fn get_low_points(entries: &Grid) -> Vec<(i32, i32, i32)> {
    let mut points = vec![];
    for (i, x) in entries {
        for (j, y) in x {
            if adjacent_points(&entries, *i, *j)
                .iter()
                .all(|(_, _, adj)| adj > y)
            {
                points.push((*i, *j, *y));
            }
        }
    }
    points
}

fn part1(contents: &str) -> i32 {
    let entries = parse_input(contents);
    get_low_points(&entries)
        .into_iter()
        .map(|(_, _, r)| r + 1)
        .sum()
}

fn basin_points(grid: &Grid, x: i32, y: i32, v: i32, acc: &mut Vec<(i32, i32, i32)>) -> i32 {
    let higher_points: Vec<(i32, i32, i32)> = adjacent_points(&grid, x, y)
        .into_iter()
        .filter(|(_, _, av)| *av > v && *av < 9)
        .collect();

    acc.append(&mut higher_points.clone());

    let res = higher_points
        .iter()
        .map(|(ax, ay, av)| basin_points(grid, *ax, *ay, *av, acc))
        .count() as i32;
    res
}

fn part2(contents: &str) -> i32 {
    let entries = parse_input(contents);
    let low_points = get_low_points(&entries);

    let mut basins: Vec<i32> = low_points
        .iter()
        .map(|(x, y, v)| {
            let mut vv = vec![(*x, *y, *v)];
            basin_points(&entries, *x, *y, *v, &mut vv);
            vv.sort();
            vv.dedup_by(|(vvx1, vvy1, _), (vvx2, vvy2, _)| vvx1 == vvx2 && vvy1 == vvy2);
            vv.len() as i32
        })
        .collect();

    basins.sort();
    basins.reverse();
    basins.iter().take(3).product()
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
    fn test_part_1() {
        let contents = "2199943210
3987894921
9856789892
8767896789
9899965678";
        assert_eq!(part1(contents), 15);
    }

    #[test]
    fn test_part_2() {
        let contents = "2199943210
3987894921
9856789892
8767896789
9899965678";
        assert_eq!(part2(contents), 1134);
    }
}
