use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug, Eq, PartialEq)]
struct Asteroid {
    x: i64,
    y: i64,
    n: i64,
    value: i64,
}

impl Asteroid {
    fn quadrant(&self) -> usize {
        match (self.x >= 0, self.y >= 0) {
            (true, true) => 1,
            (true, false) => 0,
            (false, true) => 2,
            (false, false) => 3,
        }
    }
}

impl Ord for Asteroid {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.n != other.n {
            return self.n.cmp(&other.n);
        }

        if self.quadrant() != other.quadrant() {
            return self.quadrant().cmp(&other.quadrant());
        }

        match self.quadrant() {
            0 => ((self.x.abs() as f64) / (self.y.abs() as f64))
                .partial_cmp(&((other.x.abs() as f64) / (other.y.abs() as f64)))
                .unwrap(),
            1 => ((self.y.abs() as f64) / (self.x.abs() as f64))
                .partial_cmp(&((other.y.abs() as f64) / (other.x.abs() as f64)))
                .unwrap(),
            2 => ((self.x.abs() as f64) / (self.y.abs() as f64))
                .partial_cmp(&((other.x.abs() as f64) / (other.y.abs() as f64)))
                .unwrap(),
            3 => ((self.y.abs() as f64) / (self.x.abs() as f64))
                .partial_cmp(&((other.y.abs() as f64) / (other.x.abs() as f64)))
                .unwrap(),
            _ => panic!(""),
        }
    }
}

impl PartialOrd for Asteroid {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .trim()
        .lines()
        .filter(|&line| !line.is_empty())
        .map(|line| line.trim().as_bytes().to_vec())
        .collect()
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    if a < b {
        std::mem::swap(&mut a, &mut b);
    }

    if b == 0 {
        return a;
    }

    loop {
        a %= b;
        if a == 0 {
            return b;
        }
        std::mem::swap(&mut a, &mut b);
    }
}

fn find_asteroids(asteroids: &[(usize, usize)], x: usize, y: usize) -> i64 {
    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    let mut diffs = asteroids
        .iter()
        .map(|&(a_x, a_y)| {
            (
                x.abs_diff(a_x) + y.abs_diff(a_y),
                a_x as i64 - x as i64,
                a_y as i64 - y as i64,
            )
        })
        .collect_vec();
    diffs.sort_unstable();
    diffs
        .iter()
        .skip(1)
        .map(|&(_, dx, dy)| {
            let gcd = gcd(dx.abs(), dy.abs());
            if !visited.contains(&(dx / gcd, dy / gcd)) {
                visited.insert((dx / gcd, dy / gcd));
                1
            } else {
                0
            }
        })
        .sum()
}

fn nth_asteroid(asteroids: &[(usize, usize)], x: usize, y: usize, index: usize) -> i64 {
    let mut visited: HashMap<(i64, i64), i64> = HashMap::new();
    let mut diffs = asteroids
        .iter()
        .map(|&(a_x, a_y)| {
            (
                x.abs_diff(a_x) + y.abs_diff(a_y),
                a_x as i64 - x as i64,
                a_y as i64 - y as i64,
            )
        })
        .collect_vec();
    diffs.sort_unstable();
    diffs
        .iter()
        .skip(1)
        .map(|&(_, dx, dy)| {
            let gcd = gcd(dx.abs(), dy.abs());
            let entry = visited.entry((dx / gcd, dy / gcd)).or_default();
            *entry += 1;
            Asteroid {
                x: dx,
                y: dy,
                n: *entry,
                value: 100 * (x as i64 + dx) + y as i64 + dy,
            }
        })
        .sorted_unstable()
        .nth(index)
        .map(|a| a.value)
        .unwrap()
}

pub fn part1(input: &str) -> i64 {
    let field = parse(input);
    let asteroids = (0..field.len())
        .flat_map(|i| {
            (0..field[0].len())
                .map(|j| (j, i))
                .filter(|&(j, i)| field[i][j] == b'#')
                .collect_vec()
        })
        .collect_vec();
    asteroids
        .iter()
        .map(|&(x, y)| find_asteroids(&asteroids, x, y))
        .max()
        .unwrap()
}

pub fn part2(input: &str) -> i64 {
    let field = parse(input);
    let asteroids = (0..field.len())
        .flat_map(|i| {
            (0..field[0].len())
                .map(|j| (j, i))
                .filter(|&(j, i)| field[i][j] == b'#')
                .collect_vec()
        })
        .collect_vec();
    let (_, x, y) = asteroids
        .iter()
        .map(|&(x, y)| (find_asteroids(&asteroids, x, y), x, y))
        .max()
        .unwrap();

    nth_asteroid(&asteroids, x, y, 199)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "
        .#..#
        .....
        #####
        ....#
        ...##
    ";

    const INPUT2: &str = "
        ......#.#.
        #..#.#....
        ..#######.
        .#.#.###..
        .#..#.....
        ..#....#.#
        #..#....#.
        .##.#..###
        ##...#..#.
        .#....####
    ";

    const INPUT3: &str = "
        #.#...#.#.
        .###....#.
        .#....#...
        ##.#.#.#.#
        ....#.#.#.
        .##..###.#
        ..#...##..
        ..##....##
        ......#...
        .####.###.
    ";

    const INPUT4: &str = "
        .#..#..###
        ####.###.#
        ....###.#.
        ..###.##.#
        ##.##.#.#.
        ....###..#
        ..#.#..#.#
        #..#.#.###
        .##...##.#
        .....#.#..
    ";

    const INPUT5: &str = "
        .#..##.###...#######
        ##.############..##.
        .#.######.########.#
        .###.#######.####.#.
        #####.##.#.##.###.##
        ..#####..#.#########
        ####################
        #.####....###.#.#.##
        ##.#################
        #####.##.###..####..
        ..######..##.#######
        ####.##.####...##..#
        .#####..#.######.###
        ##...#.##########...
        #.##########.#######
        .####.#.###.###.#.##
        ....##.##.###..#####
        .#.#.###########.###
        #.#.#.#####.####.###
        ###.##.####.##.#..##
    ";

    #[test]
    fn test_day10_part1() {
        assert_eq!(part1(INPUT1), 8);
        assert_eq!(part1(INPUT2), 33);
        assert_eq!(part1(INPUT3), 35);
        assert_eq!(part1(INPUT4), 41);
        assert_eq!(part1(INPUT5), 210);
    }

    #[test]
    fn test_day10_part2() {
        assert_eq!(part2(INPUT5), 802);
    }
}
