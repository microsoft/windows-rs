use std::process::Command;

fn main() {
    let output = Command::new("cargo")
        .args(&["winrt", "install"])
        .output()
        .expect("failed to execute process");
    assert!(
        output.status.success(),
        "`cargo winrt install` failed\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
