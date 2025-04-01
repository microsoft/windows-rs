fn main() {
    for package in helpers::crates("crates") {
        let mut command = std::process::Command::new("cargo.exe");
        command.args(["test", "-p", &package.name]);

        if command.status().unwrap().success() {
            println!("{}", package.name);
        } else {
            panic!("Failed: cargo test -p {}", package.name);
        }
    }
}
