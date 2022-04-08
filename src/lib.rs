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
                    let current_entry = self.entries.pop();

                    if let Some(current_entry) = current_entry {
                        if let Ok(read_dir) = fs::read_dir(&current_entry) {
                            for entry in read_dir.flatten() {
                                let entry_path = entry.path();
                                if !entry_path.is_symlink() || self.follow_symlinks {
                                    if entry_path.is_dir() {
                                        self.entries.push(entry_path);
                                    }

                                    self.buffer.push(entry);
                                }
                            }
                        }
                    }
                }

                self.buffer.pop()
            }
        }
    }
}
