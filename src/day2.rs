fn parse(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>()
}

fn execute(mut values: Vec<usize>) -> i64 {
    let mut index = 0;
    while values[index] != 99 {
        let ops = (1..4).map(|i| values[index + i]).collect::<Vec<_>>();
        match values[index] {
            1 => values[ops[2]] = values[ops[0]] + values[ops[1]],
            2 => values[ops[2]] = values[ops[0]] * values[ops[1]],
            _ => panic!("Unexpected command: {}", values[index]),
        }
        index += 4;
    }
    values[0] as i64
}

pub fn part1(input: &str) -> i64 {
    let mut values = parse(input);
    values[1] = 12;
    values[2] = 2;
    execute(values)
}

pub fn part2(input: &str) -> i64 {
    let original_values = parse(input);
    for noun in 0..100 {
        for verb in 0..100 {
            let mut values = original_values.clone();
            values[1] = noun;
            values[2] = verb;
            if execute(values) == 19690720 {
                return (100 * noun + verb) as i64;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2_part1() {
        assert_eq!(execute(parse("1,9,10,3,2,3,11,0,99,30,40,50")), 3500);
        assert_eq!(execute(parse("1,0,0,0,99")), 2);
        assert_eq!(execute(parse("2,3,0,3,99")), 2);
        assert_eq!(execute(parse("2,4,4,5,99,0")), 2);
        assert_eq!(execute(parse("1,1,1,4,99,5,6,0,99")), 30);
    }

    #[test]
    fn test_day2_part2() {}
}
