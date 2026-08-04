use std::io::BufRead;
use std::path::Path;

#[track_caller]
pub fn read_file_lines(path: impl AsRef<Path>) -> Vec<String> {
    let path = path.as_ref();
    let Ok(file) = std::fs::File::open(path) else {
        panic!("failed to open file `{}`", path.display())
    };

    let file = std::io::BufReader::new(file);
    let mut lines = vec![];

    for line in file.lines() {
        let Ok(line) = line else {
            panic!("failed to read file lines `{}`", path.display());
        };

        lines.push(line);
    }

    lines
}

#[track_caller]
pub fn write_to_file<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> bool {
    let path = path.as_ref();
    let contents = contents.as_ref();
    if std::fs::read(path).is_ok_and(|existing| existing == contents) {
        return false;
    }

    if let Some(parent) = path.parent() {
        assert!(
            std::fs::create_dir_all(parent).is_ok(),
            "failed to create directory `{}`",
            path.display()
        );
    }

    assert!(
        std::fs::write(path, contents).is_ok(),
        "failed to write file `{}`",
        path.display()
    );
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unchanged_file_is_not_rewritten() {
        let path = std::env::temp_dir().join(format!(
            "windows-bindgen-write-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        assert!(write_to_file(&path, b"one"));
        assert!(!write_to_file(&path, b"one"));
        assert!(write_to_file(&path, b"two"));
        assert_eq!(std::fs::read(&path).unwrap(), b"two");

        std::fs::remove_file(&path).unwrap();
    }
}
