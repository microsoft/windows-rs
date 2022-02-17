fn main() {
    test_yml();
    build_yml();
}

fn test_yml() {
    let root = std::path::PathBuf::from(metadata::workspace_dir());
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
    runs-on: windows-latest

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
      run: rustup update --no-self-update ${{ matrix.version }} && rustup default ${{ matrix.version }}
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
    - name: Configure environment for GNU toolchain
      shell: pwsh
      run: |
        if("${{ matrix.target }}" -eq "i686-pc-windows-gnu") {
            $MingwPath = "C:\msys64\mingw32\bin"
        } else {
            $MingwPath = "C:\msys64\mingw64\bin"
        }
        $MingwPath | Out-File -FilePath $env:GITHUB_PATH -Encoding utf8 -Append
      if: contains(matrix.target, 'windows-gnu')
    - name: Test stable
      run: |"#
        .to_string();

    for name in crates(&root) {
        if !requires_nightly(&name) {
            yml.push_str(&format!("\n        cargo test --target ${{{{ matrix.target }}}} -p {} &&", name));
        }
    }

    yml.truncate(yml.len() - 2);

    yml.push_str(
        r#"
      if: matrix.version == 'stable'

    - name: Test nightly
      run: |"#,
    );

    for name in crates(&root) {
        if requires_nightly(&name) {
            yml.push_str(&format!("\n        cargo test --target ${{{{ matrix.target }}}} -p {} &&", name));
        }
    }

    yml.truncate(yml.len() - 2);

    yml.push_str(
        r#"
      if: matrix.version == 'nightly'

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

    std::fs::write(root.join(".github/workflows/test.yml"), yml.as_bytes()).unwrap();
}

fn build_yml() {
    let root = std::path::PathBuf::from(metadata::workspace_dir());
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
    runs-on: windows-latest
    steps:
    - name: Checkout
      uses: actions/checkout@v2
    - name: Run cargo fmt
      run: cargo fmt --all -- --check

  cargo_doc:
    name: Check cargo docs
    runs-on: windows-latest
    steps:
    - name: Checkout
      uses: actions/checkout@v2
    - name: Run cargo doc
      run: cargo doc --no-deps -p windows

  generation:
    name: Check generation of `tool_${{ matrix.generator }}`
    runs-on: windows-latest
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
    runs-on: windows-latest
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

  cargo_clippy:
    name: Check clippy
    runs-on: windows-latest
    steps:
    - name: Checkout
      uses: actions/checkout@v2
    - name: Update toolchain
      run: rustup update --no-self-update nightly && rustup default nightly
    - name: Install clippy
      run: rustup component add clippy      
    - name: Run cargo clippy
      run: |"#
        .to_string();

    for name in crates(&root) {
        yml.push_str(&format!("\n        cargo clippy -p {} &&", name));
    }

    yml.truncate(yml.len() - 2);

    std::fs::write(root.join(".github/workflows/build.yml"), yml.as_bytes()).unwrap();
}

fn crates(root: &std::path::Path) -> Vec<String> {
    let mut crates = vec![];

    for dir in dirs(root, "crates/libs") {
        if dir == "windows" {
            crates.push("windows".to_string());
        } else {
            crates.push(format!("windows-{}", dir));
        }
    }

    for dir in dirs(root, "crates/samples") {
        crates.push(format!("sample_{}", dir));
    }

    for dir in dirs(root, "crates/targets") {
        crates.push(format!("windows_{}", dir));
    }

    for dir in dirs(root, "crates/tests") {
        crates.push(format!("test_{}", dir));
    }

    for dir in dirs(root, "crates/tools") {
        crates.push(format!("tool_{}", dir));
    }

    crates
}

fn requires_nightly(name: &str) -> bool {
  name.contains("implement") || name.contains("nightly") || name.starts_with("sample")
}

fn dirs(root: &std::path::Path, path: &str) -> Vec<String> {
    let mut dirs = vec![];

    if let Ok(files) = std::fs::read_dir(root.join(path)) {
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
