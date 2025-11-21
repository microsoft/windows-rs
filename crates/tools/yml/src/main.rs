mod clippy;
mod msrv;
mod no_default_features;
mod test;

use std::fmt::Write;
use std::io::BufRead;

fn main() {
    test::yml();
    clippy::yml();
    no_default_features::yml();
    msrv::yml();
}

#[track_caller]
fn write_yml<F>(path: &str, f: F)
where
    F: FnOnce(&mut String),
{
    let mut yml = String::new();
    let file = std::fs::File::open(path).unwrap();
    let file = std::io::BufReader::new(file);

    for line in file.lines() {
        let line = line.unwrap();
        yml.push_str(&line);
        yml.push('\n');

        if line.starts_with("# generated") {
            break;
        }
    }

    f(&mut yml);
    std::fs::write(path, yml.as_bytes()).unwrap();
}
