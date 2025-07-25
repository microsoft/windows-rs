use super::*;

pub fn yml() {
    let mut yml = r"name: clippy

on:
  pull_request:
    paths-ignore:
      - '.github/ISSUE_TEMPLATE/**'
      - 'web/**'
  push:
    paths-ignore:
      - '.github/ISSUE_TEMPLATE/**'
      - 'web/**'
    branches:
      - master

jobs:
  check:
    runs-on: windows-2022
    steps:
      - name: Checkout
        uses: actions/checkout@v4
      - name: Update toolchain
        run: rustup update --no-self-update nightly && rustup default nightly-x86_64-pc-windows-msvc
      - name: Add toolchain target
        run: rustup target add x86_64-pc-windows-msvc
      - name: Install clippy
        run: rustup component add clippy
      - name: Fix environment
        uses: ./.github/actions/fix-environment
        with:
          target: x86_64-pc-windows-msvc"
        .to_string();

    // This unrolling is required since "cargo clippy --all" consumes too much memory for the GitHub hosted runners.

    for package in helpers::crates("crates") {
        let name = &package.name;

        if name.starts_with("test") {
            continue;
        }

        write!(
            &mut yml,
            r"
      - name: Check {name}
        run:  cargo clippy -p {name}"
        )
        .unwrap();
    }

    std::fs::write(".github/workflows/clippy.yml", yml.as_bytes()).unwrap();
}
