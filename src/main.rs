fn main() {
    let mut day = 1;
    let input = adventofcode2019::read_input(day);
    println!(
        "Day {} part 1: {}",
        day,
        adventofcode2019::day1::part1(&input)
    );
    println!(
        "Day {} part 2: {}",
        day,
        adventofcode2019::day1::part2(&input)
    );

    day = 2;
    let input = adventofcode2019::read_input(day);
    println!(
        "Day {} part 1: {}",
        day,
        adventofcode2019::day2::part1(&input)
    );
    println!(
        "Day {} part 2: {}",
        day,
        adventofcode2019::day2::part2(&input)
    );

    day = 3;
    let input = adventofcode2019::read_input(day);
    println!(
        "Day {} part 1: {}",
        day,
        adventofcode2019::day3::part1(&input)
    );
    println!(
        "Day {} part 2: {}",
        day,
        adventofcode2019::day3::part2(&input)
    );

    day = 4;
    let input = adventofcode2019::read_input(day);
    println!(
        "Day {} part 1: {}",
        day,
        adventofcode2019::day4::part1(&input)
    );
    println!(
        "Day {} part 2: {}",
        day,
        adventofcode2019::day4::part2(&input)
    );

    day = 5;
    let input = adventofcode2019::read_input(day);
    println!(
        "Day {} part 1: {}",
        day,
        adventofcode2019::day5::part1(&input)
    );
    println!(
        "Day {} part 2: {}",
        day,
        adventofcode2019::day5::part2(&input)
    );

    day = 6;
    let input = adventofcode2019::read_input(day);
    println!(
        "Day {} part 1: {}",
        day,
        adventofcode2019::day6::part1(&input)
    );
    println!(
        "Day {} part 2: {}",
        day,
        adventofcode2019::day6::part2(&input)
    );

    day = 7;
    let input = adventofcode2019::read_input(day);
    println!(
        "Day {} part 1: {}",
        day,
        adventofcode2019::day7::part1(&input)
    );
    println!(
        "Day {} part 2: {}",
        day,
        adventofcode2019::day7::part2(&input)
    );

    day = 8;
    let input = adventofcode2019::read_input(day);
    println!(
        "Day {} part 1: {}",
        day,
        adventofcode2019::day8::part1(&input)
    );
    println!(
        "Day {} part 2:\n{}",
        day,
        adventofcode2019::day8::part2(&input)
    );

    day = 9;
    let input = adventofcode2019::read_input(day);
    println!(
        "Day {} part 1: {}",
        day,
        adventofcode2019::day9::part1(&input)
    );
    println!(
        "Day {} part 2: {}",
        day,
        adventofcode2019::day9::part2(&input)
    );
}
