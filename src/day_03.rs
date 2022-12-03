use std::collections::HashSet;

fn get_priority(chary: char) -> u32 {
    match chary {
        c if c.is_ascii_lowercase() => (c as u32) - 96,
        c => 26 + ((c as u32) - 64),
    }
}

pub fn part_1(input: &str) -> u32 {
    let mut total_priority: u32 = 0;
    for line in input.lines() {
        let total_items: Vec<u32> = line.chars().map(get_priority).collect();
        let halfway = total_items.len() / 2;

        let left_items: HashSet<u32> = HashSet::from_iter(total_items[..halfway].iter().cloned());
        let right_items: HashSet<u32> = HashSet::from_iter(total_items[halfway..].iter().cloned());

        total_priority += left_items.intersection(&right_items).sum::<u32>();
    }

    total_priority
}

pub fn part_2(input: &str) -> u32 {
    let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    let mut total_priority = 0;

    for index in (0..lines.len()).step_by(3) {
        let (elf_a, elf_b, elf_c): (HashSet<char>, HashSet<char>, HashSet<char>) = (
            lines[index].chars().collect(),
            lines[index + 1].chars().collect(),
            lines[index + 2].chars().collect(),
        );

        for item in elf_a {
            if elf_b.contains(&item) && elf_c.contains(&item) {
                println!("Start: {index}");
                println!("Common: '{item}'");
                println!("Priority: '{}'", get_priority(item));
                total_priority += get_priority(item);
                println!("Total: {total_priority}");
            }
        }
    }
    total_priority
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        let res = crate::solve_file("03", super::part_1);
        panic!("Day 3 part 1: {res}")
    }

    #[test]
    fn part_2() {
        let res = crate::solve_file("03", super::part_2);
        panic!("Day 3 part 2: {res}")
    }
}
