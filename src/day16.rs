fn process(input: &str, steps: usize) -> i64 {
    let pattern = [0, 1, 0, -1];
    let mut values = input
        .trim()
        .as_bytes()
        .iter()
        .map(|&ch| (ch as u8 - b'0') as i64)
        .collect::<Vec<_>>();
    for step in 0..steps {
        let mut new_values = vec![0; values.len()];
        for i in 0..values.len() {
            for j in 0..values.len() {
                new_values[i] += values[j] * pattern[((j + 1) / (i + 1)) % pattern.len()];
            }
            new_values[i] = new_values[i].abs() % 10;
        }
        values = new_values;
    }
    let mut result = 0;
    for i in 0..8 {
        result *= 10;
        result += values[i];
    }
    result
}

pub fn part1(input: &str) -> i64 {
    process(input, 100)
}

pub fn part2(input: &str) -> i64 {
    let pattern = [0, 1, 0, -1];
    let mut offset = input
        .trim()
        .as_bytes()
        .iter()
        .take(7)
        .fold(0, |acc, &ch| acc * 10 + (ch as u8 - b'0') as usize);
    let input_len = input.trim().as_bytes().len();
    let mut values = std::iter::repeat(input.trim().as_bytes().iter())
        .flatten()
        .take(10000 * input_len)
        .skip(offset)
        .map(|&ch| (ch as u8 - b'0') as i64)
        .collect::<Vec<_>>();
    for step in 0..100 {
        let mut new_values = vec![0; values.len()];
        let mut sum = 0;
        for i in (0..values.len()).rev() {
            sum += values[i];
            sum = sum.abs() % 10;
            new_values[i] = sum;
        }
        values = new_values;
    }
    let mut result = 0;
    for i in 0..8 {
        result *= 10;
        result += values[i];
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day16_part1() {
        assert_eq!(process("12345678", 1), 48226158);
        assert_eq!(process("12345678", 2), 34040438);
        assert_eq!(process("12345678", 3), 03415518);
        assert_eq!(process("12345678", 4), 01029498);
        assert_eq!(process("80871224585914546619083218645595", 100), 24176176);
        assert_eq!(process("19617804207202209144916044189917", 100), 73745418);
        assert_eq!(process("69317163492948606335995924319873", 100), 52432133);
    }

    #[test]
    fn test_day16_part2() {
        assert_eq!(part2("03036732577212944063491565474664"), 84462026);
        assert_eq!(part2("02935109699940807407585447034323"), 78725270);
        assert_eq!(part2("03081770884921959731165446850517"), 53553731);
    }
}
