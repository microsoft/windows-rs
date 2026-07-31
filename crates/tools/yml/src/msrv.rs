use super::*;

pub fn yml() {
    write_yml(".github/workflows/msrv.yml", |yml| {
        let mut first = true;
        let mut current_version = None;
        let mut manifests = helpers::crates("crates/libs");
        manifests.sort_by(|a, b| {
            b.package
                .rust_version
                .cmp(&a.package.rust_version)
                .then_with(|| a.package.name.cmp(&b.package.name))
        });

        for manifest in manifests {
            let name = manifest.package.name;
            if name == "windows" {
                continue;
            }
            let version = manifest.package.rust_version.expect("rust-version");

            if current_version.as_deref() != Some(version.as_str()) {
                writeln!(
                    yml,
                    r"      - name: Rust version
        run: rustup update --no-self-update {version} && rustup default {version}"
                )
                .unwrap();
                current_version = Some(version);
            }

            if first {
                writeln!(yml, "      - uses: Swatinem/rust-cache@v2").unwrap();
                first = false;
            }

            if name == "windows-composition" {
                // The `system` and `lifted` stacks are mutually exclusive, so
                // `--all-features` won't compile; check each stack on its own.
                writeln!(
                    yml,
                    r"      - name: Check {name}
        run:  cargo check -p {name}
      - name: Check {name} (lifted)
        run:  cargo check -p {name} --no-default-features --features lifted"
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
