use std::collections::BTreeMap;

fn parse_step(bytes: &[u8]) -> (u8, i64) {
    let mut steps = 0;
    for i in 1..bytes.len() {
        steps *= 10;
        steps += (bytes[i] - b'0') as i64;
    }
    (bytes[0], steps)
}

pub fn part1(input: &str) -> i64 {
    let mut xs: BTreeMap<i64, Vec<(i64, i64)>> = BTreeMap::new();
    let mut ys: BTreeMap<i64, Vec<(i64, i64)>> = BTreeMap::new();
    let mut iter = input.lines();
    let mut pos = (0, 0);
    for value in iter.next().unwrap().trim().split(",") {
        let (dir, steps) = parse_step(value.trim().as_bytes());
        match dir {
            b'U' => {
                xs.entry(pos.0).or_default().push((pos.1, pos.1 + steps));
                pos = (pos.0, pos.1 + steps);
            }

            b'D' => {
                xs.entry(pos.0).or_default().push((pos.1, pos.1 - steps));
                pos = (pos.0, pos.1 - steps);
            }
            b'R' => {
                ys.entry(pos.1).or_default().push((pos.0, pos.0 + steps));
                pos = (pos.0 + steps, pos.1);
            }
            b'L' => {
                ys.entry(pos.1).or_default().push((pos.0, pos.0 - steps));
                pos = (pos.0 - steps, pos.1);
            }
            _ => panic!(""),
        }
    }
    let mut result = i64::MAX;
    pos = (0, 0);
    for value in iter.next().unwrap().trim().split(",") {
        let (dir, steps) = parse_step(value.trim().as_bytes());
        match dir {
            b'U' => {
                for (old_y, old_xs) in
                    ys.range(pos.1.min(pos.1 + steps)..pos.1.max(pos.1 + steps) + 1)
                {
                    for old_x in old_xs {
                        if old_x.0.min(old_x.1) < pos.0 && pos.0 < old_x.0.max(old_x.1) {
                            result = result.min(old_y.abs() + pos.0.abs());
                        }
                    }
                }
                pos = (pos.0, pos.1 + steps);
            }

            b'D' => {
                for (old_y, old_xs) in
                    ys.range(pos.1.min(pos.1 - steps)..pos.1.max(pos.1 - steps) + 1)
                {
                    for old_x in old_xs {
                        if old_x.0.min(old_x.1) < pos.0 && pos.0 < old_x.0.max(old_x.1) {
                            result = result.min(old_y.abs() + pos.0.abs());
                        }
                    }
                }
                pos = (pos.0, pos.1 - steps);
            }
            b'R' => {
                for (old_x, old_ys) in
                    xs.range(pos.0.min(pos.0 + steps)..pos.0.max(pos.0 + steps) + 1)
                {
                    for old_y in old_ys {
                        if old_y.0.min(old_y.1) < pos.1 && pos.1 < old_y.0.max(old_y.1) {
                            result = result.min(old_x.abs() + pos.1.abs());
                        }
                    }
                }
                pos = (pos.0 + steps, pos.1);
            }
            b'L' => {
                for (old_x, old_ys) in
                    xs.range(pos.0.min(pos.0 - steps)..pos.0.max(pos.0 - steps) + 1)
                {
                    for old_y in old_ys {
                        if old_y.0.min(old_y.1) < pos.1 && pos.1 < old_y.0.max(old_y.1) {
                            result = result.min(old_x.abs() + pos.1.abs());
                        }
                    }
                }
                pos = (pos.0 - steps, pos.1);
            }
            _ => panic!(""),
        }
    }
    result
}

pub fn part2(input: &str) -> i64 {
    0
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
        assert_eq!(part2(""), 0);
    }
}
