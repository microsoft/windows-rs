use std::io::BufRead;

pub fn read_file_lines(path: &str) -> Vec<String> {
    let file = std::io::BufReader::new(
        std::fs::File::open(path)
            .unwrap_or_else(|_| panic!("windows-bindgen: failed to open file `{path}`")),
    );

    file.lines()
        .map(|line| {
            line.unwrap_or_else(|_| panic!("windows-bindgen: failed to read file lines `{path}`"))
        })
        .collect()
}
