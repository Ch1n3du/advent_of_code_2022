use std::{collections::VecDeque, result};

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

        self.stacks
            .iter()
            .map(|stack| *stack.back().unwrap())
            .collect()
    }

    fn run_part_2(&mut self) -> Vec<char> {
        for &Move { amount, from, to } in self.moves.iter() {
            let mut to_be_moved = Vec::new();

            for _ in 0..amount {
                let value = self.stacks[from - 1].pop_back().unwrap();
                to_be_moved.push(value);
            }
            self.stacks[to - 1].extend(to_be_moved.into_iter().rev())
        }

        self.stacks
            .iter()
            .map(|stack| *stack.back().unwrap())
            .collect()
    }
}

fn parse(input: &str) -> Program {
    let mut lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();

    let no_of_stacks = (lines[0].len() + 1) / 4;
    let mut stacks: Vec<VecDeque<char>> = Vec::with_capacity(no_of_stacks);

    for _ in 0..no_of_stacks {
        stacks.push(VecDeque::new())
    }

    let mut stack_end = 0;

    for (i, line) in lines.iter().enumerate() {
        if !line.contains("[") {
            stack_end = i;
            break;
        }

        for index in 0..no_of_stacks {
            let chary = line.chars().nth((index * 4) + 1).unwrap();
            if chary != ' ' {
                stacks[index].push_front(chary);
            }
        }
    }

    let mut moves = Vec::new();
    for move_line in &lines[stack_end + 2..] {
        let parts: Vec<&str> = move_line.split(" ").collect();

        let amount = parts[1].parse::<usize>().unwrap();
        let from = parts[3].parse::<usize>().unwrap();
        let to = parts[5].parse::<usize>().unwrap();

        moves.push(Move { amount, from, to });
    }

    Program { stacks, moves }
}

fn part_1(input: &str) -> Vec<char> {
    let mut program = parse(input);
    program.run_part_1()
}

fn part_2(input: &str) -> Vec<char> {
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
        panic!("{res:?}")
    }
}
