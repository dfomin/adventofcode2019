use std::fs;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;

pub fn read_input(day: i32) -> String {
    fs::read_to_string(format!("inputs/day{}.txt", day)).unwrap()
}
