use super::*;

pub fn yml() {
    let mut yml = r"name: msrv

on:
  pull_request:
    paths-ignore:
      - '.github/ISSUE_TEMPLATE/**'
      - '.github/workflows/web.yml'
      - 'web/**'
  push:
    paths-ignore:
      - '.github/ISSUE_TEMPLATE/**'
      - '.github/workflows/web.yml'
      - 'web/**'
    branches:
      - master

jobs:
  msrv:
    runs-on: windows-2025
    steps:
      - name: Checkout
        uses: actions/checkout@v5"
        .to_string();

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

        write!(
            &mut yml,
            r"
      - name: Rust version
        run: rustup update --no-self-update {version} && rustup default {version}
      - name: Check {name}
        run:  cargo check -p {name}{features}"
        )
        .unwrap();
    }

    std::fs::write(".github/workflows/msrv.yml", yml.as_bytes()).unwrap();
}
