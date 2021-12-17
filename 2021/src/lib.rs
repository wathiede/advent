pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
//pub mod day14;
pub mod day15;
pub mod day16;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

use aoc_runner_derive::aoc_lib;

#[macro_export]
macro_rules! debug_print{
    ($($arg:tt)*) => (#[cfg(debug_assertions)] print!($($arg)*));
}

#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => (#[cfg(debug_assertions)] println!($($arg)*));
}

aoc_lib! { year = 2021 }
