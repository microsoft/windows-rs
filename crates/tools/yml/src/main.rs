fn main() {
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
        if !name.contains("implement") {
            yml.push_str(&format!("\n        cargo test --target ${{{{ matrix.other }}}} -p {} &&", name));
        }
    }

    yml.truncate(yml.len() - 2);

    yml.push_str(
        r#"
      if: matrix.rust == 'stable'

    - name: Test nightly
      run: |"#,
    );

    for name in crates(&root) {
        if name.contains("implement") {
            yml.push_str(&format!("\n        cargo test --target ${{{{ matrix.other }}}} -p {} &&", name));
        }
    }

    yml.truncate(yml.len() - 2);

    yml.push_str(
        r#"
      if: matrix.rust == 'nightly'

    - name: Test clippy
      run: |"#,
    );

    for name in crates(&root) {
        yml.push_str(&format!("\n        cargo clippy -p {} &&", name));
    }

    yml.truncate(yml.len() - 2);

    yml.push_str(
        r#"
      if: matrix.rust == 'nightly' && matrix.other == 'x86_64-pc-windows-msvc'
"#,
    );

    std::fs::write(root.join(".github/workflows/test.yml"), yml.as_bytes()).unwrap();
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
