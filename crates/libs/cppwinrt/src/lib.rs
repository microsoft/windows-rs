#![doc = include_str!("../readme.md")]
#![cfg(windows)]

const VERSION: &str = "2.0.240405.15";

/// Calls the C++/WinRT compiler with the given arguments.
///
/// Use `cppwinrt["-help"]` for available options.
#[track_caller]
pub fn cppwinrt<I, S>(args: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let mut path = std::env::temp_dir();
    path.push(format!("cppwinrt-{VERSION}.exe"));

    let bytes = std::include_bytes!("../cppwinrt.exe");

    // Concurrent builds can cause this to fail, so we just make sure the bytes match on failure.
    if std::fs::write(&path, bytes).is_err() {
        assert_eq!(*bytes, *std::fs::read(&path).unwrap());
    }

    let mut command = std::process::Command::new(&path);
    command.args(args);
    let output = command.output().expect("failed to run cppwinrt");
    _ = std::fs::remove_file(path);

    if output.status.success() {
        String::from_utf8_lossy(&output.stdout).to_string()
    } else {
        panic!("{}", String::from_utf8_lossy(&output.stderr))
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    #[should_panic(expected = "'-invalid' is not supported")]
    fn invalid_arg() {
        cppwinrt(["-invalid"]);
    }

    #[test]
    fn unexpected_version() {
        let ok = cppwinrt(["-help"]);
        assert!(ok.contains(VERSION), "unexpected version");
    }
}
