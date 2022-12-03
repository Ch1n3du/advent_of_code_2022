pub mod day_01;
pub mod day_02;
pub mod day_03;

pub fn solve_file<T>(day: &str, solution: fn(&str) -> T) -> T {
    let path = format!("inputs/day_{day}.txt");
    let input = std::fs::read_to_string(path).unwrap();
    solution(&input)
}
