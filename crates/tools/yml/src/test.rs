use super::*;

pub fn yml() {
    let mut yml =
        r#"name: test

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
  test:
    strategy:
      matrix:
        include:
          - version: stable
            host: x86_64-pc-windows-msvc
            target: x86_64-pc-windows-msvc
            runner: windows-2025
          - version: nightly
            host: x86_64-pc-windows-msvc
            target: i686-pc-windows-msvc
            runner: windows-2025
          - version: nightly
            host: x86_64-pc-windows-gnu
            target: x86_64-pc-windows-gnu
            runner: windows-2025
          - version: stable
            host: x86_64-pc-windows-gnu
            target: i686-pc-windows-gnu
            runner: windows-2025
          - version: stable
            host: aarch64-pc-windows-msvc
            target: aarch64-pc-windows-msvc
            runner: windows-11-arm

    runs-on: ${{ matrix.runner }}

    steps:
      - name: Checkout
        uses: actions/checkout@v5
      - name: Update toolchain
        run: rustup update --no-self-update ${{ matrix.version }} && rustup default ${{ matrix.version }}-${{ matrix.host }}
      - name: Add toolchain target
        run: rustup target add ${{ matrix.target }}
      - name: Install fmt, clippy
        run: rustup component add clippy rustfmt
      - name: Fix environment
        uses: ./.github/actions/fix-environment
        with:
          target: ${{ matrix.target }}"#
    .to_string();

    // This unrolling is required since "cargo test --all" consumes too much memory for the GitHub hosted runners
    // and the occasional "cargo clean" is required to avoid running out of disk space in the same runners.

    for (count, manifest) in helpers::crates("crates").iter().enumerate() {
        let name = &manifest.package.name;
        if count.is_multiple_of(50) {
            write!(
                &mut yml,
                r"
      - name: Clean
        run:  cargo clean"
            )
            .unwrap();
        }

        write!(
            &mut yml,
            r"
      - name: Test {name}
        run:  cargo test -p {name} --target ${{{{ matrix.target }}}}"
        )
        .unwrap();
    }

    write!(
        &mut yml,
        r"
      - name: Check diff
        shell: bash
        run: |
          git add -N .
          git diff --exit-code || (echo 'Tests changed code in the repo.'; exit 1)
"
    )
    .unwrap();

    std::fs::write(".github/workflows/test.yml", yml.as_bytes()).unwrap();
}
