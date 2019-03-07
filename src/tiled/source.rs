use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use failure::Fallible;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Source {
    File(PathBuf),
}

impl Source {
    pub fn new_file(file: impl AsRef<Path>) -> Source {
        Source::File(dunce::canonicalize(file).unwrap())
    }

    pub fn relative(&self, rel: &str) -> Source {
        match self {
            Source::File(file) => {
                Source::File(dunce::canonicalize(file.parent().unwrap().join(rel)).unwrap())
            }
        }
    }

    pub fn read_all(&self) -> Fallible<Arc<[u8]>> {
        match self {
            Source::File(path) => Ok(std::fs::read(path)?.into()),
        }
    }
}

impl std::fmt::Display for Source {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Source::File(path) => std::fmt::Display::fmt(&path.to_string_lossy(), fmt),
        }
    }
}
