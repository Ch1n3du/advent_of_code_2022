use advent_of_code_2022::solutions::day_05::part_1;

fn main() {
    // let src = std::fs::read_to_string("inputs/05_xxl.txt").unwrap();
    // println!("{:?}", part_1(&src));
    let path = "/a/b/c";
    println!("{:?}", path.split("/").collect::<Vec<&str>>())
}
