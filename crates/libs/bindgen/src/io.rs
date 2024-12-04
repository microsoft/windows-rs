use std::io::BufRead;

pub fn read_file_lines(path: &str) -> Vec<String> {
    let file = std::io::BufReader::new(
        std::fs::File::open(path).unwrap_or_else(|_| panic!("failed to open file `{path}`")),
    );

    file.lines()
        .map(|line| line.unwrap_or_else(|_| panic!("failed to read file lines `{path}`")))
        .collect()
}

pub fn write_to_file<C: AsRef<[u8]>>(path: &str, contents: C) {
    if let Some(parent) = std::path::Path::new(path).parent() {
        std::fs::create_dir_all(parent)
            .unwrap_or_else(|_| panic!("failed to create directory `{path}`"));
    }

    std::fs::write(path, contents).unwrap_or_else(|_| panic!("failed to write file `{path}`"));
}
