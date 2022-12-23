use std::borrow::Borrow;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct File {
    pub name: String,
    pub size: usize,
}

#[derive(Clone)]
pub struct Dir {
    pub(crate) name: String,
    pub(crate) sub_dirs: Vec<Dir>,
    pub(crate) files: Vec<File>,
}

pub struct DirIterator<'a> {
    parent_dirs: Vec<(&'a Dir, usize)>,
    current_dir: &'a Dir,
    current_subdirs_index: usize,
}

impl<'a> DirIterator<'a> {
    fn new(root: &'a Dir) -> Self {
        DirIterator {
            current_dir: root,
            current_subdirs_index: 0,
            parent_dirs: vec![],
        }
    }
}

impl<'a> Iterator for DirIterator<'a> {
    type Item = &'a Dir;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_subdirs_index >= self.current_dir.sub_dirs.len() {
            (self.current_dir, self.current_subdirs_index) = self.parent_dirs.pop()?;

            return self.next();
        }

        self.parent_dirs
            .push((self.current_dir, self.current_subdirs_index + 1));

        self.current_dir = self.current_dir.sub_dirs[self.current_subdirs_index].borrow();
        self.current_subdirs_index = 0;

        Some(self.current_dir)
    }
}

impl Display for Dir {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.indented_fmt(f, "")?;
        writeln!(f)
    }
}

impl<'a> IntoIterator for &'a Dir {
    type Item = &'a Dir;
    type IntoIter = DirIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl Dir {
    pub fn iter(&self) -> DirIterator {
        DirIterator::new(self)
    }

    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            sub_dirs: vec![],
            files: vec![],
        }
    }

    fn indented_fmt(&self, f: &mut Formatter<'_>, prefix: &str) -> std::fmt::Result {
        for file in self.files.iter() {
            writeln!(f, "{}├──{} {}", prefix, file.name, file.size)?;
        }
        for dir in self.sub_dirs.iter() {
            writeln!(f, "{}├─{}", prefix, dir.name)?;
            dir.indented_fmt(f, &format!("{prefix}  "))?;
        }
        write!(f, "")
    }

    pub fn new_fs() -> Self {
        let mut fs = Dir::new("");
        fs.sub_dirs.push(Dir::new("/"));

        fs
    }

    pub fn total_size(&self) -> usize {
        let mut size = 0;

        // add 'self' to iterator as we have to add our own file sum as well
        for dir in self.iter().chain(Some(self)) {
            size += dir.files.iter().map(|f| f.size).sum::<usize>();
        }

        size
    }

    pub fn total_sum_of_all_dirs_smaller_than(&self, max_size: usize) -> usize {
        self.iter()
            .map(|dir| dir.total_size())
            .filter(|size| *size < max_size)
            .sum()
    }

    pub fn smallest_dir_greater_than(&self, max_size: usize) -> Option<usize> {
        self.iter()
            .map(|dir| dir.total_size())
            .filter(|size| *size > max_size)
            .min()
    }
}
