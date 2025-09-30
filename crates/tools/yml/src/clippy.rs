use super::*;

pub fn yml() {
    let mut yml = r"name: clippy

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
  clippy:
    runs-on: windows-2025
    steps:
      - name: Checkout
        uses: actions/checkout@v5
      - name: Update toolchain
        run: rustup update --no-self-update nightly && rustup default nightly-x86_64-pc-windows-msvc
      - name: Add toolchain target
        run: rustup target add x86_64-pc-windows-msvc
      - name: Install clippy
        run: rustup component add clippy
      - name: Install rustfmt
        run: rustup component add rustfmt
      - name: Fix environment
        uses: ./.github/actions/fix-environment
        with:
          target: x86_64-pc-windows-msvc"
        .to_string();

    // This unrolling is required since "cargo clippy --all" consumes too much memory for the GitHub hosted runners.

    for manifest in helpers::crates("crates") {
        let name = manifest.package.name;

        write!(
            &mut yml,
            r"
      - name: Check {name}
        run:  cargo clippy -p {name} --tests"
        )
        .unwrap();
    }

    std::fs::write(".github/workflows/clippy.yml", yml.as_bytes()).unwrap();
}
