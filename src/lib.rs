#[cfg(feature = "pathfilter")]
use pathfilter::PathFilter;
use std::{
    fs::{self, DirEntry},
    path::PathBuf,
};

pub struct PathWalker {
    directories: Vec<PathBuf>,
    items: Vec<DirEntry>,
    follow_symlinks: bool,
    #[cfg(feature = "pathfilter")]
    filters: Vec<Box<dyn PathFilter>>,
}

impl PathWalker {
    pub fn new(path: PathBuf) -> Self {
        Self {
            directories: vec![path],
            items: Vec::new(),
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

impl PathWalker {
    fn handle_entry(&mut self, entry: DirEntry) {
        let entry_path = entry.path();

        #[cfg(feature = "pathfilter")]
        if self.filters.iter().any(|f| f.ignore(&entry_path)) {
            return;
        }

        if let Ok(metadata) = entry.metadata() {
            if self.follow_symlinks || !metadata.is_symlink() {
                if metadata.is_dir() {
                    self.directories.push(entry_path);
                }

                self.items.push(entry);
            }
        };
    }
}

impl Iterator for PathWalker {
    type Item = DirEntry;

    fn next(&mut self) -> Option<Self::Item> {
        match self.items.pop() {
            Some(entry) => Some(entry),
            None => {
                while self.items.is_empty() && !self.directories.is_empty() {
                    self.directories
                        .pop()
                        .map(fs::read_dir)
                        .into_iter()
                        .flatten()
                        .flatten()
                        .flatten()
                        .for_each(|entry| self.handle_entry(entry))
                }

                self.items.pop()
            }
        }
    }
}
