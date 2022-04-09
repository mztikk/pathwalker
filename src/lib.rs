use std::{
    fs::{self, DirEntry},
    path::PathBuf,
};

pub struct PathWalker {
    entries: Vec<PathBuf>,
    buffer: Vec<DirEntry>,
    follow_symlinks: bool,
}

impl PathWalker {
    pub fn new(path: PathBuf) -> Self {
        Self {
            entries: vec![path],
            buffer: Vec::new(),
            follow_symlinks: false,
        }
    }

    pub fn follow_symlinks(mut self) -> Self {
        self.follow_symlinks = true;
        self
    }
}

impl Iterator for PathWalker {
    type Item = DirEntry;

    fn next(&mut self) -> Option<Self::Item> {
        match self.buffer.pop() {
            Some(entry) => Some(entry),
            None => {
                while self.buffer.is_empty() && !self.entries.is_empty() {
                    for (entry_path, entry) in self
                        .entries
                        .pop()
                        .map(fs::read_dir)
                        .into_iter()
                        .flatten()
                        .flatten()
                        .flatten()
                        .map(|e| (e.path(), e))
                    {
                        if !entry_path.is_symlink() || self.follow_symlinks {
                            if entry_path.is_dir() {
                                self.entries.push(entry_path);
                            }

                            self.buffer.push(entry);
                        }
                    }
                }

                self.buffer.pop()
            }
        }
    }
}
