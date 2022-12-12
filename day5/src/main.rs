use crate::CrateMoverType::{CrateMover9000, CrateMover9001};
use anyhow::{bail, Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader, Seek};
use std::str::FromStr;

#[derive(Debug)]
struct Stacks(Vec<Vec<char>>);

impl Stacks {
    fn new() -> Stacks {
        Stacks(vec![])
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn push(&mut self, line: &str) -> Result<()> {
        // we rely on the given format: all lines have the same length, even if the
        // current line does not have an item on the last stack, i.e. it's:
        // "    [D]    ", not "    [D]" if we have three stacks
        // In general, the format is: one crate is four characters long, the latter is at index 1
        // "[W] [B] [T] [F] [L] [T] [M] [F] [T]"
        // "[T]             [P]     [J]        "
        //   ^   ^   ^   ^   ^   ^   ^   ^   ^
        let num_of_chars_per_crate = 4;
        let num_of_crates = line.len() / 4;

        for i in 0..num_of_crates + 1 {
            let current_crate = line.chars().nth((i * num_of_chars_per_crate) + 1).unwrap();

            if self.len() <= i {
                self.0.push(vec![]);
            }

            if current_crate == ' ' {
                continue;
            }

            self.0[i].insert(0, current_crate);
        }
        Ok(())
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct MoveCommand {
    from: usize,
    to: usize,
    count: usize,
}

enum CrateMoverType {
    CrateMover9000,
    CrateMover9001,
}

#[derive(Debug)]
struct Program(Vec<MoveCommand>);

impl Program {
    fn new() -> Self {
        Program(vec![])
    }

    fn add_command(&mut self, command: MoveCommand) {
        self.0.resize(self.0.len() + 1, command);
    }

    fn execute(&self, stacks: &mut Stacks, mover: CrateMoverType) -> String {
        let mut result = String::from("");
        for command in &self.0 {
            let stack = &mut stacks.0[(command.from - 1) as usize];
            let length = stack.len();
            let crates: &mut Vec<char> =
                &mut stack.drain(length - command.count..=length - 1).collect();

            if let CrateMover9000 = mover {
                crates.reverse();
            }

            stacks.0[(command.to - 1) as usize].append(crates);
        }
        for stack in &stacks.0 {
            result.push(*stack.last().unwrap());
        }

        result
    }
}

impl MoveCommand {
    // commands are like: "move 3 from 8 to 2"
    fn from_str(s: &str) -> Result<MoveCommand> {
        let tokens = s.split(' ').collect::<Vec<&str>>();

        match (tokens[0], tokens[2], tokens[4]) {
            ("move", "from", "to") => {
                let count = u32::from_str(tokens[1])?;
                let from = u32::from_str(tokens[3])?;
                let to = u32::from_str(tokens[5])?;
                Ok(MoveCommand {
                    from: from as usize,
                    to: to as usize,
                    count: count as usize,
                })
            }
            _ => {
                bail!("error parsing move command: got line '{}'", s)
            }
        }
    }
}

fn parse_input<T>(reader: BufReader<T>) -> Result<(Stacks, Program)>
where
    T: std::io::Read,
{
    let mut stacks = Stacks::new();
    let mut program = Program::new();

    for line in reader.lines() {
        let line = line.unwrap();

        if line.len() == 0 {
            continue;
        }

        if line.chars().nth(0).unwrap() == '[' {
            stacks.push(&line)?;
        }

        if &line[0..4].to_string() == "move" {
            let _ = &program.add_command(MoveCommand::from_str(&line)?);
        }
    }

    Ok((stacks, program))
}

fn main() -> Result<()> {
    let file_name = "input";
    let mut file =
        File::open(&file_name).context(format!("while opening file '{}'", &file_name))?;

    let (mut stacks, program) = parse_input(BufReader::new(&file))?;

    println!("part1: {}", program.execute(&mut stacks, CrateMover9000));

    file.rewind()?;
    let (mut stacks, program) = parse_input(BufReader::new(&file))?;
    println!("part2: {}", program.execute(&mut stacks, CrateMover9001));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::MoveCommand;
    use std::io::BufReader;

    #[test]
    fn push_to_new_stack() {
        let mut stacks = super::Stacks::new();

        assert_eq!(stacks.len(), 0);

        let result = &stacks.push("    [D]    ");
        assert!(result.is_ok());

        assert_eq!(stacks.0[0].len(), 0);
        assert_eq!(stacks.len(), 3);
        assert_eq!(stacks.0[1][0], 'D');
    }

    #[test]
    fn push_to_new_stack2() {
        let mut stacks = super::Stacks::new();

        assert_eq!(stacks.len(), 0);

        let result = &stacks.push("    [D]    ");
        assert!(result.is_ok());
        let result = &stacks.push("[N] [C]    ");
        assert!(result.is_ok());
        let result = &stacks.push("[Z] [M] [P]");
        assert!(result.is_ok());

        assert_eq!(stacks.0[0].len(), 2);
        assert_eq!(stacks.0[1].len(), 3);
        assert_eq!(stacks.0[2].len(), 1);
        assert_eq!(stacks.len(), 3);
        assert_eq!(stacks.0[1][1], 'C');
        assert_eq!(stacks.0[1][0], 'M');
    }

    #[test]
    fn parse_move_command() {
        let cmd = MoveCommand::from_str("move 3 from 8 to 12");

        assert!(cmd.is_ok());
        assert_eq!(
            cmd.unwrap(),
            MoveCommand {
                from: 8,
                to: 12,
                count: 3
            }
        )
    }

    #[test]
    fn test_parse_input() {
        let reader = BufReader::new(
            r#"[T]             [P]     [J]
[F]     [S]     [T]     [R]     [B]
[V]     [M] [H] [S]     [F]     [R]
[Z]     [P] [Q] [B]     [S] [W] [P]
[C]     [Q] [R] [D] [Z] [N] [H] [Q]
[W] [B] [T] [F] [L] [T] [M] [F] [T]
[S] [R] [Z] [V] [G] [R] [Q] [N] [Z]
[Q] [Q] [B] [D] [J] [W] [H] [R] [J]
 1   2   3   4   5   6   7   8   9

move 3 from 8 to 2
move 3 from 1 to 5
move 3 from 1 to 4
move 2 from 7 to 4
move 3 from 7 to 4
move 8 from 5 to 7
move 2 from 1 to 8
"#
            .as_bytes(),
        );

        let input_result = super::parse_input(reader);

        assert!(input_result.is_ok());

        let (stacks, program) = input_result.unwrap();

        assert_eq!(stacks.0[0].len(), 8);
        assert_eq!(stacks.0[1].len(), 3);
        assert_eq!(stacks.len(), 9);
        assert_eq!(program.0.len(), 7)
    }
}
