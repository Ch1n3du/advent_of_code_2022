use std::collections::{HashSet, VecDeque};

fn all_unique(queue: &VecDeque<u8>) -> bool {
    let mut checked = HashSet::<u8>::with_capacity(queue.len());

    for chary in queue {
        if checked.contains(chary) {
            return false;
        } else {
            checked.insert(chary.clone());
        }
    }

    true
}

fn solution(input: &str, width: usize) -> u32 {
    let mut chars = input.bytes();

    let mut queue: VecDeque<u8> = VecDeque::with_capacity(width);

    for _ in 0..width {
        queue.push_back(chars.next().unwrap())
    }

    if all_unique(&queue) {
        return 4;
    }

    for (i, chary) in chars.enumerate() {
        queue.pop_front();
        queue.push_back(chary);

        if all_unique(&queue) {
            return (width + 1 + i) as u32;
        }
    }
    0
}

pub fn part_1(input: &str) -> u32 {
    solution(input, 4)
}

pub fn part_2(input: &str) -> u32 {
    solution(input, 14)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        let res = crate::solve_file(6, 1, super::part_1);
        assert_eq!(res, 1760)
    }

    #[test]
    fn test_part_2() {
        let res = crate::solve_file(6, 2, super::part_2);
        assert_eq!(res, 2974)
    }
}
