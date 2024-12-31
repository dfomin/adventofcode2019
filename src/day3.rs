#[derive(Debug, Clone, Copy)]
struct Line {
    fixed: i64,
    start: i64,
    end: i64,
    distance: i64,
}

impl Line {
    fn intersection_distance(&self, other: &Line) -> i64 {
        self.distance
            + (other.fixed - self.start).abs()
            + other.distance
            + (self.fixed - other.start).abs()
    }

    fn contains(&self, point: i64) -> bool {
        self.start.min(self.end) < point && point < self.start.max(self.end)
    }

    fn intersect(&self, other: &Line) -> Option<(i64, i64, i64)> {
        if self.contains(other.fixed) && other.contains(self.fixed) {
            Some((self.fixed, other.fixed, self.intersection_distance(other)))
        } else {
            None
        }
    }
}

fn parse_step(bytes: &[u8]) -> (u8, i64) {
    let mut steps = 0;
    for byte in &bytes[1..] {
        steps *= 10;
        steps += (byte - b'0') as i64;
    }
    (bytes[0], steps)
}

fn parse_line(line: &str) -> Vec<Vec<Line>> {
    let mut pos = (0, 0);
    let mut distance = 0;
    let mut lines = vec![vec![]; 2];
    for value in line.trim().split(",") {
        let (dir, steps) = parse_step(value.trim().as_bytes());
        let (new_pos, horizontal) = match dir {
            b'U' => ((pos.0, pos.1 + steps), false),
            b'D' => ((pos.0, pos.1 - steps), false),
            b'R' => ((pos.0 + steps, pos.1), true),
            b'L' => ((pos.0 - steps, pos.1), true),
            _ => panic!(""),
        };
        let line = if horizontal {
            Line {
                fixed: pos.1,
                start: pos.0,
                end: new_pos.0,
                distance,
            }
        } else {
            Line {
                fixed: pos.0,
                start: pos.1,
                end: new_pos.1,
                distance,
            }
        };
        distance += (pos.0 - new_pos.0).abs() + (pos.1 - new_pos.1).abs();
        pos = new_pos;

        let index = if horizontal { 0 } else { 1 };
        lines[index].push(line);
    }
    lines
}

fn intersect_lines(hor_lines: &[Line], ver_lines: &[Line], length: bool) -> i64 {
    hor_lines
        .iter()
        .filter_map(|hor_line| {
            ver_lines
                .iter()
                .filter_map(|ver_line| ver_line.intersect(hor_line))
                .map(|point| {
                    if !length {
                        point.0.abs() + point.1.abs()
                    } else {
                        point.2
                    }
                })
                .min()
        })
        .min()
        .unwrap_or(i64::MAX)
}

fn solve(input: &str, length: bool) -> i64 {
    let mut iter = input.lines();
    let first_lines = parse_line(iter.next().unwrap());
    let second_lines = parse_line(iter.next().unwrap());

    intersect_lines(&first_lines[0], &second_lines[1], length).min(intersect_lines(
        &second_lines[0],
        &first_lines[1],
        length,
    ))
}

pub fn part1(input: &str) -> i64 {
    solve(input, false)
}

pub fn part2(input: &str) -> i64 {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "R8,U5,L5,D3
        U7,R6,D4,L4";

    const INPUT2: &str = "R75,D30,R83,U83,L12,D49,R71,U7,L72
        U62,R66,U55,R34,D71,R55,D58,R83";

    const INPUT3: &str = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
        U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

    #[test]
    fn test_day3_part1() {
        assert_eq!(part1(INPUT1), 6);
        assert_eq!(part1(INPUT2), 159);
        assert_eq!(part1(INPUT3), 135);
    }

    #[test]
    fn test_day3_part2() {
        assert_eq!(part2(INPUT1), 30);
        assert_eq!(part2(INPUT2), 610);
        assert_eq!(part2(INPUT3), 410);
    }
}
