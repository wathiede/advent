pub mod day1;

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
