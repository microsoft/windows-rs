use super::*;

pub fn yml() {
    write_yml(".github/workflows/msrv.yml", |yml| {
        for manifest in helpers::crates("crates/libs") {
            let name = manifest.package.name;
            if name == "windows" {
                continue;
            }
            let version = manifest.package.rust_version.expect("rust-version");

            writeln!(
                yml,
                r"      - name: Rust version
        run: rustup update --no-self-update {version} && rustup default {version}
      - name: Check {name}
        run:  cargo check -p {name} --all-features"
            )
            .unwrap();
        }
    });

    write_yml(".github/workflows/msrv-windows.yml", |yml| {
        for manifest in helpers::crates("crates/libs") {
            let name = manifest.package.name;
            if name != "windows" {
                continue;
            }
            let version = manifest.package.rust_version.expect("rust-version");

            writeln!(
                yml,
                r"      - name: Rust version
        run: rustup update --no-self-update {version} && rustup default {version}
      - name: Check {name}
        run:  cargo check -p {name} --all-features"
            )
            .unwrap();
        }
    });
}
