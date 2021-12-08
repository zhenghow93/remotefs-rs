//! ## File
//!
//! file system types related to file entries and directories

/**
 * MIT License
 *
 * remotefs - Copyright (c) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
// -- ext
use std::path::{Path, PathBuf};

// -- mod
mod metadata;
mod permissions;

// -- export
pub use metadata::Metadata;
pub use permissions::{UnixPex, UnixPexClass};

/// Entry represents a generic entry in a directory

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Entry {
    Directory(Directory),
    File(File),
}

/// Directory provides an interface to file system directories

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Directory {
    /// Directory name
    pub name: String,
    /// File absolute path
    pub path: PathBuf,
    /// File metadata
    pub metadata: Metadata,
}

/// ### File
///
/// File provides an interface to file system files

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct File {
    /// File name
    pub name: String,
    /// Absolute path
    pub path: PathBuf,
    /// File type
    pub extension: Option<String>,
    /// File metadata
    pub metadata: Metadata,
}

impl Entry {
    /// Get absolute path from `Entry`
    pub fn path(&self) -> &Path {
        match self {
            Entry::Directory(dir) => dir.path.as_path(),
            Entry::File(file) => file.path.as_path(),
        }
    }

    /// Get file name from `Entry`
    pub fn name(&self) -> &'_ str {
        match self {
            Entry::Directory(dir) => dir.name.as_ref(),
            Entry::File(file) => file.name.as_ref(),
        }
    }

    /// Get metadata from `Entry`
    pub fn metadata(&self) -> &Metadata {
        match self {
            Entry::Directory(dir) => &dir.metadata,
            Entry::File(file) => &file.metadata,
        }
    }

    /// Get file type from `Entry`. For directories is always None
    pub fn extension(&self) -> Option<&'_ str> {
        match self {
            Entry::Directory(_) => None,
            Entry::File(file) => file.extension.as_deref(),
        }
    }

    /// Returns whether a Entry is a directory
    pub fn is_dir(&self) -> bool {
        matches!(self, Entry::Directory(_))
    }

    /// Returns whether a Entry is a File
    pub fn is_file(&self) -> bool {
        matches!(self, Entry::File(_))
    }

    /// Returns whether Entry is hidden
    pub fn is_hidden(&self) -> bool {
        self.name().starts_with('.')
    }

    /// Unwrap Entry as File
    pub fn unwrap_file(self) -> File {
        match self {
            Entry::File(file) => file,
            _ => panic!("unwrap_file: not a File"),
        }
    }

    /// Unwrap Entry as Directory
    pub fn unwrap_dir(self) -> Directory {
        match self {
            Entry::Directory(dir) => dir,
            _ => panic!("unwrap_dir: not a Directory"),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_create_fs_dir() {
        let entry: Entry = Entry::Directory(Directory {
            name: String::from("foo"),
            path: PathBuf::from("/foo"),
            metadata: Metadata::default(),
        });
        assert_eq!(entry.metadata().size, 0);
        assert_eq!(entry.is_dir(), true);
        assert_eq!(entry.is_file(), false);
        assert_eq!(entry.unwrap_dir().path, PathBuf::from("/foo"));
    }

    #[test]
    fn should_create_fs_file() {
        let entry: Entry = Entry::File(File {
            name: String::from("bar.txt"),
            path: PathBuf::from("/bar.txt"),
            extension: Some(String::from("txt")),
            metadata: Metadata::default(),
        });
        assert_eq!(entry.path(), Path::new("/bar.txt"));
        assert_eq!(entry.name(), String::from("bar.txt"));
        assert_eq!(entry.extension(), Some("txt"));
        assert_eq!(entry.is_dir(), false);
        assert_eq!(entry.is_file(), true);
        assert_eq!(entry.unwrap_file().path, PathBuf::from("/bar.txt"));
    }

    #[test]
    #[should_panic]
    fn should_fail_unwrapping_directory() {
        let entry: Entry = Entry::File(File {
            name: String::from("bar.txt"),
            path: PathBuf::from("/bar.txt"),
            metadata: Metadata::default(),
            extension: Some(String::from("txt")),
        });
        entry.unwrap_dir();
    }

    #[test]
    #[should_panic]
    fn should_fail_unwrapping_file() {
        let entry: Entry = Entry::Directory(Directory {
            name: String::from("foo"),
            path: PathBuf::from("/foo"),
            metadata: Metadata::default(),
        });
        entry.unwrap_file();
    }

    #[test]
    fn should_return_is_hidden_for_hidden_files() {
        let entry: Entry = Entry::File(File {
            name: String::from("bar.txt"),
            path: PathBuf::from("/bar.txt"),
            metadata: Metadata::default(),
            extension: Some(String::from("txt")),
        });
        assert_eq!(entry.is_hidden(), false);
        let entry: Entry = Entry::File(File {
            name: String::from(".gitignore"),
            path: PathBuf::from("/.gitignore"),
            metadata: Metadata::default(),
            extension: Some(String::from("txt")),
        });
        assert_eq!(entry.is_hidden(), true);
        let entry: Entry = Entry::Directory(Directory {
            name: String::from(".git"),
            path: PathBuf::from("/.git"),
            metadata: Metadata::default(),
        });
        assert_eq!(entry.is_hidden(), true);
    }
}
