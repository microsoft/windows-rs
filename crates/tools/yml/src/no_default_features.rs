use super::*;

pub fn yml() {
    let mut yml = r"name: no-default-features

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
  no-default-features:
    runs-on: windows-2025
    steps:
      - name: Checkout
        uses: actions/checkout@v5
      - name: Update toolchain
        run: rustup update --no-self-update nightly && rustup default nightly-x86_64-pc-windows-msvc
      - name: Add toolchain target
        run: rustup target add x86_64-pc-windows-msvc
      - name: Fix environment
        uses: ./.github/actions/fix-environment
        with:
          target: x86_64-pc-windows-msvc"
        .to_string();

    for manifest in helpers::crates("crates/libs") {
        let name = manifest.package.name;
        write!(
            &mut yml,
            r"
      - name: Check {name}
        run:  cargo check -p {name} --no-default-features"
        )
        .unwrap();
    }

    std::fs::write(".github/workflows/no-default-features.yml", yml.as_bytes()).unwrap();
}
