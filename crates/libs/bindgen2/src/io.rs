use std::io::BufRead;

pub fn read_file_lines(path: &str) -> Vec<String> {
    let file = std::io::BufReader::new(
        std::fs::File::open(path).unwrap_or_else(|_| path_panic(path, "failed to open file")),
    );
    file.lines()
        .map(|line| line.unwrap_or_else(|_| path_panic(path, "failed to read file lines")))
        .collect()
}

fn path_panic(path: &str, message: &str) -> ! {
    panic!("error: {message}\n  --> {path}");
}
