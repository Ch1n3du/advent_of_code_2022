fn contains(container: (u32, u32), contained: (u32, u32)) -> bool {
    (container.0 <= contained.0 && contained.0 <= container.1)
        && (container.1 >= contained.1 && contained.1 >= container.0)
}

pub fn part_1(input: &str) -> u32 {
    input
        .lines()
        .filter(|line| {
            let values: Vec<u32> = line
                .split(|c| c == '-' || c == ',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect();
            let (first, second) = ((values[0], values[1]), (values[2], values[3]));

            contains(first, second) || contains(second, first)
        })
        .count() as u32
}

fn overlaps(first: (u32, u32), second: (u32, u32)) -> bool {
    (first.0 <= second.0 && second.0 <= first.1) || (second.0 <= first.0 && first.0 <= second.1)
}

pub fn part_2(input: &str) -> u32 {
    input
        .lines()
        .filter(|line| {
            let values: Vec<u32> = line
                .split(|c| c == '-' || c == ',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect();
            let (first, second) = ((values[0], values[1]), (values[2], values[3]));

            overlaps(first, second)
        })
        .count() as u32
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_part_1() {
        let res = crate::solve_file(4, 1, super::part_1);
        assert_eq!(res, 507, "Iono");
    }

    #[test]
    fn test_part_2() {
        let res = crate::solve_file(4, 2, super::part_2);
        assert_eq!(res, 897, "Iono");
    }
}
