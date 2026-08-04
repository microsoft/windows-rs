use std::io::BufRead;

#[track_caller]
pub fn read_file_lines(path: &str) -> Vec<String> {
    let Ok(file) = std::fs::File::open(path) else {
        panic!("failed to open file `{path}`")
    };

    let file = std::io::BufReader::new(file);
    let mut lines = vec![];

    for line in file.lines() {
        let Ok(line) = line else {
            panic!("failed to read file lines `{path}`");
        };

        lines.push(line);
    }

    lines
}

#[track_caller]
pub fn write_to_file<C: AsRef<[u8]>>(path: &str, contents: C) -> bool {
    let contents = contents.as_ref();
    if std::fs::read(path).is_ok_and(|existing| existing == contents) {
        return false;
    }

    if let Some(parent) = std::path::Path::new(path).parent() {
        assert!(
            std::fs::create_dir_all(parent).is_ok(),
            "failed to create directory `{path}`"
        );
    }

    assert!(
        std::fs::write(path, contents).is_ok(),
        "failed to write file `{path}`"
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
        let path = path.to_str().unwrap();

        assert!(write_to_file(path, b"one"));
        assert!(!write_to_file(path, b"one"));
        assert!(write_to_file(path, b"two"));
        assert_eq!(std::fs::read(path).unwrap(), b"two");

        std::fs::remove_file(path).unwrap();
    }
}
