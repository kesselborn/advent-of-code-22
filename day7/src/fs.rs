use anyhow::{bail, Result};
use log::debug;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct File {
    pub name: String,
    pub size: usize,
}

#[derive(Clone)]
pub struct Dir {
    pub(crate) name: String,
    pub(crate) dirs: Vec<Dir>,
    pub(crate) files: Vec<File>,
}

impl Display for Dir {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.indented_fmt(f, "")?;
        writeln!(f)
    }
}

impl Dir {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            dirs: vec![],
            files: vec![],
        }
    }

    fn indented_fmt(&self, f: &mut Formatter<'_>, prefix: &str) -> std::fmt::Result {
        for file in self.files.iter() {
            writeln!(f, "{}├──{} {}", prefix, file.name, file.size)?;
        }
        for dir in self.dirs.iter() {
            writeln!(f, "{}├─{}", prefix, dir.name)?;
            dir.indented_fmt(f, &format!("{prefix}  "))?;
        }
        write!(f, "")
    }

    pub fn new_fs() -> Self {
        let mut fs = Dir::new("");
        fs.dirs.push(Dir::new("/"));

        fs
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

    pub fn total_size(&self) -> usize {
        let mut size = 0;
        for dir in self.dirs.iter() {
            size += dir.total_size();
        }

        size + self
            .files
            .iter()
            .fold(0, |acc: usize, file| acc + file.size)
    }

    fn dir_sizes(&self, sums: &mut Vec<(String, usize)>) {
        for dir in self.dirs.iter() {
            sums.push((dir.clone().name, dir.total_size()));
            dir.dir_sizes(sums);
        }
    }

    pub fn dirs_smaller_than(&self, max_size: usize) -> Vec<(String, usize)> {
        let mut dir_sizes: Vec<(String, usize)> = vec![];
        self.dir_sizes(&mut dir_sizes);

        debug!("{:?}", dir_sizes);

        dir_sizes = dir_sizes
            .iter()
            .filter(|(_, size)| *size <= max_size)
            .map(|(name, size)| (name.to_owned(), size.to_owned()))
            .collect::<Vec<_>>();

        debug!("{:?}", dir_sizes);

        dir_sizes
    }

    pub fn dirs_greater_than(&self, max_size: usize) -> Vec<(String, usize)> {
        let mut dir_sizes: Vec<(String, usize)> = vec![];
        self.dir_sizes(&mut dir_sizes);

        debug!("{:?}", dir_sizes);

        dir_sizes = dir_sizes
            .iter()
            .filter(|(_, size)| *size >= max_size)
            .map(|(name, size)| (name.to_owned(), size.to_owned()))
            .collect::<Vec<_>>();

        debug!("{:?}", dir_sizes);

        dir_sizes
    }

    pub fn total_sum_of_all_dirs_smaller_than(&self, max_size: usize) -> usize {
        let small_dirs = self.dirs_smaller_than(max_size);

        small_dirs.iter().fold(0, |acc, (_, size)| size + acc)
    }

    pub fn smallest_dir_greater_than(&self, max_size: usize) -> Option<usize> {
        let mut small_dirs = self
            .dirs_greater_than(max_size)
            .iter()
            .map(|(_, size)| size.to_owned())
            .collect::<Vec<usize>>();

        small_dirs.sort();
        small_dirs.first().map(|size| size.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use crate::fs::Dir;

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
}
