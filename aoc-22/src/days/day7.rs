use std::str::FromStr;

use crate::day_exec::DayExecutor;
pub struct Day7;

const MIN_SEARCH_SIZE: usize = 100000;
const FS_TOTAL_SPACE: usize = 70000000;
const FS_UPDATE_SPACE_REQ: usize = 30000000;

impl DayExecutor for Day7 {
    fn exec_part1(&self, input: String) -> Box<dyn std::fmt::Display> {
        Box::new(format!(
            "Sum of all dir sizes under {}: {}",
            MIN_SEARCH_SIZE,
            solve_part1(&input)
        ))
    }

    fn exec_part2(&self, input: String) -> Box<dyn std::fmt::Display> {
        Box::new(format!(
            "Smallest dir that can be deleted so update has enough space has size of {}",
            solve_part2(&input)
        ))
    }
}

fn process_input(fs: &mut Filesystem, input: &str) {
    for line in input.lines() {
        if line.starts_with('$') {
            let cmd = line
                .parse::<Command>()
                .expect("Expect command to be parsable from line starting wiht '$'");

            match cmd {
                Command::Cd(arg) => fs.change_dir(arg),
                Command::Ls => (),
            }
        } else {
            let entity = line
                .parse::<FilesystemEntity>()
                .expect("Lines that do not start with '$' must be filesystem entities");

            match entity {
                FilesystemEntity::Directory(_) => (),
                FilesystemEntity::File(file) => fs.add_file_to_current_dir(file),
            }
        }
    }
}

fn solve_part1(input: &str) -> usize {
    let mut fs = Filesystem::new();

    process_input(&mut fs, input);

    fs.dir_sizes()
        .into_iter()
        .filter(|size| *size < MIN_SEARCH_SIZE)
        .sum()
}

fn solve_part2(input: &str) -> usize {
    let mut fs = Filesystem::new();

    process_input(&mut fs, input);

    fs.change_dir(CdArg::Root);
    let (_, space_occupied) = fs.current_dir();
    let free_space = FS_TOTAL_SPACE - space_occupied;
    let min_delete_size = FS_UPDATE_SPACE_REQ - free_space;

    fs.dir_sizes()
        .into_iter()
        .filter(|dir_size| *dir_size >= min_delete_size)
        .min()
        .expect("Could not find single dir to delete")
}

enum Command {
    Cd(CdArg),
    Ls,
}

#[derive(Debug)]
struct CommandParseError(String);
impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with('$') {
            Err(CommandParseError(s.to_owned()))
        } else {
            let trimmed = s.trim_start_matches("$").trim();
            if trimmed.starts_with("cd") {
                let arg = trimmed.trim_start_matches("cd ");
                arg.parse::<CdArg>()
                    .map(|arg| Self::Cd(arg))
                    .map_err(|_| CommandParseError(arg.to_owned()))
            } else if trimmed.starts_with("ls") {
                Ok(Self::Ls)
            } else {
                Err(CommandParseError(trimmed.to_owned()))
            }
        }
    }
}
enum CdArg {
    Root,
    Back,
    In(String),
}

struct CdArgParseErorr(String);
impl FromStr for CdArg {
    type Err = CdArgParseErorr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        match trimmed {
            "/" => Ok(Self::Root),
            ".." => Ok(Self::Back),
            name if trimmed.len() > 0 => Ok(Self::In(name.to_owned())),
            _ => Err(CdArgParseErorr(s.to_owned())),
        }
    }
}

enum FilesystemEntity {
    Directory(Dir),
    File(File),
}

#[derive(Debug)]
struct FilesystemEntityError(String);
impl FromStr for FilesystemEntity {
    type Err = FilesystemEntityError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("dir") {
            let dir = s
                .parse::<Dir>()
                .map_err(|DirParseError(err)| FilesystemEntityError(err))?;
            Ok(Self::Directory(dir))
        } else if s
            .chars()
            .next()
            .ok_or(FilesystemEntityError(s.to_owned()))?
            .is_digit(10)
        {
            let file = s
                .parse::<File>()
                .map_err(|FileParseError(err)| FilesystemEntityError(err))?;
            Ok(Self::File(file))
        } else {
            Err(FilesystemEntityError(s.to_owned()))
        }
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

impl File {
    fn new(name: String, size: usize) -> Self {
        Self { name, size }
    }
}

struct FileParseError(String);
impl FromStr for File {
    type Err = FileParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (size_str, name_str) = s
            .trim()
            .split_once(' ')
            .ok_or(FileParseError(s.to_owned()))?;

