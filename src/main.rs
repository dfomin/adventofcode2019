fn main() {
    let day = 1;
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
}
