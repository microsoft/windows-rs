use std::collections::{BTreeMap, BTreeSet};
use std::io::{prelude::*, SeekFrom};
use std::process::{Command, Stdio};
use std::str::FromStr;

fn main() {
    const ALL_PLATFORMS: [&str; 5] = [
        "x86_64_gnu",
        "i686_gnu",
        "x86_64_gnullvm",
        "aarch64_gnullvm",
        "i686_gnullvm",
    ];
    let mut platforms = BTreeSet::new();
    for platform in std::env::args().skip(1) {
        if ALL_PLATFORMS.contains(&&*platform) {
            platforms.insert(platform);
        } else if platform == "all" {
            platforms.extend(ALL_PLATFORMS.iter().map(|s| s.to_string()));
        } else {
            eprintln!("Unknown platform: {platform}");
            return;
        }
    }
    if platforms.is_empty() {
        eprintln!("Please specify at least one platform or use 'all' argument");
        return;
    };

    for platform in platforms {
        let tools = if platform.ends_with("_gnu") {
            &["dlltool", "ar", "objcopy"][..]
        } else {
            &["llvm-dlltool", "llvm-ar"][..]
        };
        for tool in tools {
            if Command::new(tool)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .is_err()
            {
                eprintln!("Could not find {tool}. Is it in your $PATH?");
                std::process::exit(1);
            }
        }

        build_platform(&platform, tools[0], tools[1]);
    }
}

fn build_platform(platform: &str, dlltool: &str, ar: &str) {
    println!("Platform: {platform}");

    let libraries = lib::libraries();
    let output = std::path::PathBuf::from(format!("crates/targets/{platform}/lib"));
    std::fs::create_dir_all(&output).unwrap();
    std::fs::create_dir_all(&output).unwrap();

    for (library, functions) in &libraries {
        build_library(&output, dlltool, library, functions, platform);
    }

    build_mri(&output, ar, &libraries);

    for library in libraries.keys() {
        std::fs::remove_file(output.join(format!("lib{library}.a"))).unwrap();
    }
}

fn build_library(
    output: &std::path::Path,
    dlltool: &str,
    library: &str,
    functions: &BTreeMap<String, lib::CallingConvention>,
    platform: &str,
) {
    println!("{library}");

    // Note that we don't use set_extension as it confuses PathBuf when the library name includes a period.

    let def_path = output.join(format!("{library}.def"));
    let mut def = std::fs::File::create(def_path).unwrap();

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
        match calling_convention {
            lib::CallingConvention::Stdcall(params) if platform.starts_with("i686_gnu") => def
                .write_all(format!("{function}@{params} @ 0\n").as_bytes())
                .unwrap(),
            _ => def
                .write_all(format!("{function} @ 0\n").as_bytes())
                .unwrap(),
        }
    }

    drop(def);

    let mut cmd = Command::new(dlltool);
    cmd.current_dir(output);

    if platform.starts_with("i686_gnu") {
        cmd.arg("-k");
    }

    cmd.arg("-d");
    cmd.arg(format!("{library}.def"));
    cmd.arg("-l");
    cmd.arg(format!("lib{library}.a"));
    cmd.arg("-m");
    cmd.arg(match platform {
        "i686_gnu" | "i686_gnullvm" => "i386",
        "x86_64_gnu" | "x86_64_gnullvm" => "i386:x86-64",
        "aarch64_gnullvm" => "arm64",
        _ => unreachable!(),
    });
    if dlltool == "dlltool" {
        // Work around https://sourceware.org/bugzilla/show_bug.cgi?id=29497
        cmd.arg("-f");
        cmd.arg(match platform {
            "i686_gnu" => "--32",
            "x86_64_gnu" => "--64",
            _ => unreachable!(),
        });
        // Ensure consistency in the prefixes used by dlltool.
        cmd.arg("-t");
        cmd.arg(format!("{library}_").replace('.', "_").replace('-', "_"));
        // Ensure deterministic output. (dlltool might be built with DEFAULT_AR_DETERMINISTIC=0)
        cmd.arg("--deterministic-libraries");
    }
    cmd.output().unwrap();

    if dlltool == "dlltool" {
        make_reproducible(output, library);
    }
    std::fs::remove_file(output.join(format!("{library}.def"))).unwrap();
}

