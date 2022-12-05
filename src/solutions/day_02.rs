fn part_1(input: &str) -> u32 {
    let mut score = 0;

    for line in input.lines() {
        let mut chars: Vec<char> = line.chars().collect();

        score += match (chars[0], chars[2]) {
            // Rock Rock
            ('A', 'X') => 1 + 3,
            // Rock Paper
            ('A', 'Y') => 2 + 6,
            // Rock Scissors
            ('A', 'Z') => 3 + 0,

            // Paper Rock
            ('B', 'X') => 1 + 0,
            // Paper Paper
            ('B', 'Y') => 2 + 3,
            // Paper Scissors
            ('B', 'Z') => 3 + 6,

            // Scissors Rock
            ('C', 'X') => 1 + 6,
            // Scissors Paper
            ('C', 'Y') => 2 + 0,
            // Scissors Scissors
            ('C', 'Z') => 3 + 3,
            _ => 0,
        };
    }
    score
}

fn part_2(input: &str) -> u32 {
    let mut score = 0;

    for line in input.lines() {
        let mut chars: Vec<char> = line.chars().collect();

        score += match (chars[0], chars[2]) {
            // lose to rock so play scissors
            ('A', 'X') => 3 + 0,
            // Rock draw with rock so play rock
            ('A', 'Y') => 1 + 3,
            // Win rock so play paper
            ('A', 'Z') => 2 + 6,

            // lose to paper so play rock
            ('B', 'X') => 1 + 0,
            // draw with paper so play paper
            ('B', 'Y') => 2 + 3,
            // win paper so play scissors
            ('B', 'Z') => 3 + 6,

            // Scissors Rock
            ('C', 'X') => 2 + 0,
            // Scissors Paper
            ('C', 'Y') => 3 + 3,
            // Scissors Scissors
            ('C', 'Z') => 1 + 6,
            _ => 0,
        };
    }
    score
}

#[cfg(test)]
mod tests {
    #[test]
    fn day_2_part_1() {
        let res = crate::solve_file(2, 1, super::part_1);
    }

    #[test]
    fn day_2_part_2() {
        let res = crate::solve_file(2, 2, super::part_2);
    }
}
