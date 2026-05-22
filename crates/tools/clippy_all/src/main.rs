use std::process::Command;

fn main() {
    let crates = helpers::crates("crates");
    let mut failed = vec![];

    for krate in &crates {
        let name = &krate.package.name;

        println!("clippy: {name}");

        let status = Command::new("cargo")
            .arg("clippy")
            .arg("--all-targets")
            .arg("-p")
            .arg(name)
            .status()
            .expect("failed to run cargo clippy");

        if !status.success() {
            failed.push(name.clone());
        }
    }

    if !failed.is_empty() {
        eprintln!("\nclippy failed for:");
        for name in &failed {
            eprintln!("  {name}");
        }
        std::process::exit(1);
    }

    println!("\nclippy passed for all {} crates", crates.len());
}
