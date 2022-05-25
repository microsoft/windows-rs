use std::fmt::Write;

fn main() {
    test_yml();
    build_yml();
}

fn test_yml() {
    let mut yml = r#"name: Test

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
      uses: actions/checkout@v2
    - name: Update toolchain
      run: rustup update --no-self-update ${{ matrix.version }} && rustup default ${{ matrix.version }}-${{ matrix.target }}
    - name: Add toolchain target
      run: rustup target add ${{ matrix.target }}
    - name: Install clippy
      run: rustup component add clippy      
    - name: Configure Cargo for GNU toolchain
      shell: pwsh
      run: |
        Add-Content $env:USERPROFILE\.cargo\config @"
            [target.x86_64-pc-windows-gnu]
            linker = `"C:\\msys64\\mingw64\\bin\\x86_64-w64-mingw32-gcc.exe`"
            ar = `"C:\\msys64\\mingw64\\bin\\ar.exe`"
            [target.i686-pc-windows-gnu]
            linker = `"C:\\msys64\\mingw32\\bin\\i686-w64-mingw32-gcc.exe`"
            ar = `"C:\\msys64\\mingw32\\bin\\ar.exe`"
        "@
      if: contains(matrix.target, 'windows-gnu')
    - name: Configure environment
      shell: pwsh
      run: |
        switch -Wildcard ("${{ matrix.target }}")
        {
          "i686-pc-windows-gnu"
          {
            "C:\msys64\mingw32\bin" >> $env:GITHUB_PATH
          }
          "x86_64-pc-windows-gnu"
          {
            "C:\msys64\mingw64\bin" >> $env:GITHUB_PATH
          }
          "i686*"
          {
            "C:\Program Files (x86)\Windows Kits\10\bin\10.0.22000.0\x86" >> $env:GITHUB_PATH
            ((Resolve-Path "C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\*\bin\Hostx86\x86")
              | Sort-Object -Descending | Select -First 1).ToString() >> $env:GITHUB_PATH
          }
          "x86_64*"
          {
            "C:\Program Files (x86)\Windows Kits\10\bin\10.0.22000.0\x64" >> $env:GITHUB_PATH
            ((Resolve-Path "C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\VC\Tools\MSVC\*\bin\Hostx64\x64")
              | Sort-Object -Descending | Select -First 1).ToString() >> $env:GITHUB_PATH
          }
          "*"
          {
            (Join-Path $env:GITHUB_WORKSPACE "target\debug\deps").ToString() >> $env:GITHUB_PATH
            (Join-Path $env:GITHUB_WORKSPACE "target\test\debug\deps").ToString() >> $env:GITHUB_PATH
            "INCLUDE=C:\Program Files (x86)\Windows Kits\10\include\10.0.22000.0\winrt;C:\Program Files (x86)\Windows Kits\10\include\10.0.22000.0\cppwinrt" `
              >> $env:GITHUB_ENV
          }
        }
    - name: Test
      run: >"#
        .to_string();

    let crates = crates();
    let (first, last) = crates.split_at(crates.len() / 2);

    for name in first {
        write!(&mut yml, "\n        cargo test --target ${{{{ matrix.target }}}} -p {} &&", name).unwrap();
    }

    write!(&mut yml, "\n        cargo clean &&").unwrap();

    for name in last {
        write!(&mut yml, "\n        cargo test --target ${{{{ matrix.target }}}} -p {} &&", name).unwrap();
    }

    yml.truncate(yml.len() - 2);

    yml.push_str(
        r#"

    - name: Check import libs
      shell: pwsh
      run: |
        $VisualStudioRoot = & vswhere -latest -property installationPath -format value
        $DumpbinPath = Resolve-Path "$VisualStudioRoot\VC\Tools\MSVC\*\bin\*\x86\dumpbin.exe" |
          Select -ExpandProperty Path -First 1
        $Tests = @(
          [Tuple]::Create("aarch64_msvc","AA64"),
          [Tuple]::Create("x86_64_msvc","8664"),
          [Tuple]::Create("i686_msvc","14C")
        )
        foreach($Test in $Tests) {
          $Target = $Test.Item1
          $Magic = $Test.Item2
          $Output = [string](& $DumpbinPath /headers crates/targets/$Target/lib/windows.lib)
          if($Output -match "Machine\s*: $Magic" -ne $True) {
            Write-Error "Import lib check failed for $Target ($Magic)."
            Exit 1
          }
        }
      if: matrix.version == 'stable' && matrix.target == 'x86_64-pc-windows-msvc'
    "#,
    );

    std::fs::write(".github/workflows/test.yml", yml.as_bytes()).unwrap();
}

fn build_yml() {
    let mut yml = r#"name: Build

on:
  pull_request:
  push:
    branches:
      - master

env:
  RUSTFLAGS: -Dwarnings

jobs:
  
  cargo_fmt:
    name: Check cargo formatting
    runs-on: windows-2019
    steps:
    - name: Checkout
      uses: actions/checkout@v2
    - name: Run cargo fmt
      run: cargo fmt --all -- --check

  cargo_doc:
    name: Check cargo docs
    runs-on: windows-2019
    steps:
    - name: Checkout
      uses: actions/checkout@v2
    - name: Run cargo doc
      run: cargo doc --no-deps -p windows

  generation:
    name: Check generation of `tool_${{ matrix.generator }}`
    runs-on: windows-2019
    strategy:
      matrix:
        generator: [bindings, windows, sys, yml]
    steps:
    - name: Checkout
      uses: actions/checkout@v2
    - name: Run tool_${{ matrix.generator }}
      run: cargo run -p tool_${{ matrix.generator }}
    - name: Compare
      shell: bash
      run: git diff --exit-code || (echo '::error::Generated `tool_${{ matrix.generator }}` are out-of-date. Please run `cargo run -p tool_${{ matrix.generator }}`'; exit 1)

  cargo_sys:
    name: Check windows-sys
    runs-on: windows-2019
    strategy:
      matrix:
        rust: [1.46.0, stable, nightly]
    steps:
    - name: Checkout
      uses: actions/checkout@v2
    - name: Update toolchain
      run: rustup update --no-self-update ${{ matrix.rust }} && rustup default ${{ matrix.rust }}
    - name: Run cargo check
      run: cargo check -p windows-sys --all-features

  cargo_windows:
    name: Check windows
    runs-on: windows-2019
    strategy:
      matrix:
        rust: [1.59.0, stable, nightly]
    steps:
    - name: Checkout
      uses: actions/checkout@v2
    - name: Update toolchain
      run: rustup update --no-self-update ${{ matrix.rust }} && rustup default ${{ matrix.rust }}
    - name: Run cargo check
      run: cargo check -p windows --features Foundation,Win32_Foundation,Win32_Graphics_Direct2D

  cargo_clippy:
    name: Check clippy
    runs-on: windows-2019
    steps:
    - name: Checkout
      uses: actions/checkout@v2
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

    for name in crates() {
        write!(&mut yml, "\n        cargo clippy -p {} &&", name).unwrap();
    }

    yml.truncate(yml.len() - 2);

    std::fs::write(".github/workflows/build.yml", yml.as_bytes()).unwrap();
}

fn crates() -> Vec<String> {
    let mut crates = vec![];

    for dir in dirs("crates/libs") {
        if dir == "windows" {
            crates.push("windows".to_string());
        } else {
            crates.push(format!("windows-{}", dir));
        }
    }

    for dir in dirs("crates/samples") {
        crates.push(format!("sample_{}", dir));
    }

    for dir in dirs("crates/targets") {
        crates.push(format!("windows_{}", dir));
    }

    for dir in dirs("crates/tests") {
        crates.push(format!("test_{}", dir));
    }

    for dir in dirs("crates/tools") {
        crates.push(format!("tool_{}", dir));
    }

    crates
}

fn dirs(path: &str) -> Vec<String> {
    let mut dirs = vec![];

    if let Ok(files) = std::fs::read_dir(path) {
        for file in files.filter_map(|file| file.ok()) {
            if let Ok(file_type) = file.file_type() {
                if file_type.is_dir() {
                    dirs.push(file.file_name().to_str().unwrap().to_string());
                }
            }
        }
    }

    dirs
}
