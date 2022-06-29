use std::{
    fs::{self, DirEntry},
    path::PathBuf,
};

#[cfg(feature = "pathfilter")]
use pathfilter::PathFilter;

pub struct PathWalker {
    entries: Vec<PathBuf>,
    buffer: Vec<DirEntry>,
    follow_symlinks: bool,
    #[cfg(feature = "pathfilter")]
    filters: Vec<Box<dyn PathFilter>>,
}

impl PathWalker {
    pub fn new(path: PathBuf) -> Self {
        Self {
            entries: vec![path],
            buffer: Vec::new(),
            follow_symlinks: false,
            #[cfg(feature = "pathfilter")]
            filters: Vec::new(),
        }
    }

    pub fn follow_symlinks(mut self) -> Self {
        self.follow_symlinks = true;
        self
    }
}

    #[cfg(feature = "pathfilter")]
impl PathWalker {
    pub fn add_filter(mut self, filter: Box<dyn PathFilter>) -> Self {
        self.filters.push(filter);
        self
    }

    pub fn with_filter(mut self, filter: impl PathFilter + 'static) -> Self {
        self.filters.push(Box::new(filter));
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
                        #[cfg(feature = "pathfilter")]
                        if self.filters.iter().any(|f| f.ignore(&entry_path)) {
                            continue;
                        }

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
