use std::fs;
use std::io::Read;

use anyhow::{bail, Context, Result};

struct File {
    name: String,
    size: u32,
}

struct Dir {
    dirs: Vec<Dir>,
    files: Vec<File>,
}

struct CdCommand {
    target: String,
}

enum Command {
    Cd(CdCommand),
    Ls,
}

enum ParseResult {
    Dirname(String),
    File(File),
    Command(Command),
}

fn main() -> Result<()> {
    let file_name = "input";
    let mut file =
        fs::File::open(&file_name).context(format!("while opening file '{}'", &file_name))?;

    let mut input = String::new();
    let _ = file.read_to_string(&mut input)?;

    println!("part1: {}", input);
    // println!("part2: {}", find_marker_pos(&input, 14).unwrap());

    Ok(())
}

fn parse_line(input: &str) -> Result<ParseResult> {
    match input.split(" ").collect::<Vec<&str>>().as_slice() {
        ["$", "ls"] => Ok(ParseResult::Command(Command::Ls)),
        ["$", "cd", target] => Ok(ParseResult::Command(Command::Cd(CdCommand {
            target: target.to_string(),
        }))),
        ["dir", name] => Ok(ParseResult::Dirname(name.to_string())),
        [size_str, file_name] => {
            let size: u32 = size_str
                .parse()
                .context(format!("error parsing file size from line '{}'", input))?;
            Ok(ParseResult::File(File {
                name: file_name.to_string(),
                size,
            }))
        }
        _ => bail!("error: can't parse the line '{}'", input),
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_line, ParseResult};

    #[test]
    fn parse_cd_command() {
        let r = parse_line("$ cd xxx");

        assert!(r.is_ok());
        assert!(matches!(
            r.as_ref().unwrap(),
            ParseResult::Command(super::Command::Cd(_))
        ));

        if let Ok(ParseResult::Command(super::Command::Cd(cd_command))) = r {
            assert_eq!(cd_command.target, "xxx".to_owned())
        } else {
            assert!(false)
        }
    }

    #[test]
    fn parse_ls_command() {
        let r = parse_line("$ ls");

        assert!(r.is_ok());
        assert!(matches!(
            r.unwrap(),
            ParseResult::Command(super::Command::Ls)
        ));
    }

    #[test]
    fn parse_dir() {
        let r = parse_line("dir svgbqd");

        assert!(r.is_ok());
        assert!(matches!(r.as_ref().unwrap(), ParseResult::Dirname(_)));

        if let Ok(ParseResult::Dirname(dir_name)) = r {
            assert_eq!(dir_name, "svgbqd".to_owned())
        }
    }

    #[test]
    fn parse_file() {
        let r = parse_line("8033020 d.log");

        assert!(r.is_ok());
        assert!(matches!(r.as_ref().unwrap(), ParseResult::File(_)));

        if let Ok(ParseResult::File(file)) = r {
            assert_eq!(file.name, "d.log".to_owned());
            assert_eq!(file.size, 8033020);
        }
    }
}
