use std::fmt::Write;

fn main() {
    test_yml();
    clippy_yml();
    no_default_features_yml();
    msrv_yml();
}

fn test_yml() {
    let mut yml = format!(
        r"name: test

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

    strategy:
      matrix:
        include:
          - version: stable
            host: x86_64-pc-windows-msvc
            target: x86_64-pc-windows-msvc
            etc:
          - version: nightly
            host: x86_64-pc-windows-msvc
            target: i686-pc-windows-msvc
            etc:
          - version: nightly
            host: x86_64-pc-windows-gnu
            target: x86_64-pc-windows-gnu
            etc:
          - version: stable
            host: x86_64-pc-windows-gnu
            target: i686-pc-windows-gnu
            etc:
          - version: stable
            host: x86_64-pc-windows-msvc
            target: aarch64-pc-windows-msvc
            etc: --no-run
          - version: nightly
            host: x86_64-pc-windows-msvc
            target: aarch64-pc-windows-msvc
            etc: --no-run

    steps:
      - name: Checkout
        uses: actions/checkout@v4
      - name: Update toolchain
        run: rustup update --no-self-update ${{{{ matrix.version }}}} && rustup default ${{{{ matrix.version }}}}-${{{{ matrix.host }}}}
      - name: Add toolchain target
        run: rustup target add ${{{{ matrix.target }}}}
      - name: Install fmt
        run: rustup component add rustfmt
      - name: Fix environment
        uses: ./.github/actions/fix-environment"
    );

    // This unrolling is required since "cargo test --all" consumes too much memory for the GitHub hosted runners
    // and the occasional "cargo clean" is required to avoid running out of disk space in the same runners.

    for (count, package) in helpers::crates("crates").iter().enumerate() {
        let name = &package.name;
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
        run:  cargo test -p {name} --target ${{{{ matrix.target }}}} ${{{{ matrix.etc }}}}"
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

    std::fs::write(format!(".github/workflows/test.yml"), yml.as_bytes()).unwrap();
}

fn clippy_yml() {
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

    for package in helpers::crates("crates/libs") {
        let name = package.name;
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

fn no_default_features_yml() {
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

fn msrv_yml() {
    let mut yml = r"name: msrv

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
        uses: actions/checkout@v4"
        .to_string();

    for package in helpers::crates("crates/libs") {
        let name = package.name;
        let version = package.rust_version.expect("rust-version");

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
