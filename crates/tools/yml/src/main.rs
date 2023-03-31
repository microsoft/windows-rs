use regex::Regex;
use std::fmt::Write;
use std::path::Path;

fn main() {
    test_yml();
    clippy_yml();
}

fn test_yml() {
    let mut yml = r#"name: test

on:
  pull_request:
  push:
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
          - version: nightly
            target: i686-pc-windows-msvc
          - version: nightly
            target: x86_64-pc-windows-gnu
          - version: stable
            target: i686-pc-windows-gnu

    steps:
      - name: Checkout
        uses: actions/checkout@v3
      - name: Update toolchain
        run: rustup update --no-self-update ${{ matrix.version }} && rustup default ${{ matrix.version }}-${{ matrix.target }}
      - name: Add toolchain target
        run: rustup target add ${{ matrix.target }}
      - name: Install clippy
        run: rustup component add clippy
      - name: Fix environment
        uses: ./.github/actions/fix-environment
      - name: Test
        run: >"#
        .to_string();

    let crates = crates(true);
    let (first, last) = crates.split_at(crates.len() / 2);

    for name in first {
        write!(&mut yml, "\n          cargo test -p {name} &&").unwrap();
    }

    write!(&mut yml, "\n          cargo clean &&").unwrap();

    for name in last {
        write!(&mut yml, "\n          cargo test -p {name} &&").unwrap();
    }

    yml.truncate(yml.len() - 3);
    std::fs::write(".github/workflows/test.yml", yml.as_bytes()).unwrap();
}

fn clippy_yml() {
    let mut yml = r#"name: clippy

on:
  pull_request:
  push:
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
        uses: actions/checkout@v3
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

    for name in crates(false) {
        write!(&mut yml, "\n          cargo clippy -p {name} &&").unwrap();
    }

    yml.truncate(yml.len() - 3);
    std::fs::write(".github/workflows/clippy.yml", yml.as_bytes()).unwrap();
}

fn crates(filter: bool) -> Vec<String> {
    let regex = Regex::new(r#"name = "([^"]+)""#).expect("regex");
    let mut names = find("crates", &regex, filter);
    names.sort();
    names
}

fn find<P: AsRef<Path>>(path: P, regex: &Regex, filter: bool) -> Vec<String> {
    let mut names = vec![];

    if let Ok(files) = std::fs::read_dir(path) {
        for file in files.filter_map(|file| file.ok()) {
            if let Ok(file_type) = file.file_type() {
                if file_type.is_dir() {
                    names.append(&mut find(file.path(), regex, filter));
                } else if file.file_name() == "Cargo.toml" {
                    let text = std::fs::read_to_string(file.path()).expect("Cargo.toml");
                    let name = regex.captures(&text).expect("captures").get(1).expect("name");
                    if !filter || !name.as_str().ends_with("_x") {
                        names.push(name.as_str().to_string());
                    }
                }
            }
        }
    }

    names
}
