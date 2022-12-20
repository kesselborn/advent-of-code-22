use std::fmt::{Display, Formatter};
use std::fs;
use std::io::Read;
use std::str::Split;

use anyhow::{bail, Context, Result};

struct File {
    name: String,
    size: u32,
}

struct Dir {
    name: String,
    dirs: Box<Vec<Dir>>,
    files: Box<Vec<File>>,
}

impl Display for Dir {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.indented_fmt(f, "")?;
        writeln!(f, "")
    }
}

impl Dir {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            dirs: Box::new(vec![]),
            files: Box::new(vec![]),
        }
    }

    fn indented_fmt(&self, f: &mut Formatter<'_>, prefix: &str) -> std::fmt::Result {
        for file in self.files.iter() {
            writeln!(f, "{}├──{} {}", prefix, file.name, file.size)?;
        }
        for dir in self.dirs.iter() {
            writeln!(f, "{}├─{}", prefix, dir.name)?;
            dir.indented_fmt(f, &format!("{}  ", prefix))?;
        }
        write!(f, "")
    }

    fn new_fs() -> Self {
        let mut fs = Dir::new("");
        fs.dirs.push(Dir::new("/"));

        fs
    }

    fn find_dir(&mut self, name: &str) -> Result<&mut Self> {
        if self.name == name {
            return Ok(self);
        }

        for dir in self.dirs.iter_mut() {
            if let Ok(dir) = dir.find_dir(name) {
                return Ok(dir);
            }
        }

        bail!(
            "no directory with name '{}' found under directory '{}'",
            name,
            self.name
        )
    }

    fn total_size(&self) -> u32 {
        let mut size = 0;
        for dir in self.dirs.iter() {
            size += dir.total_size();
        }

        size + self.files.iter().fold(0, |acc: u32, file| acc + file.size)
    }
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

fn parse_session(line_iterator: &mut Split<&str>, current_dir: &mut Dir) -> Result<()> {
    loop {
        if let Some(line) = line_iterator.next() {
            match parse_line(line) {
                Ok(ParseResult::Command(Command::Ls)) => {}
                Ok(ParseResult::File(file)) => current_dir.files.push(File {
                    name: file.name,
                    size: file.size,
                }),
                Ok(ParseResult::Dirname(dirname)) => {
                    let dir = Dir::new(&dirname);
                    current_dir.dirs.push(dir);
                }
                Ok(ParseResult::Command(Command::Cd(cd_command))) => {
                    if cd_command.target == ".." {
                        return Ok(());
                    }

                    let target_dir = current_dir.find_dir(&cd_command.target)?;
                    parse_session(line_iterator, target_dir)?;
                }
                Err(e) => {
                    bail!("error parsing session: {}", e)
                }
            }
        } else {
            return Ok(());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_line, Dir, ParseResult};
    use std::fmt::format;

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
    fn parse_dir_line() {
        let r = parse_line("dir svgbqd");

        assert!(r.is_ok());
        assert!(matches!(r.as_ref().unwrap(), ParseResult::Dirname(_)));

        if let Ok(ParseResult::Dirname(dir_name)) = r {
            assert_eq!(dir_name, "svgbqd".to_owned())
        }
    }

    #[test]
    fn parse_file_line() {
        let r = parse_line("8033020 d.log");

        assert!(r.is_ok());
        assert!(matches!(r.as_ref().unwrap(), ParseResult::File(_)));

        if let Ok(ParseResult::File(file)) = r {
            assert_eq!(file.name, "d.log".to_owned());
            assert_eq!(file.size, 8033020);
        }
    }

    #[test]
    fn test_find_dir() {
        let mut fs = Dir::new("");
        fs.dirs.push(Dir::new("a"));

        let a_dir = fs.find_dir("a");
        assert!(a_dir.is_ok());
        assert_eq!(a_dir.as_ref().unwrap().name, "a");

        if let Ok(a_dir) = a_dir {
            a_dir.dirs.push(Dir::new("b"));
            a_dir.dirs.push(Dir::new("c"))
        }

        let b_dir = fs.find_dir("b");
        assert!(b_dir.is_ok());
        assert_eq!(b_dir.as_ref().unwrap().name, "b");

        if let Ok(b_dir) = b_dir {
            b_dir.dirs.push(Dir::new("d"))
        }

        let b_dir = fs.find_dir("c");
        assert!(b_dir.is_ok());
        assert_eq!(b_dir.as_ref().unwrap().name, "c");

        let b_dir = fs.find_dir("d");
        assert!(b_dir.is_ok());
        assert_eq!(b_dir.as_ref().unwrap().name, "d");
    }

    #[test]
    fn parse_session() {
        let session = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

        let mut line_iterator = session.split("\n");
        let mut fs = Dir::new_fs();
        let x = super::parse_session(&mut line_iterator, &mut fs);

        assert!(x.is_ok(), "error was: {:?}", x.err());

        let e_dir = fs.find_dir("e");

        assert!(e_dir.is_ok());
        assert_eq!(&e_dir.as_ref().unwrap().name, "e");
        assert_eq!(e_dir.as_ref().unwrap().total_size(), 584);

        let a_dir = fs.find_dir("a");
        assert!(a_dir.is_ok());
        assert_eq!(a_dir.as_ref().unwrap().total_size(), 94853);

        let root_dir = fs.find_dir("/");
        assert!(root_dir.is_ok());
        assert_eq!(root_dir.as_ref().unwrap().total_size(), 48381165);
    }
}
