use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::{Rc, Weak},
};

use anyhow::{anyhow, Context, Result};

use crate::problem::{Problem, Solution};

struct Directory {
    parent: Option<Weak<RefCell<Directory>>>,
    file_size: u64,
    directories: HashMap<String, Rc<RefCell<Directory>>>,
}

impl Directory {
    pub fn dir_size(&self) -> u64 {
        let subdirectory_size = self
            .directories
            .values()
            .map(|d| {
                if let Ok(dir) = d.try_borrow() {
                    dir.dir_size()
                } else {
                    0
                }
            })
            .sum::<u64>();

        self.file_size + subdirectory_size
    }
}

struct Filesystem {
    root: Rc<RefCell<Directory>>,
    current_directory: Weak<RefCell<Directory>>,
}

impl Filesystem {
    pub fn new() -> Self {
        let root_directory = Rc::new(RefCell::new(Directory {
            parent: None,
            file_size: 0,
            directories: HashMap::new(),
        }));
        Self {
            root: root_directory,
            current_directory: Weak::new(),
        }
    }

    pub fn init(&mut self) {
        self.current_directory = Rc::downgrade(&self.root);
    }

    pub fn mkdir(&self, name: &str) -> Result<()> {
        let new_directory = Rc::new(RefCell::new(Directory {
            parent: Some(Weak::clone(&self.current_directory)),
            file_size: 0,
            directories: HashMap::new(),
        }));

        self.current_directory
            .upgrade()
            .context("Could not access current directory")?
            .try_borrow_mut()?
            .directories
            .insert(name.to_owned(), new_directory);

        Ok(())
    }

    pub fn chdir(&mut self, name: &str) -> Result<()> {
        if name == ".." {
            if let Some(parent_dir) = &self
                .current_directory
                .upgrade()
                .context("Could not access current directory")?
                .try_borrow()?
                .parent
            {
                self.current_directory = Weak::clone(parent_dir);
                Ok(())
            } else {
                Err(anyhow!("Could not access parent of current directory"))
            }
        } else if name == "/" {
            self.current_directory = Rc::downgrade(&self.root);
            Ok(())
        } else if let Some(new_dir) = self
            .current_directory
            .upgrade()
            .context("Could not access current directory")?
            .try_borrow_mut()?
            .directories
            .get(name)
        {
            self.current_directory = Rc::downgrade(new_dir);
            Ok(())
        } else {
            Err(anyhow!("Could not change directory"))
        }
    }

    pub fn mkfile(&mut self, size: u64) -> Result<()> {
        self.current_directory
            .upgrade()
            .context("Could not access current directory")?
            .try_borrow_mut()?
            .file_size += size;

        Ok(())
    }
}

pub struct Problem07 {}

impl Problem07 {
    pub fn new() -> Problem07 {
        Problem07 {}
    }

    fn parse(&self, data: &str) -> Filesystem {
        let mut fs = Filesystem::new();
        fs.init();

        data.lines().for_each(|l| {
            if l.starts_with("$ cd") {
                fs.chdir(&l[5..]).unwrap()
            } else if l.starts_with("dir") {
                fs.mkdir(&l[4..]).unwrap();
            } else if l.starts_with(|c: char| c.is_numeric()) {
                let mut file_split = l.split(' ');
                fs.mkfile(file_split.next().unwrap().parse::<u64>().unwrap())
                    .unwrap();
            }
        });

        fs
    }

    fn solve_actual(&self, filesystem: &Filesystem) -> u64 {
        let root_directory = filesystem.root.borrow();
        let mut dirs_to_walk: VecDeque<Rc<RefCell<Directory>>> =
            VecDeque::from_iter(root_directory.directories.values().map(Rc::clone));
        let mut total_size = 0;

        while let Some(dir) = dirs_to_walk.pop_front() {
            let dir_ref = dir.borrow();
            let dir_size = dir_ref.dir_size();

            if dir_size <= 100000 {
                total_size += dir_size;
            }

            dirs_to_walk.extend(dir_ref.directories.values().map(Rc::clone));
        }

        total_size
    }

    fn solve_actual_part2(&self, filesystem: &Filesystem) -> u64 {
        let root_directory = filesystem.root.borrow();
        let root_size = root_directory.dir_size();
        let space_remaining = 70_000_000 - root_size;
        let space_to_free = 30_000_000 - space_remaining;
        let mut dirs_to_walk: VecDeque<Rc<RefCell<Directory>>> =
            VecDeque::from_iter(root_directory.directories.values().map(Rc::clone));

        let mut smallest_candidate = root_size;

        while let Some(dir) = dirs_to_walk.pop_front() {
            let dir_ref = dir.borrow();
            let dir_size = dir_ref.dir_size();

            // Only directories that are already large enough to be candidates
            // can possibly have children that are also candidates.
            if dir_size >= space_to_free {
                if dir_size <= smallest_candidate {
                    smallest_candidate = dir_size;
                }
                dirs_to_walk.extend(dir_ref.directories.values().map(Rc::clone));
            }
        }

        smallest_candidate
    }
}

impl Problem for Problem07 {
    fn name(&self) -> &str {
        "Day 7: No Space Left On Device"
    }

    fn solve(&self) -> Solution {
        let data = get_input!("./inputs/problem_07.txt");
        let filesystem = self.parse(&data);
        Solution::U64(self.solve_actual(&filesystem))
    }

    fn solve_part2(&self) -> Solution {
        let data = get_input!("./inputs/problem_07.txt");
        let filesystem = self.parse(&data);
        Solution::U64(self.solve_actual_part2(&filesystem))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_actual_from_example() {
        let problem = Problem07::new();
        let fs = problem.parse(
            "$ cd /
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
7214296 k",
        );
        assert_eq!(problem.solve_actual(&fs), 95437);
    }

    #[test]
    fn test_solve_actual_part2_from_example() {
        let problem = Problem07::new();
        let fs = problem.parse(
            "$ cd /
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
7214296 k",
        );
        assert_eq!(problem.solve_actual_part2(&fs), 24933642);
    }
}
