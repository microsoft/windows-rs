use super::*;

pub fn yml() {
    write_yml(".github/workflows/msrv.yml", |yml| {
        for manifest in helpers::crates("crates/libs") {
            let name = manifest.package.name;
            let version = manifest.package.rust_version.expect("rust-version");

            let features = if name == "windows" {
                // We can't use `--all-features` for the `windows` crate as that would exhaust the available
                // memory on GitHub VMs so this is just a smoke test for representative Win32 and WinRT APIs.
                " --features Globalization,Win32_Graphics_Direct2D"
            } else {
                " --all-features"
            };

            writeln!(
                yml,
                r"      - name: Rust version
        run: rustup update --no-self-update {version} && rustup default {version}
      - name: Check {name}
        run:  cargo check -p {name}{features}"
            )
            .unwrap();
        }
    });
}
