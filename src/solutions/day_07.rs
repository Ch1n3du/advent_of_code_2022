use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Dirent {
    Dir(String),
    File { name: String, size: u64 },
}

#[derive(Debug, Clone)]
enum Command {
    Ls { children: Vec<Dirent> },
    Cd { path: String },
}

fn parse(input: &str) -> Vec<Command> {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut current = 0;
    let mut commands = Vec::new();

    while current < lines.len() {
        let current_line = lines[current];

        if current_line.starts_with("$ cd") {
            let path = current_line.split(" ").nth(2).unwrap().to_string();
            commands.push(Command::Cd { path })
        } else {
            // Ignore the ls
            current += 1;
            let mut children = Vec::new();

            while current < lines.len() && !lines[current].starts_with("$") {
                children.push(parse_dirent(&lines[current]));
                current += 1;
            }

            commands.push(Command::Ls { children });
            // Continue because you've already modified current.
            continue;
        }
        current += 1;
    }

    commands
}

fn parse_dirent(raw_line: &str) -> Dirent {
    if raw_line.starts_with("dir") {
        let name = raw_line.split(" ").nth(1).unwrap().to_string();
        Dirent::Dir(name)
    } else {
        let file_parts = raw_line.split(" ").collect::<Vec<&str>>();
        Dirent::File {
            name: file_parts[1].to_string(),
            size: file_parts[0].parse::<u64>().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Emulator {
    commands: Vec<Command>,
    curr_command: usize,
    curr_path: Vec<String>,
    directory_tree: HashMap<String, Vec<Dirent>>,
}

impl Emulator {
    fn new(input: &str) -> Emulator {
        Emulator {
            commands: parse(input),
            curr_command: 0,
            curr_path: Vec::new(),
            directory_tree: HashMap::new(),
        }
    }

    fn get_curr_path(&self) -> String {
        if self.curr_path.is_empty() {
            "/".to_string()
        } else {
            self.curr_path
                .iter()
                .fold("".to_string(), |acc, part| format!("{acc}/{part}"))
        }
    }

    fn is_at_end(&self) -> bool {
        self.curr_command > self.commands.len() - 1
    }

    fn execute_command(&mut self) {
        // println!(
        //     "\n{}{}{}",
        //     "=".repeat(14),
        //     self.curr_command,
        //     "=".repeat(14)
        // );
        // println!("Current Dir[Before]: {:?}", &self.curr_path);
        // println!("Dirs [Before]: {:?}", &self.directory_tree);
        // println!("Command: {:?}", &self.commands[self.curr_command]);
        match &self.commands[self.curr_command] {
            Command::Ls { children } => {
                let curr_path = self.get_curr_path();
                self.directory_tree.insert(curr_path, children.clone());
            }
            Command::Cd { path } => path.split("/").for_each(|part| match part {
                ".." => {
                    self.curr_path.pop();
                }
                "" => self.curr_path.truncate(0),
                dir_name => self.curr_path.push(dir_name.to_string()),
            }),
        }
        // println!("Current Dir[After]: {:?}", &self.curr_path);
        // println!("Dirs [After]: {:?}", &self.directory_tree);
        // println!("{}\n", "=".repeat(30));
    }

    fn execute_commands(&mut self) {
        while !self.is_at_end() {
            self.execute_command();
            self.curr_command += 1;
        }
    }

    fn format_path(root: &str, name: &str) -> String {
        if root == "/" {
            format!("/{name}")
        } else {
            format!("{root}/{name}")
        }
    }

    pub fn get_size_(&self, root: &str, depth: usize, acc: &mut Vec<u64>) -> u64 {
        let mut total = 0;
        if let Some(children) = self.directory_tree.get(root) {
            for dirent in children {
                match dirent {
                    Dirent::Dir(name) => {
                        let path = Emulator::format_path(root, name);
                        let size = self.get_size_(&path, depth + 1, acc);

                        total += size;
                        acc.push(size)
                        // println!("{}- {name}(dir, size={size})", "  ".repeat(depth));
                    }
                    Dirent::File { name, size } => {
                        total += *size;
                        // println!("{}- '{name}' (file) -> {size}", "  ".repeat(depth));
                    }
                }
            }
        }
        total
    }

    pub fn get_size(&self, root: &str, depth: usize) -> Vec<u64> {
        let mut acc = Vec::new();
        let res = self.get_size_("/", 0, &mut acc);

        acc.push(res);
        acc
    }
}

fn part_1(input: &str) -> u64 {
    let mut emu = Emulator::new(input);
    emu.execute_commands();

    emu.get_size("/", 0)
        .into_iter()
        .filter_map(|size| if size <= 100_000 { Some(size) } else { None })
        .sum()
}

fn part_2(input: &str) -> u64 {
    let mut emu = Emulator::new(input);
    emu.execute_commands();

    let mut dirs = emu.get_size("/", 0);
    dirs.sort();
    // println!("{:?}", dirs);
    let max = dirs.last().unwrap().clone();

    // println!("Empty: {}", 70_000_000 - max);

    for size in dirs {
        if max - size <= 40_000_000 {
            println!("{max} - {size} == {}", max - size);
            return size;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        let res = crate::solve_file(7, 1, super::part_1);
        assert_eq!(1743217, res)
    }

    #[test]
    fn test_part_2() {
        let res = crate::solve_file(7, 2, super::part_2);
        assert_eq!(8319096, res)
    }
}
