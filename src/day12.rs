use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Moon {
    pos: [i64; 3],
    speed: [i64; 3],
}

impl Moon {
    fn energy(&self) -> i64 {
        self.pos.iter().map(|x| x.abs()).sum::<i64>()
            * self.speed.iter().map(|x| x.abs()).sum::<i64>()
    }

    fn update(&mut self) {
        self.pos.iter_mut().zip(self.speed).for_each(|(p, s)| {
            *p += s;
        });
    }

    fn is_static(&self) -> bool {
        self.speed.into_iter().all(|s| s == 0)
    }
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

fn apply_gravity(first: &mut Moon, second: &mut Moon) {
    for i in 0..3 {
        if first.pos[i] > second.pos[i] {
            first.speed[i] -= 1;
            second.speed[i] += 1;
        } else if first.pos[i] < second.pos[i] {
            first.speed[i] += 1;
            second.speed[i] -= 1;
        }
    }
}

fn simulate(moons: &mut [Moon], steps: usize) -> i64 {
    (0..steps).for_each(|_| {
        (0..moons.len())
            .tuple_combinations()
            .for_each(|pair: (usize, usize)| {
                let (first, second) = moons.split_at_mut(pair.1);
                apply_gravity(&mut first[pair.0], &mut second[0]);
            });
        moons.iter_mut().for_each(|moon| moon.update());
    });
    moons.iter().map(|moon| moon.energy()).sum()
}

fn parse(input: &str) -> Vec<Moon> {
    let re = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();
    re.captures_iter(input)
        .map(|captures| {
            let values: [i64; 3] = captures
                .extract::<3>()
                .1
                .iter()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            Moon {
                pos: values,
                speed: [0; 3],
            }
        })
        .collect()
}

pub fn part1(input: &str) -> i64 {
    let mut moons = parse(input);
    simulate(&mut moons, 1000)
}

pub fn part2(input: &str) -> i64 {
    let mut moons = parse(input);
    let initials = moons.iter().map(|moon| moon.pos).collect::<Vec<_>>();
    let mut cycles: Vec<Option<i64>> = vec![None; 3];
    let mut cycles_left = cycles.len();
    for i in 1.. {
        simulate(&mut moons, 1);
        for k in 0..3 {
            if cycles[k].is_none()
                && moons
                    .iter()
                    .zip(initials.iter())
                    .all(|(moon, initial)| moon.speed[k] == 0 && moon.pos[k] == initial[k])
            {
                cycles[k] = Some(i);
                cycles_left -= 1;
                if cycles_left == 0 {
                    let lcm = cycles[0].unwrap() * cycles[1].unwrap()
                        / gcd(cycles[0].unwrap(), cycles[1].unwrap());
                    return cycles[2].unwrap() * lcm / gcd(lcm, cycles[2].unwrap());
                }
            }
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "
        <x=-1, y=0, z=2>
        <x=2, y=-10, z=-7>
        <x=4, y=-8, z=8>
        <x=3, y=5, z=-1>
    ";

    const INPUT2: &str = "
        <x=-8, y=-10, z=0>
        <x=5, y=5, z=10>
        <x=2, y=-7, z=3>
        <x=9, y=-8, z=-3>
    ";

    #[test]
    fn test_day12_part1() {
        let mut moons = parse(INPUT1);
        assert_eq!(simulate(&mut moons, 10), 179);

        moons = parse(INPUT2);
        assert_eq!(simulate(&mut moons, 100), 1940);
    }

    #[test]
    fn test_day12_part2() {
        assert_eq!(part2(INPUT1), 2772);
        assert_eq!(part2(INPUT2), 4686774924);
    }
}
