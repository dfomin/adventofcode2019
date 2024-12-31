use std::cmp::Ordering;

fn solve(low: i64, high: i64, strict_pair: bool) -> i64 {
    let mut result = 0;
    for mut i in low..high + 1 {
        let mut pair_counter = 1;
        let mut pair = false;
        let mut ok = true;
        while i > 0 {
            let cur = i % 10;
            let next = (i / 10) % 10;
            match cur.cmp(&next) {
                Ordering::Equal => {
                    pair_counter += 1;
                    if !strict_pair {
                        pair = true;
                    }
                }
                Ordering::Less => {
                    ok = false;
                    break;
                }
                Ordering::Greater => {
                    if pair_counter == 2 {
                        pair = true;
                    }
                    pair_counter = 1;
                }
            }
            i /= 10;
        }
        if ok && pair {
            result += 1;
        }
    }
    result
}

pub fn part1(input: &str) -> i64 {
    let mut iter = input.split("-");
    let low = iter.next().unwrap().parse::<i64>().unwrap();
    let high = iter.next().unwrap().parse::<i64>().unwrap();
    solve(low, high, false)
}

pub fn part2(input: &str) -> i64 {
    let mut iter = input.split("-");
    let low = iter.next().unwrap().parse::<i64>().unwrap();
    let high = iter.next().unwrap().parse::<i64>().unwrap();
    solve(low, high, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day4_part1() {}

    #[test]
    fn test_day4_part2() {}
}
