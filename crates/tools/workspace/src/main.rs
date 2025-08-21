use std::fmt::Write;
use std::io::BufRead;

fn main() {
    let mut toml = String::new();

    let file = std::fs::File::open("Cargo.toml").unwrap();
    let file = std::io::BufReader::new(file);

    for line in file.lines() {
        let line = line.unwrap();
        toml.push_str(&line);
        toml.push('\n');

        if line == "# generated dependencies" {
            break;
        }
    }

    for manifest in helpers::crates("crates/libs") {
        let package = manifest.package;

        writeln!(
            &mut toml,
            r#"{} = {{ version = "{}", path = "crates/libs/{}", default-features = false }}"#,
            package.name,
            package.version,
            package.name.trim_start_matches("windows-")
        )
        .unwrap();
    }

    std::fs::write("Cargo.toml", toml).unwrap();
}
