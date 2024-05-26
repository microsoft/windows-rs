/*!
Learn more about Rust for Windows here: <https://github.com/microsoft/windows-rs>
*/

const VERSION: &str = "2.0.240405.15";

/// Calls the C++/WinRT compiler with the given arguments.
///
/// Use `cppwinrt["-help"]` for available options.
pub fn cppwinrt<I, S>(args: I) -> Result<String, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let mut path = std::env::temp_dir();
    path.push(format!("cppwinrt-{VERSION}.exe"));

    std::fs::write(&path, std::include_bytes!("../cppwinrt.exe")).unwrap();
    let mut command = std::process::Command::new(&path);
    command.args(args);
    let output = command.output().expect("failed to run cppwinrt");
    _ = std::fs::remove_file(path);

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test() {
        let ok = cppwinrt(["-help"]).unwrap();
        assert!(ok.contains(VERSION), "unexpected version");

        let err = cppwinrt(["-invalid"]).unwrap_err();
        assert!(err.contains("'-invalid' is not supported"));
    }
}
