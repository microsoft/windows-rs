use crate::file::WinmdFile;

use std::path::{Path, PathBuf};

/// Get [`WinmdFile`]s from the operating system
///
/// This searches well known paths for Windows metadata related to
/// operating system APIs.
pub fn from_os() -> Vec<WinmdFile> {
    let windir = std::env::var("windir").expect("No `windir` environent variable set");
    let mut path = PathBuf::from(windir);
    path.push(super::SYSTEM32);
    path.push("winmetadata");
    from_dir(path)
}

/// Get [`WinmdFile`]s from a directory
pub fn from_dir<P: AsRef<Path>>(directory: P) -> Vec<WinmdFile> {
    let files = std::fs::read_dir(directory)
        .unwrap()
        .filter_map(|value| value.ok())
        .map(|value| value.path());
    // TODO: filter out directories and non-metadata files
    from_files(files)
}

/// Get [`WinmdFile`]s from an iterator of file paths
pub fn from_files<P: IntoIterator<Item = PathBuf>>(filenames: P) -> Vec<WinmdFile> {
    filenames.into_iter().map(WinmdFile::new).collect()
}
