use super::*;

pub fn yml() {
    write_yml(".github/workflows/msrv.yml", |yml| {
        let mut first = true;
        for manifest in helpers::crates("crates/libs") {
            let name = manifest.package.name;
            if name == "windows" {
                continue;
            }
            let version = manifest.package.rust_version.expect("rust-version");

            writeln!(
                yml,
                r"      - name: Rust version
        run: rustup update --no-self-update {version} && rustup default {version}"
            )
            .unwrap();

            if first {
                writeln!(yml, "      - uses: Swatinem/rust-cache@v2").unwrap();
                first = false;
            }

            if name == "windows-composition" {
                // The `system` and `reactor` stacks are mutually exclusive, so
                // `--all-features` won't compile; check each stack on its own.
                writeln!(
                    yml,
                    r"      - name: Check {name}
        run:  cargo check -p {name}
      - name: Check {name} (reactor)
        run:  cargo check -p {name} --no-default-features --features reactor"
                )
                .unwrap();
                continue;
            }

            writeln!(
                yml,
                r"      - name: Check {name}
        run:  cargo check -p {name} --all-features"
            )
            .unwrap();
        }
    });

    write_yml(".github/workflows/msrv-windows.yml", |yml| {
        let mut first = true;
        for manifest in helpers::crates("crates/libs") {
            let name = manifest.package.name;
            if name != "windows" {
                continue;
            }
            let version = manifest.package.rust_version.expect("rust-version");

            writeln!(
                yml,
                r"      - name: Rust version
        run: rustup update --no-self-update {version} && rustup default {version}"
            )
            .unwrap();

            if first {
                writeln!(yml, "      - uses: Swatinem/rust-cache@v2").unwrap();
                first = false;
            }

            writeln!(
                yml,
                r"      - name: Check {name}
        run:  cargo check -p {name} --all-features"
            )
            .unwrap();
        }
    });
}
