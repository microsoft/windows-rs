fn main() {
    let root = std::path::PathBuf::from(reader::workspace_dir());
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
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        include:
        - os: windows-latest
          rust: stable
          other: x86_64-pc-windows-msvc
          platform: x64
        - os: windows-latest
          rust: nightly
          other: i686-pc-windows-msvc
          platform: x86
        - os: windows-latest
          rust: nightly
          other: x86_64-pc-windows-gnu
          platform: x64
        - os: windows-latest
          rust: stable
          other: i686-pc-windows-gnu
          platform: x86
    steps:
    - name: Checkout
      uses: actions/checkout@v2

    - name: Update toolchain
      run: rustup update --no-self-update ${{ matrix.rust }} && rustup default ${{ matrix.rust }}

    - name: Add toolchain target
      run: rustup target add ${{ matrix.other }}

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
      if: contains(matrix.other, 'windows-gnu')

    - name: Configure environment for GNU toolchain
      shell: pwsh
      run: |
        if("${{ matrix.other }}" -eq "i686-pc-windows-gnu") {
          $MingwPath = "C:\msys64\mingw32\bin"
        } else {
          $MingwPath = "C:\msys64\mingw64\bin"
        }
        $MingwPath | Out-File -FilePath $env:GITHUB_PATH -Encoding utf8 -Append
      if: contains(matrix.other, 'windows-gnu')

    - name: Test stable (${{ matrix.os }})
      run: |"#
        .to_string();

    for name in dirs(&root, "crates/tests") {
        if !name.starts_with("implement") {
            yml.push_str(&format!("\n        cargo test --target ${{{{ matrix.other }}}} -p test_{} &&", name));
        }
    }

    yml.truncate(yml.len() - 2);

    yml.push_str(
        r#"
      if: contains(matrix.rust, 'stable')

    - name: Test nightly (${{ matrix.os }})
      run: |"#,
    );

    for name in dirs(&root, "crates/tests") {
        if name.starts_with("implement") {
            yml.push_str(&format!("\n        cargo test --target ${{{{ matrix.other }}}} -p test_{} &&", name));
        }
    }

    for name in dirs(&root, "crates/samples") {
        yml.push_str(&format!("\n        cargo test --target ${{{{ matrix.other }}}} -p {} &&", name));
    }

    yml.truncate(yml.len() - 2);

    yml.push_str(
        r#"
      if: contains(matrix.rust, 'nightly')
"#,
    );

    std::fs::write(root.join(".github/workflows/test.yml"), yml.as_bytes()).unwrap();
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
