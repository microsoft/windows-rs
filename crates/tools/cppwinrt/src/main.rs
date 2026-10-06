use helpers::nuget_package;
use std::fs;
use std::path::Path;
use std::process::Command;

const CPPWINRT_VERSION: &str = "2.0.250303.1";

fn main() {
    let package = nuget_package("Microsoft.Windows.CppWinRT", CPPWINRT_VERSION);
    let compiler = package.join("bin/cppwinrt.exe");
    let output = Command::new(&compiler)
        .arg("-help")
        .output()
        .unwrap_or_else(|error| panic!("cannot run `{}`: {error}", compiler.display()));
    assert!(
        output.status.success(),
        "`{}` failed ({}): {}",
        compiler.display(),
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
    let help = String::from_utf8(output.stdout).unwrap();
    let expected = format!("C++/WinRT v{CPPWINRT_VERSION}");
    assert!(
        help.lines().any(|line| line == expected),
        "unexpected compiler version in `{}`: {help}",
        compiler.display()
    );

    let dest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../libs/cppwinrt");
    fs::copy(&compiler, dest.join("cppwinrt.exe"))
        .unwrap_or_else(|error| panic!("cannot copy `{}`: {error}", compiler.display()));
    fs::write(dest.join("version.txt"), CPPWINRT_VERSION).unwrap();
}
