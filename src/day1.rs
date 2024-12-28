fn count_fuel(mut value: i64) -> i64 {
    let mut result = 0;
    loop {
        value = value / 3 - 2;
        if value <= 0 {
            return result;
        }
        result += value;
    }
}

pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| line.trim().parse::<i64>().unwrap())
        .map(|v| v / 3 - 2)
        .sum()
}

pub fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| line.trim().parse::<i64>().unwrap())
        .map(count_fuel)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "12
    14
    1969
    100756";

    #[test]
    fn test_day1_part1() {
        assert_eq!(part1(INPUT), 2 + 2 + 654 + 33583);
    }

    #[test]
    fn test_day1_part2() {
        assert_eq!(part2(INPUT), 2 + 2 + 966 + 50346);
    }
}
