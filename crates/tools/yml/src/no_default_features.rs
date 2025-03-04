use super::*;

pub fn yml() {
    let mut yml = r"name: no-default-features

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
      - name: Fix environment
        uses: ./.github/actions/fix-environment"
        .to_string();

    for package in helpers::crates("crates/libs") {
        let name = package.name;
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
