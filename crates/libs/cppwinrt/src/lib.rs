#![doc = include_str!("../readme.md")]
#![cfg(windows)]

#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    clippy::upper_case_acronyms
)]
mod bindings;

use bindings::*;

/// Calls the C++/WinRT compiler with the given arguments.
///
/// Use `cppwinrt(["-help"])` for available options.
#[track_caller]
pub fn cppwinrt<I, S>(args: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let mut path = std::env::temp_dir();
    path.push(unique());
    std::fs::create_dir_all(&path).unwrap();
    path.push("cppwinrt.exe");
    std::fs::write(&path, std::include_bytes!("../cppwinrt.exe")).unwrap();

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

fn unique() -> String {
    let mut guid = GUID::default();
    unsafe { CoCreateGuid(&mut guid) };

    format!(
        "{:08X?}-{:04X?}-{:04X?}-{:02X?}{:02X?}-{:02X?}{:02X?}{:02X?}{:02X?}{:02X?}{:02X?}",
        guid.data1,
        guid.data2,
        guid.data3,
        guid.data4[0],
        guid.data4[1],
        guid.data4[2],
        guid.data4[3],
        guid.data4[4],
        guid.data4[5],
        guid.data4[6],
        guid.data4[7]
    )
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
        assert!(ok.contains("2.0.250303.1"), "unexpected version");
    }
}
