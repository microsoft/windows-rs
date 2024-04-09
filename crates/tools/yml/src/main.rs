use std::fmt::Write;

fn main() {
    test_yml();
    clippy_yml();
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
  test:
    name: Test
    runs-on: windows-2019

    strategy:
      matrix:
        include:
          - version: stable
            target: x86_64-pc-windows-msvc
            no-run:
          - version: nightly
            target: i686-pc-windows-msvc
            no-run:
          - version: stable
            target: aarch64-pc-windows-msvc
            no-run: --no-run
          - version: nightly
            target: aarch64-pc-windows-msvc
            no-run: --no-run
          - version: nightly
            target: x86_64-pc-windows-gnu
            no-run:
          - version: stable
            target: i686-pc-windows-gnu
            no-run:

    steps:
      - name: Checkout
        uses: actions/checkout@v4
      - name: Add toolchain target
        run: rustup target add ${{ matrix.target }}
      - name: Update toolchain
        run: rustup update --no-self-update ${{ matrix.version }} && rustup default ${{ matrix.version }}
      - name: Fix environment
        uses: ./.github/actions/fix-environment
      - name: Test
        run: >"
        .to_string();

    let crates = lib::crates("crates");
    let (first, last) = crates.split_at(crates.len() / 2);

    for (name, _) in first {
        write!(&mut yml, "\n          cargo test ${{{{ matrix.no-run }}}} --target ${{{{ matrix.target }}}} -p {name} &&").unwrap();
    }

    write!(&mut yml, "\n          cargo clean &&").unwrap();

    for (name, _) in last {
        write!(&mut yml, "\n          cargo test ${{{{ matrix.no-run }}}} --target ${{{{ matrix.target }}}} -p {name} &&").unwrap();
    }

    yml.truncate(yml.len() - 3);

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
    let mut yml = r#"name: clippy

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
  cargo_clippy:
    name: Check
    runs-on: windows-2019
    steps:
      - name: Checkout
        uses: actions/checkout@v4
      - name: Update toolchain
        run: rustup update --no-self-update nightly && rustup default nightly-x86_64-pc-windows-msvc
      - name: Install clippy
        run: rustup component add clippy
      - name: Configure environment
        shell: pwsh
        run: |
          "C:\Program Files (x86)\Windows Kits\10\bin\10.0.22000.0\x64" >> $env:GITHUB_PATH
          ((Resolve-Path "C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\*\bin\Hostx64\x64")
            | Sort-Object -Descending | Select -First 1).ToString() >> $env:GITHUB_PATH
          (Join-Path $env:GITHUB_WORKSPACE "target\debug\deps").ToString() >> $env:GITHUB_PATH
          (Join-Path $env:GITHUB_WORKSPACE "target\test\debug\deps").ToString() >> $env:GITHUB_PATH
          "INCLUDE=C:\Program Files (x86)\Windows Kits\10\include\10.0.22000.0\winrt;C:\Program Files (x86)\Windows Kits\10\include\10.0.22000.0\cppwinrt" `
            >> $env:GITHUB_ENV
      - name: Run cargo clippy
        run: |"#
        .to_string();

    for (name, _) in lib::crates("crates") {
        write!(&mut yml, "\n          cargo clippy -p {name} &&").unwrap();
    }

    yml.truncate(yml.len() - 3);
    std::fs::write(".github/workflows/clippy.yml", yml.as_bytes()).unwrap();
}
