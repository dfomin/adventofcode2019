use std::fs;

pub mod day1;
pub mod day2;

pub fn read_input(day: i32) -> String {
    fs::read_to_string(format!("inputs/day{}.txt", day)).unwrap()
}
