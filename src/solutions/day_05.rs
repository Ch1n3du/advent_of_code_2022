use std::collections::VecDeque;

#[derive(Debug)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}
#[derive(Debug)]
struct Program {
    stacks: Vec<VecDeque<char>>,
    moves: Vec<Move>,
}

impl Program {
    fn run_part_1(&mut self) -> Vec<char> {
        for &Move { amount, from, to } in self.moves.iter() {
            for _ in 0..amount {
                let value = self.stacks[from - 1].pop_back().unwrap();
                self.stacks[to - 1].push_back(value)
            }
        }

        // Get the stack tops
        self.stacks
            .iter()
            .map(|stack| *stack.back().unwrap())
            .collect()
    }

    fn run_part_2(&mut self) -> Vec<char> {
        for &Move { amount, from, to } in self.moves.iter() {
            // Create a Vector to store popped values that are to be reversed
            let mut to_be_moved = Vec::new();

            for _ in 0..amount {
                let value = self.stacks[from - 1].pop_back().unwrap();
                to_be_moved.push(value);
            }

            // Push the Vector reversed to the destination stack
            self.stacks[to - 1].extend(to_be_moved.into_iter().rev())
        }

        // Get the stack tops
        self.stacks
            .iter()
            .map(|stack| *stack.back().unwrap())
            .collect()
    }
}

fn parse(input: &str) -> Program {
    let mut lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();

    // Get the number of stacks by checking the length of the first line
    let no_of_stacks = (lines[0].len() + 1) / 4;
    let mut stacks: Vec<VecDeque<char>> = Vec::with_capacity(no_of_stacks);

    for _ in 0..no_of_stacks {
        stacks.push(VecDeque::new())
    }

    // Create a variable to store the end of stack section after parsing it.
    let mut stack_section_end = 0;

    for (i, line) in lines.iter().enumerate() {
        if !line.contains("[") {
            // Add two to the index to account for blank lines
            stack_section_end = i + 2;
            // println!("Stack Section End: {}", stack_section_end);
            break;
        }

        for index in 0..no_of_stacks {
            let chary = line.chars().nth((index * 4) + 1).unwrap();
            if chary != ' ' {
                stacks[index].push_front(chary);
            }
        }
    }

    // Parse the moves by splitting on space.
    let mut moves = Vec::new();
    for move_line in &lines[stack_section_end..] {
        let parts: Vec<&str> = move_line.split(" ").collect();

        let amount = parts[1].parse::<usize>().unwrap();
        let from = parts[3].parse::<usize>().unwrap();
        let to = parts[5].parse::<usize>().unwrap();

        moves.push(Move { amount, from, to });
    }

    Program { stacks, moves }
}

pub fn part_1(input: &str) -> Vec<char> {
    let mut program = parse(input);
    program.run_part_1()
}

pub fn part_2(input: &str) -> Vec<char> {
    let mut program = parse(input);
    program.run_part_2()
}

#[cfg(test)]
mod tests {
    #[test]
    fn can_parse() {
        let src = std::fs::read_to_string("inputs/05.txt").unwrap();
        let program = super::parse(src.as_str());
        // panic!("{program:?}")
    }

    #[test]
    fn test_part_1() {
        let res = crate::solve_file(5, 1, super::part_1);
        // panic!("{res:?}")
    }

    #[test]
    fn test_part_2() {
        let res = crate::solve_file(5, 2, super::part_2);
        // panic!("{res:?}")
    }
}
