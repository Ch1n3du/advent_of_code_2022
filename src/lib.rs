use std::fmt::Debug;

pub mod solutions;

pub fn solve_file<T: Debug>(day: u8, part: u8, solution: fn(&str) -> T) -> T {
    let path = format!("inputs/{day:02}.txt");
    let input = std::fs::read_to_string(path).unwrap();
    let res = solution(&input);

    println!("Day {day:02}, Part {part:02} Solution: {res:?}");
    res
}
