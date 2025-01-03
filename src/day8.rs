fn decode(input: &str, width: usize, height: usize) -> String {
    let mut image = vec![vec![b'2'; width]; height];
    input
        .trim()
        .as_bytes()
        .chunks(width * height)
        .fold(&mut image, |image, chunk| {
            chunk.iter().enumerate().for_each(|(i, &ch)| {
                let x = i % width;
                let y = i / width;
                if image[y][x] == b'2' && ch != b'2' {
                    image[y][x] = if ch == b'0' { b' ' } else { b'#' };
                }
            });
            image
        })
        .iter()
        .map(|row| std::str::from_utf8(&row).unwrap())
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn part1(input: &str) -> i64 {
    input
        .trim()
        .as_bytes()
        .chunks(25 * 6)
        .map(|chunk| {
            chunk.iter().fold((0, 0, 0), |mut acc, &ch| {
                match ch {
                    b'0' => acc.0 += 1,
                    b'1' => acc.1 += 1,
                    b'2' => acc.2 += 1,
                    _ => (),
                }
                acc
            })
        })
        .min()
        .map(|(_, x, y)| x * y)
        .unwrap()
}

pub fn part2(input: &str) -> String {
    decode(input, 25, 6)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day8_part1() {}

    #[test]
    fn test_day8_part2() {
        assert_eq!(decode("0222112222120000", 2, 2), String::from(" #\n# "));
    }
}
