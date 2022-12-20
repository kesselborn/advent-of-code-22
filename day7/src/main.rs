use env_logger;
use log::{debug, LevelFilter};
use std::fmt::{Display, Formatter};
use std::io::Read;
use std::str::Split;
use std::{env, fs};

use anyhow::{bail, Context, Result};

#[derive(Clone)]
struct File {
    name: String,
    size: u32,
}

#[derive(Clone)]
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
    pub fn new(name: &str) -> Self {
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

    pub fn new_fs() -> Self {
        let mut fs = Dir::new("");
        fs.dirs.push(Dir::new("/"));

        fs
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

    fn parse_session_part(line_iterator: &mut Split<&str>, current_dir: &mut Dir) -> Result<()> {
        loop {
            if let Some(line) = line_iterator.next() {
                if line == "" {
                    continue;
                }
                match Self::parse_line(line) {
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

                        if let Some(target_dir) = current_dir
                            .dirs
                            .iter_mut()
                            .find(|dir| dir.name == cd_command.target)
                        {
                            Self::parse_session_part(line_iterator, target_dir)?;
                        } else {
                            bail!("trying to cd into a non existing directory with name '{}' -- available subdirs: {:?}", cd_command.target, current_dir.dirs.iter().map(|dir| &dir.name).collect::<Vec<_>>())
                        }
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

    pub fn parse_session(session: &str) -> Result<Self> {
        let mut line_iterator = session.split("\n");
        let mut fs = Dir::new_fs();
        Self::parse_session_part(&mut line_iterator, &mut fs)?;

        Ok(fs)
    }

    pub fn find_dir(&mut self, name: &str) -> Result<&mut Self> {
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

    pub fn total_size(&self) -> u32 {
        let mut size = 0;
        for dir in self.dirs.iter() {
            size += dir.total_size();
        }

        size + self.files.iter().fold(0, |acc: u32, file| acc + file.size)
    }

    fn dir_sizes(&self, sums: &mut Vec<(String, u32)>) {
        for dir in self.dirs.iter() {
            sums.push((dir.clone().name, dir.total_size()));
            dir.dir_sizes(sums);
        }
    }

    pub fn total_sum_of_all_dirs_smaller_then(&self, max_size: u32) -> u32 {
        let mut dir_sizes: Vec<(String, u32)> = vec![];
        self.dir_sizes(&mut dir_sizes);

        debug!("{:?}", dir_sizes);
        debug!(
            "{:?}",
            dir_sizes
                .iter()
                .filter(|x| (**x).1 <= max_size)
                .collect::<Vec<_>>()
        );
        dir_sizes
            .iter()
            .filter(|x| (**x).1 <= max_size)
            .map(|tuple| tuple.1)
            .sum()
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

    let mut log_builder = env_logger::builder();

    if let Ok(debug) = env::var("DEBUG") {
        if debug == "1" {
            log_builder.filter_module(module_path!(), LevelFilter::Debug);
        }
    }
    log_builder.init();

    let mut input = String::new();
    let _ = file.read_to_string(&mut input)?;

    let fs = Dir::parse_session(&input)?;
    println!("part1: {}", fs.total_sum_of_all_dirs_smaller_then(100_000));
    // println!("part2: {}", find_marker_pos(&input, 14).unwrap());

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{Dir, ParseResult};

    #[test]
    fn parse_cd_command() {
        let r = super::Dir::parse_line("$ cd xxx");

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
        let r = super::Dir::parse_line("$ ls");

        assert!(r.is_ok());
        assert!(matches!(
            r.unwrap(),
            ParseResult::Command(super::Command::Ls)
        ));
    }

    #[test]
    fn parse_dir_line() {
        let r = super::Dir::parse_line("dir svgbqd");

        assert!(r.is_ok());
        assert!(matches!(r.as_ref().unwrap(), ParseResult::Dirname(_)));

        if let Ok(ParseResult::Dirname(dir_name)) = r {
            assert_eq!(dir_name, "svgbqd".to_owned())
        }
    }

    #[test]
    fn parse_file_line() {
        let r = super::Dir::parse_line("8033020 d.log");

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

        let fs = Dir::parse_session(session);
        assert!(fs.is_ok(), "error was: {:?}", fs.err());

        let mut fs = fs.unwrap();

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

        assert_eq!(fs.total_sum_of_all_dirs_smaller_then(100_000), 95437);
    }
}