        let size = size_str
            .parse::<usize>()
            .map_err(|_| FileParseError(size_str.to_owned()))?;

        let name = name_str.trim().to_owned();

        Ok(Self::new(name, size))
    }
}

#[derive(Debug)]
struct Dir {
    name: String,
    files: Vec<File>,
}

impl Dir {
    fn new(name: String) -> Self {
        Self {
            name,
            files: Vec::new(),
        }
    }

    fn file_sizes(&self) -> usize {
        self.files.iter().map(|f| f.size).sum()
    }

    fn add_file(&mut self, file: File) {
        self.files.push(file)
    }
}

struct DirParseError(String);
impl FromStr for Dir {
    type Err = DirParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, dir_name) = s
            .trim()
            .split_once(' ')
            .ok_or(DirParseError(s.to_owned()))?;

        Ok(Self::new(dir_name.to_owned()))
    }
}

struct Filesystem {
    dirs: Vec<Dir>,
    active_dir_idx: usize,
    dir_links: Vec<DirLink>,
}

impl Filesystem {
    fn new() -> Self {
        let root = Dir::new("/".to_owned());
        let root_link = DirLink::root();
        Self {
            dirs: vec![root],
            active_dir_idx: 0,
            dir_links: vec![root_link],
        }
    }

    fn change_dir(&mut self, arg: CdArg) {
        match arg {
            CdArg::Root => self.active_dir_idx = 0,
            CdArg::Back => {
                self.active_dir_idx = self.dir_links[self.active_dir_idx]
                    .parent
                    .expect("Can't cd back from root")
            }
            CdArg::In(dir_name) => {
                if let Some(existing_child_idx) = self.dir_links[self.active_dir_idx]
                    .children
                    .iter()
                    .find_map(|child_idx| {
                        if self.dirs[*child_idx].name == *dir_name {
                            Some(child_idx)
                        } else {
                            None
                        }
                    })
                {
                    self.active_dir_idx = *existing_child_idx;
                } else {
                    let new_link = self.dir_links[self.active_dir_idx].add_child(self.dirs.len());
                    self.dirs.push(Dir::new(dir_name));
                    self.active_dir_idx = new_link.me;
                    self.dir_links.push(new_link);
                }
            }
        }
    }

    fn current_dir(&self) -> (&Dir, usize) {
        (
            &self.dirs[self.active_dir_idx],
            self.dir_links[self.active_dir_idx].total_dir_size,
        )
    }

    fn add_file_to_current_dir(&mut self, file: File) {
        let file_size = file.size;
        self.dirs[self.active_dir_idx].files.push(file);
        self.dir_links[self.active_dir_idx].total_dir_size += file_size;

        let mut idx = self.dir_links[self.active_dir_idx].parent;
        while let Some(parent_idx) = idx {
            idx = self.dir_links[parent_idx].parent;

            self.dir_links[parent_idx].total_dir_size += file_size;
        }
    }

    fn dir_sizes(&self) -> Vec<usize> {
        self.dir_links
            .iter()
            .map(|link| link.total_dir_size)
            .collect()
    }
}

struct DirLink {
    parent: Option<usize>,
    children: Vec<usize>,
    me: usize,
    total_dir_size: usize,
}

impl DirLink {
    fn root() -> Self {
        Self {
            parent: None,
            children: Vec::new(),
            me: 0,
            total_dir_size: 0,
        }
    }

    fn new(parent: usize, me: usize) -> Self {
        Self {
            parent: Some(parent),
            children: Vec::new(),
            me,
            total_dir_size: 0,
        }
    }

    fn add_child(&mut self, child_idx: usize) -> Self {
        let child = Self::new(self.me, child_idx);
        self.children.push(child.me);
        child
    }
}
