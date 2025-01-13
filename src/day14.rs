use std::collections::HashMap;

#[derive(Debug)]
struct Chemical {
    amount: i64,
    ingredients: Vec<(String, i64)>,
}

impl Chemical {
    fn is_primitive(&self) -> bool {
        return self.ingredients.len() == 1 && self.ingredients[0].0 == "ORE";
    }

    fn disassemble(&self, data: &HashMap<String, Chemical>) -> HashMap<String, i64> {
        self.ingredients
            .iter()
            .fold(HashMap::new(), |mut acc, (name, amount)| {
                let chemical = data.get(name).unwrap();
                if !chemical.is_primitive() {
                    chemical
                        .disassemble(data)
                        .into_iter()
                        .for_each(|(key, value)| {
                            *acc.entry(key).or_default() += amount * value;
                        });
                } else {
                    *acc.entry(name.clone()).or_default() += amount;
                }
                acc
            })
    }

    fn count(&self, amount: i64) -> (HashMap<String, i64>, i64) {
        let count = amount / self.amount + if amount % self.amount > 0 { 1 } else { 0 };
        let ingredients =
            self.ingredients
                .iter()
                .fold(HashMap::new(), |mut acc, (name, amount)| {
                    acc.insert(name.clone(), amount * count);
                    acc
                });
        (ingredients, count * self.amount - amount)
    }
}

fn parse_chemical(input: &str) -> (String, i64) {
    let parts = input.split(" ").collect::<Vec<_>>();
    (parts[1].to_string(), parts[0].parse().unwrap())
}

fn parse(input: &str) -> HashMap<String, Chemical> {
    input.lines().fold(
        HashMap::new(),
        |mut acc: HashMap<String, Chemical>, line| {
            let parts = line.trim().split(" => ").collect::<Vec<_>>();
            let (chemical, amount) = parse_chemical(parts[1]);
            acc.insert(
                chemical,
                Chemical {
                    amount,
                    ingredients: parts[0].split(", ").map(parse_chemical).collect(),
                },
            );
            acc
        },
    )
}

fn sort(chemicals: &HashMap<String, Chemical>) -> Vec<String> {
    let mut result = vec![];
    let mut vertices: HashMap<String, i64> = HashMap::new();
    vertices.insert("FUEL".to_string(), 0);
    let mut stack = vec!["FUEL".to_string()];
    while let Some(v) = stack.last() {
        let color = vertices.entry(v.to_string()).or_insert(0);
        if *color == 0 {
            *color = 1;
            if let Some(c) = chemicals.get(v) {
                for target in &c.ingredients {
                    stack.push(target.0.clone());
                }
            }
        } else if *color == 1 {
            result.push(v.to_string());
            *color = 2;
            stack.pop();
        } else {
            stack.pop();
        }
    }
    result
}

fn solve(chemicals: &HashMap<String, Chemical>, amount: i64) -> i64 {
    let mut sorted = sort(&chemicals);
    let mut lefts: HashMap<String, i64> = HashMap::new();
    let mut needed: HashMap<String, i64> = HashMap::new();
    needed.insert("FUEL".to_string(), amount);
    while let Some(name) = sorted.pop() {
        if let Some(chemical) = chemicals.get(&name) {
            let (used, left) =
                chemical.count(needed.get(&name).unwrap() - lefts.get(&name).unwrap_or(&0));
            for (used_ingr, used_amount) in used {
                *needed.entry(used_ingr).or_default() += used_amount;
            }
            lefts.insert(name, left);
        } else {
            break;
        }
    }
    *needed.get("ORE").unwrap()
}

pub fn part1(input: &str) -> i64 {
    let chemicals = parse(input);
    solve(&chemicals, 1)
}

pub fn part2(input: &str) -> i64 {
    let chemicals = parse(input);
    let mut low = 1;
    let mut high = 1000000000000;
    while low < high - 1 {
        let mid = (low + high) / 2;
        if solve(&chemicals, mid) > 1000000000000 {
            high = mid;
        } else {
            low = mid;
        }
    }

    if solve(&chemicals, high) <= 1000000000000 {
        high
    } else {
        low
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "10 ORE => 10 A
        1 ORE => 1 B
        7 A, 1 B => 1 C
        7 A, 1 C => 1 D
        7 A, 1 D => 1 E
        7 A, 1 E => 1 FUEL";

    const INPUT2: &str = "9 ORE => 2 A
        8 ORE => 3 B
        7 ORE => 5 C
        3 A, 4 B => 1 AB
        5 B, 7 C => 1 BC
        4 C, 1 A => 1 CA
        2 AB, 3 BC, 4 CA => 1 FUEL";

    const INPUT3: &str = "157 ORE => 5 NZVS
        165 ORE => 6 DCFZ
        44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
        12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
        179 ORE => 7 PSHF
        177 ORE => 5 HKGWZ
        7 DCFZ, 7 PSHF => 2 XJWVT
        165 ORE => 2 GPVTF
        3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";

    const INPUT4: &str = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
        17 NVRVD, 3 JNWZP => 8 VPVL
        53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
        22 VJHF, 37 MNCFX => 5 FWMGM
        139 ORE => 4 NVRVD
        144 ORE => 7 JNWZP
        5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
        5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
        145 ORE => 6 MNCFX
        1 NVRVD => 8 CXFTF
        1 VJHF, 6 MNCFX => 4 RFSQX
        176 ORE => 6 VJHF";

    const INPUT5: &str = "171 ORE => 8 CNZTR
        7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
        114 ORE => 4 BHXH
        14 VRPVC => 6 BMBT
        6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
        6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
        15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
        13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
        5 BMBT => 4 WPTQ
        189 ORE => 9 KTJDG
        1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
        12 VRPVC, 27 CNZTR => 2 XDBXC
        15 KTJDG, 12 BHXH => 5 XCVML
        3 BHXH, 2 VRPVC => 7 MZWV
        121 ORE => 7 VRPVC
        7 XCVML => 6 RJRHP
        5 BHXH, 4 VRPVC => 5 LTCX";

    #[test]
    fn test_day14_part1() {
        assert_eq!(part1(INPUT1), 31);
        assert_eq!(part1(INPUT2), 165);
        assert_eq!(part1(INPUT3), 13312);
        assert_eq!(part1(INPUT4), 180697);
        assert_eq!(part1(INPUT5), 2210736);
    }

    #[test]
    fn test_day14_part2() {
        assert_eq!(part2(INPUT3), 82892753);
        assert_eq!(part2(INPUT4), 5586022);
        assert_eq!(part2(INPUT5), 460664);
    }
}
