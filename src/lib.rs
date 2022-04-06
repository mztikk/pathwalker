use std::{
    fs::{self, DirEntry},
    path::PathBuf,
};

pub struct PathWalker {
    entries: Vec<PathBuf>,
    buffer: Vec<DirEntry>,
}

impl PathWalker {
    pub fn new(path: PathBuf) -> Self {
        Self {
            entries: vec![path],
            buffer: Vec::new(),
        }
    }
}

impl Iterator for PathWalker {
    type Item = DirEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.buffer.is_empty() {
            return self.buffer.pop();
        }

        let entry = self.entries.pop();

        if let Some(entry) = entry {
            if let Ok(paths) = fs::read_dir(entry) {
                for path in paths.flatten() {
                    let entry_path = path.path();
                    if !entry_path.is_symlink() {
                        if entry_path.is_dir() {
                            self.entries.push(entry_path);
                        }

                        self.buffer.push(path);
                    }
                }
            }
        }

        self.buffer.pop()
    }
}
