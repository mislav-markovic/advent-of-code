use std::str::FromStr;

use crate::day_exec::DayExecutor;
pub struct Day7;

impl DayExecutor for Day7 {
    fn exec_part1(&self, input: String) -> Box<dyn std::fmt::Display> {
        Box::new("TODO")
    }

    fn exec_part2(&self, input: String) -> Box<dyn std::fmt::Display> {
        Box::new("TODO")
    }
}

enum Command {
    Cd(CdArg),
    Ls,
}

enum CdArg {
    Root,
    Back,
    In(String),
}

enum FilesystemEntity {
    Directory(Dir),
    File(File),
}

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

struct Dir {
    name: String,
    files: Vec<File>,
    subdirs: Vec<Dir>,
}

impl Dir {
    fn new(name: String) -> Self {
        Self {
            name,
            files: Vec::new(),
            subdirs: Vec::new(),
        }
    }

    fn file_sizes(&self) -> usize {
        self.files.iter().map(|f| f.size).sum()
    }

    fn total_size(&self) -> usize {
        self.file_sizes()
            + self
                .subdirs
                .iter()
                .map(|dir| dir.total_size())
                .sum::<usize>()
    }

    fn add_file(&mut self, file: File) {
        self.files.push(file)
    }

    fn add_subdir(&mut self, child: Dir) {
        self.subdirs.push(child)
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

struct Path {
    parts: Vec<String>,
}

impl Path {
    fn root() -> Self {
        Self {
            parts: vec!["/".to_owned()],
        }
    }

    fn back(&mut self) {
        self.parts.pop().expect("Can't move back above the roo");
    }

    fn move_down(&mut self, to: String) {
        self.parts.push(to);
    }

    fn full_path(&self) -> String {
        self.parts.join("/")
    }

    fn active_dir_name(&self) -> &str {
        self.parts
            .last()
            .expect("Path should always have at leas one dir in it")
    }

    fn parts(&self) -> &[String] {
        self.parts.as_slice()
    }
}

struct Filesystem {
    current_dir_path: Path,
    root_dir: Dir,
}

impl Filesystem {
    fn new() -> Self {
        let root = Path::root();
        let root_name = root.active_dir_name().to_owned();
        Self {
            current_dir_path: root,
            root_dir: Dir::new(root_name),
        }
    }

    fn change_dir(&mut self, cd_arg: CdArg) {
        match cd_arg {
            CdArg::Root => self.current_dir_path = Path::root(),
            CdArg::Back => self.current_dir_path.back(),
            CdArg::In(dir_name) => {
                let curr = self.current_dir_mut();
                if let None = curr.subdirs.iter().find(|d| d.name == dir_name) {
                    curr.add_subdir(Dir::new(dir_name.clone()))
                }
                self.current_dir_path.move_down(dir_name)
            }
        }
    }

    fn current_dir_mut(&mut self) -> &mut Dir {
        let mut res = &mut self.root_dir;

        for dirs in self.current_dir_path.parts().iter().skip(1) {
            res =
                res.subdirs.iter_mut().find(|d| d.name == *dirs).expect(
                    "Filesystem invariant broken! Current dir path must point to existing dir",
                );
        }

        res
    }
}
