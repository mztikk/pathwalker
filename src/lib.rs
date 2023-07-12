use direntryfilter::{DirEntryFilter, DirectoryOnlyFilter, FileOnlyFilter, IgnoreDirEntry};
use moar_options::*;
#[cfg(feature = "pathfilter")]
use pathfilter::{IgnorePath, PathFilter};
use std::{
    fs::{self, DirEntry},
    path::PathBuf,
};

pub struct PathWalker {
    directories: Vec<PathBuf>,
    items: Vec<DirEntry>,
    follow_symlinks: bool,
    max_depth: Option<u64>,
    current_depth: u64,
    direntry_filters: Vec<DirEntryFilter>,
    #[cfg(feature = "pathfilter")]
    path_filters: Vec<PathFilter>,
}

impl PathWalker {
    pub fn new<T: Into<PathBuf>>(path: T) -> Self {
        Self {
            directories: vec![path.into()],
            items: Vec::new(),
            follow_symlinks: false,
            max_depth: None,
            current_depth: 0,
            direntry_filters: Vec::new(),
            #[cfg(feature = "pathfilter")]
            path_filters: Vec::new(),
        }
    }

    pub fn follow_symlinks(mut self) -> Self {
        self.follow_symlinks = true;
        self
    }

    pub fn with_max_depth<T: Into<Option<u64>>>(mut self, max_depth: T) -> Self {
        self.max_depth = max_depth.into();
        self
    }
}

impl Default for PathWalker {
    fn default() -> Self {
        Self::new(".")
    }
}

#[cfg(feature = "pathfilter")]
impl PathWalker {
    pub fn with_filter<F: Into<PathFilter>>(mut self, filter: F) -> Self {
        self.path_filters.push(filter.into());
        self
    }

    pub fn with_filters<T: AsRef<[PathFilter]>>(mut self, filters: T) -> Self {
        self.path_filters.extend_from_slice(filters.as_ref());
        self
    }
}

impl PathWalker {
    pub fn files_only(mut self) -> Self {
        self.direntry_filters.push(FileOnlyFilter::default().into());
        self
    }

    pub fn directories_only(mut self) -> Self {
        self.direntry_filters
            .push(DirectoryOnlyFilter::default().into());
        self
    }
}

impl PathWalker {
    fn handle_entry(&mut self, entry: DirEntry) {
        let entry_path = entry.path();

        #[cfg(feature = "pathfilter")]
        if self.path_filters.ignore(&entry_path) {
            return;
        }

        if let Ok(file_type) = entry.file_type() {
            if self.follow_symlinks || !file_type.is_symlink() {
                if file_type.is_dir()
                    && self
                        .max_depth
                        .owned_is_none_or(|max_depth| self.current_depth < max_depth)
                {
                    self.directories.push(entry_path);
                    self.current_depth += 1;
                }

                if self.direntry_filters.ignore(&entry) {
                    return;
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