/// Work around lack of determinism in dlltool output, and at the same time remove
/// unnecessary sections, hints, and symbols.
fn make_reproducible(output: &std::path::Path, library: &str) {
    let lib_path = output.join(format!("lib{library}.a"));
    let tmp_path = output.join("tmp.a");
    std::fs::rename(&lib_path, &tmp_path).unwrap();

    let mut cmd = Command::new("objcopy");
    cmd.current_dir(output);
    cmd.arg("--remove-section=.bss");
    cmd.arg("--remove-section=.data");
    cmd.arg("--strip-unneeded-symbol=fthunk");
    cmd.arg("--strip-unneeded-symbol=hname");
    cmd.arg("--strip-unneeded-symbol=.file");
    cmd.arg("--strip-unneeded-symbol=.text");
    cmd.arg("--strip-unneeded-symbol=.data");
    cmd.arg("--strip-unneeded-symbol=.bss");
    cmd.arg("--strip-unneeded-symbol=.idata$7");
    cmd.arg("--strip-unneeded-symbol=.idata$5");
    cmd.arg("--strip-unneeded-symbol=.idata$4");
    cmd.arg("tmp.a");
    cmd.arg(format!("lib{library}.a"));
    cmd.output().unwrap();

    std::fs::remove_file(tmp_path).unwrap();

    let mut archive = std::fs::File::options()
        .read(true)
        .write(true)
        .open(&lib_path)
        .unwrap();
    let len = archive.metadata().unwrap().len();
    let mut header = [0u8; 8];
    archive.read_exact(&mut header).unwrap();
    assert_eq!(&header, b"!<arch>\n");
    loop {
        let mut buf = [0u8; 32];

        //
        // Archive member header
        //

        // Name
        archive.read_exact(&mut buf[..16]).unwrap();
        let skip_member = match &buf[..4] {
            b"/   " => true, // Linker member
            b"//  " => true, // Long names table
            _ => false,
        };

        // Timestamp, User ID, Group ID, and Mode
        archive.seek(SeekFrom::Current(12 + 6 + 6 + 8)).unwrap();

        // Size, sans header
        archive.read_exact(&mut buf[..10]).unwrap();
        let size = i64::from_str(
            std::str::from_utf8(buf.split(u8::is_ascii_whitespace).next().unwrap()).unwrap(),
        )
        .unwrap();

        // End of archive member header marker
        archive.read_exact(&mut buf[..2]).unwrap();
        assert_eq!(&buf[..2], [0x60, 0x0A]);

        if !skip_member {
            let start = archive.stream_position().unwrap();

            // Machine type
            archive.read_exact(&mut buf[..2]).unwrap();

            if buf[..2] != [0x00, 0x00] {
                //
                // COFF File Header
                //

                // Section count
                archive.read_exact(&mut buf[..2]).unwrap();
                let sections = u16::from_le_bytes((&buf[..2]).try_into().unwrap());

                // Timestamp + symbol table offset + rest
                archive.seek(SeekFrom::Current(4 + 4 + 8)).unwrap();

                for _ in 1..sections {
                    //
                    // Section Header
                    //

                    // Name
                    archive.read_exact(&mut buf[..8]).unwrap();
                    let is_idata6 = std::str::from_utf8(&buf[..8]).unwrap() == ".idata$6";

                    // Virtual size and address
                    archive.seek(SeekFrom::Current(8)).unwrap();

                    // Size of data
                    archive.read_exact(&mut buf[..4]).unwrap();
                    let size_of_data = u32::from_le_bytes((&buf[..4]).try_into().unwrap());

                    // Pointer to raw data
                    archive.read_exact(&mut buf[..4]).unwrap();
                    let start_of_data = u32::from_le_bytes((&buf[..4]).try_into().unwrap())
                        + u32::try_from(start).unwrap();

                    // ... and rest
                    archive.seek(SeekFrom::Current(16)).unwrap();

                    if is_idata6 {
                        //
                        // Hint and Name table
                        //

                        let last_position = archive.stream_position().unwrap();
                        archive.seek(SeekFrom::Start(start_of_data.into())).unwrap();

                        let end_of_data = u64::from(start_of_data) + u64::from(size_of_data);
                        while archive.stream_position().unwrap() < end_of_data {
                            // Hint
                            archive.write_all(&[0u8; 2]).unwrap();

                            // Name
                            loop {
                                archive.read_exact(&mut buf[..1]).unwrap();
                                if buf[0] == b'\0' {
                                    break;
                                }
                            }

                            // Padding
                            if archive.stream_position().unwrap() % 2 == 0 {
                                archive.seek(SeekFrom::Current(1)).unwrap();
                            }
                        }

                        archive.seek(SeekFrom::Start(last_position)).unwrap();
                        break;
                    }
                }
            }

            archive.seek(SeekFrom::Start(start)).unwrap();
        }

        if archive.seek(SeekFrom::Current((size + 1) & !1)).unwrap() >= len {
            break;
        }
    }
    archive.flush().unwrap();
    drop(archive);
}

fn build_mri(
    output: &std::path::Path,
    ar: &str,
    libraries: &BTreeMap<String, BTreeMap<String, lib::CallingConvention>>,
) {
    let mri_path = output.join("unified.mri");
    let mut mri = std::fs::File::create(&mri_path).unwrap();
    println!("Generating {}", mri_path.to_string_lossy());

    mri.write_all(format!("CREATE libwindows.{}.a\n", std::env!("CARGO_PKG_VERSION")).as_bytes())
        .unwrap();

    for library in libraries.keys() {
        mri.write_all(format!("ADDLIB lib{library}.a\n").as_bytes())
            .unwrap();
    }

    mri.write_all(b"SAVE\nEND\n").unwrap();

    let mut cmd = Command::new(ar);
    cmd.current_dir(output);
    cmd.arg("-M");
    cmd.stdin(std::fs::File::open(&mri_path).unwrap());
    cmd.output().unwrap();

    std::fs::remove_file(&mri_path).unwrap();
}
