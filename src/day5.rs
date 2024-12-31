fn parse(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i64>>()
}

fn value(values: &[i64], index: i64, step: i64, modes: i64) -> i64 {
    if modes / 10i64.pow(step as u32 - 1) % 10 == 0 {
        values[index as usize]
    } else {
        index
    }
}

fn execute(mut values: Vec<i64>) -> i64 {
    let mut index = 0i64;
    let mut output = 0;
    while values[index as usize] % 100 != 99 {
        let op = values[index as usize] % 100;
        let modes = values[index as usize] / 100;
        let ops = (1..4)
            .map(|i| value(&values, index + i, i, modes))
            .collect::<Vec<_>>();
        match op {
            1 => {
                values[ops[2] as usize] = values[ops[0] as usize] + values[ops[1] as usize];
                index += 4;
            }

            2 => {
                values[ops[2] as usize] = values[ops[0] as usize] * values[ops[1] as usize];
                index += 4;
            }
            3 => {
                values[ops[0] as usize] = 1;
                index += 2;
            }
            4 => {
                output = values[ops[0] as usize];
                index += 2;
            }
            _ => panic!("Unexpected command: {}", values[index as usize]),
        }
    }
    output
}

pub fn part1(input: &str) -> i64 {
    let values = parse(input);
    execute(values)
}

pub fn part2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day5_part1() {}

    #[test]
    fn test_day5_part2() {}
}
