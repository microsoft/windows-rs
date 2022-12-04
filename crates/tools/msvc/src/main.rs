use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::str::FromStr;

fn main() {
    let platform = if let Some(platform) = option_env!("Platform") {
        match platform {
            "x86" => "i686_msvc",
            "x64" => "x86_64_msvc",
            "arm64" => "aarch64_msvc",
            _ => {
                println!("Unknown platform");
                return;
            }
        }
    } else {
        println!("Please run this tool from a Visual Studio command prompt");
        return;
    };

    let libraries = lib::libraries();
    let output = std::path::PathBuf::from("crates/targets/baseline");
    let _ = std::fs::remove_dir_all(&output);
    std::fs::create_dir_all(&output).unwrap();

    for (library, functions) in &libraries {
        build_library(&output, library, functions);
    }

    let mut cmd = std::process::Command::new("lib");
    cmd.current_dir(&output);
    cmd.arg("/nologo");
    cmd.arg("/out:windows.lib");

    for library in libraries.keys() {
        cmd.arg(format!("{library}.lib"));
    }

    cmd.output().unwrap();

    for library in libraries.keys() {
        std::fs::remove_file(output.join(format!("{library}.lib"))).unwrap();
    }

    // Clear out timestamps in the resulting library.
    let mut archive = std::fs::File::options().read(true).write(true).open(output.join("windows.lib")).unwrap();
    let len = archive.metadata().unwrap().len();
    let mut header = [0u8; 8];
    archive.read_exact(&mut header).unwrap();
    assert_eq!(&header, b"!<arch>\n");
    for num in 1.. {
        archive.seek(SeekFrom::Current(16)).unwrap(); // identifier
        assert_eq!(archive.write(b"-1          ").unwrap(), 12); // replace archive timestamp
        let mut buf = [0u8; 32];
        archive.read_exact(&mut buf).unwrap(); // remainder of the archive member header
        let mut size = i64::from_str(std::str::from_utf8(buf[20..][..10].split(u8::is_ascii_whitespace).next().unwrap()).unwrap()).unwrap();
        // The first two members are indexes, skip them.
        if num > 3 {
            archive.read_exact(&mut buf[..4]).unwrap(); // member header
            let timestamp_offset = if buf[..4] == [0, 0, 0xff, 0xff] { 4 } else { 0 };
            archive.seek(SeekFrom::Current(timestamp_offset)).unwrap();
            archive.write_all(&[0; 4]).unwrap(); // replace member timestamp
            size -= timestamp_offset + 8;
        }
        if archive.seek(SeekFrom::Current((size + 1) & !1)).unwrap() >= len {
            break;
        }
    }

    std::fs::rename(output.join("windows.lib"), format!("crates/targets/{platform}/lib/windows.lib")).unwrap();
}

fn build_library(output: &std::path::Path, library: &str, functions: &BTreeMap<String, lib::CallingConvention>) {
    println!("{library}");

    // Note that we don't use set_extension as it confuses PathBuf when the library name includes a period.

    let mut path = std::path::PathBuf::from(output);
    path.push(format!("{library}.c"));
    let mut c = std::fs::File::create(&path).unwrap();

    path.pop();
    path.push(format!("{library}.def"));
    let mut def = std::fs::File::create(&path).unwrap();

    def.write_all(
        format!(
            r#"
LIBRARY {library}
EXPORTS
"#
        )
        .as_bytes(),
    )
    .unwrap();

    for (function, calling_convention) in functions {
        let buffer = match calling_convention {
            lib::CallingConvention::Stdcall(size) => {
                let mut buffer = format!("void __stdcall {function}(");

                for param in 0..(*size / 4) {
                    use std::fmt::Write;
                    write!(&mut buffer, "int p{param}, ").unwrap();
                }

                if buffer.ends_with(' ') {
                    buffer.truncate(buffer.len() - 2);
                }

                buffer.push_str(") {}\n");
                buffer
            }
            lib::CallingConvention::Cdecl => {
                format!("void __cdecl {function}() {{}}\n")
            }
        };

        c.write_all(buffer.as_bytes()).unwrap();
        def.write_all(format!("{function}\n").as_bytes()).unwrap();
    }

    drop(c);
    drop(def);

    let mut cmd = std::process::Command::new("cl");
    cmd.current_dir(output);
    cmd.arg("/nologo");
    cmd.arg("/c");
    cmd.arg(format!("{library}.c"));
    cmd.output().unwrap();

    let mut cmd = std::process::Command::new("lib");
    cmd.current_dir(output);
    cmd.arg("/nologo");
    cmd.arg(format!("/out:{library}.lib"));
    cmd.arg(format!("/def:{library}.def"));
    cmd.arg(format!("{library}.obj"));
    cmd.output().unwrap();

    path.pop();
    path.push(format!("{library}.c"));
    // Don't remove the .c file as it represents the baseline.

    path.pop();
    path.push(format!("{library}.def"));
    std::fs::remove_file(&path).unwrap();

    path.pop();
    path.push(format!("{library}.exp"));
    std::fs::remove_file(&path).unwrap_or_else(|_| panic!("{:?}", path));

    path.pop();
    path.push(format!("{library}.obj"));
    std::fs::remove_file(&path).unwrap();
}
