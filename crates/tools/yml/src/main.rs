use std::fmt::Write;

fn main() {
    test_yml();
    clippy_yml();
    arm64_yml();
}

fn test_yml() {
    let mut yml = r"name: test

on:
  pull_request:
  push:
    paths-ignore:
      - '.github/ISSUE_TEMPLATE/**'
    branches:
      - master

env:
  RUSTFLAGS: -Dwarnings

jobs:
  check:
    runs-on: windows-2019

    strategy:
      matrix:
        include:
          - version: stable
            target: x86_64-pc-windows-msvc
          - version: nightly
            target: i686-pc-windows-msvc
          - version: nightly
            target: x86_64-pc-windows-gnu
          - version: stable
            target: i686-pc-windows-gnu

    steps:
      - name: Checkout
        uses: actions/checkout@v4
      - name: Update toolchain
        run: rustup update --no-self-update ${{ matrix.version }} && rustup default ${{ matrix.version }}-${{ matrix.target }}
      - name: Add toolchain target
        run: rustup target add ${{ matrix.target }}
      - name: Install fmt
        run: rustup component add rustfmt
      - name: Fix environment
        uses: ./.github/actions/fix-environment"
        .to_string();

    // This unrolling is required since "cargo test --all" consumes too much memory for the GitHub hosted runners
    // and the occasional "cargo clean" is required to avoid running out of disk space in the same runners.

    for (count, (name, _)) in lib::crates("crates").iter().enumerate() {
        if count % 50 == 0 {
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
        run:  cargo test -p {name}"
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

fn clippy_yml() {
    let mut yml = r"name: clippy

on:
  pull_request:
  push:
    paths-ignore:
      - '.github/ISSUE_TEMPLATE/**'
    branches:
      - master

env:
  RUSTFLAGS: -Dwarnings

jobs:
  check:
    runs-on: windows-2019

    strategy:
      matrix:
        include:
          - version: nightly
            target: x86_64-pc-windows-msvc

    steps:
      - name: Checkout
        uses: actions/checkout@v4
      - name: Update toolchain
        run: rustup update --no-self-update ${{ matrix.version }} && rustup default ${{ matrix.version }}-${{ matrix.target }}
      - name: Add toolchain target
        run: rustup target add ${{ matrix.target }}
      - name: Install clippy
        run: rustup component add clippy
      - name: Fix environment
        uses: ./.github/actions/fix-environment"
        .to_string();

    // This unrolling is required since "cargo clippy --all" consumes too much memory for the GitHub hosted runners.

    for (name, _) in lib::crates("crates") {
        write!(
            &mut yml,
            r"
      - name: Clippy {name}
        run:  cargo clippy -p {name}"
        )
        .unwrap();
    }

    std::fs::write(".github/workflows/clippy.yml", yml.as_bytes()).unwrap();
}

// Ideally this would just be another matrix dimension in test_yml but Cargo limitations are blocking this for the time being.
// See https://github.com/rust-lang/cargo/issues/9661
fn arm64_yml() {
    let mut yml = r"name: arm64

on:
  pull_request:
  push:
    paths-ignore:
      - '.github/ISSUE_TEMPLATE/**'
    branches:
      - master

env:
  RUSTFLAGS: -Dwarnings

jobs:
  check:
    runs-on: windows-2019

    strategy:
      matrix:
        include:
          - version: stable
            target: aarch64-pc-windows-msvc
          - version: nightly
            target: aarch64-pc-windows-msvc

    steps:
      - name: Checkout
        uses: actions/checkout@v4
      - name: Update toolchain
        run: rustup update --no-self-update ${{ matrix.version }} && rustup default ${{ matrix.version }}-x86_64-pc-windows-msvc
      - name: Add toolchain target
        run: rustup target add ${{ matrix.target }}
      - name: Fix environment
        uses: ./.github/actions/fix-environment"
      .to_string();

    for (count, (name, _)) in lib::crates("crates").iter().enumerate() {
        if count % 50 == 0 {
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
        run:  cargo test -p {name} --no-run --target ${{{{ matrix.target }}}}"
        )
        .unwrap();
    }

    std::fs::write(".github/workflows/arm64.yml", yml.as_bytes()).unwrap();
}
