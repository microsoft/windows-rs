//! Tests for the COFF import-library reader (`windows_rdl::implib`).
//!
//! The synthetic-archive tests construct minimal but valid archives in memory
//! so they run on every platform without the Windows SDK. An additional,
//! best-effort integration test reads a real `kernel32.lib` when the SDK is
//! installed.

use windows_rdl::implib::{self, Import};

/// Build a 60-byte archive member header followed by `data` (padded to an even
/// length, as archives require).
fn member(name: &str, data: &[u8]) -> Vec<u8> {
    let mut out = vec![b' '; 60];
    out[..name.len()].copy_from_slice(name.as_bytes());
    let size = data.len().to_string();
    out[48..48 + size.len()].copy_from_slice(size.as_bytes());
    out[58] = 0x60;
    out[59] = 0x0A;
    out.extend_from_slice(data);
    if data.len() % 2 == 1 {
        out.push(0x0A);
    }
    out
}

/// Build a short-import member body (`IMPORT_OBJECT_HEADER` + the two
/// null-terminated names) for `symbol` exported by `dll`.
fn short_import(symbol: &str, dll: &str) -> Vec<u8> {
    let mut strings = vec![];
    strings.extend_from_slice(symbol.as_bytes());
    strings.push(0);
    strings.extend_from_slice(dll.as_bytes());
    strings.push(0);

    let mut data = vec![];
    data.extend_from_slice(&[0x00, 0x00, 0xFF, 0xFF]); // Sig1 + Sig2
    data.extend_from_slice(&0u16.to_le_bytes()); // Version
    data.extend_from_slice(&0x8664u16.to_le_bytes()); // Machine (AMD64)
    data.extend_from_slice(&0u32.to_le_bytes()); // TimeDateStamp
    data.extend_from_slice(&(strings.len() as u32).to_le_bytes()); // SizeOfData
    data.extend_from_slice(&0u16.to_le_bytes()); // Ordinal/Hint
    data.extend_from_slice(&0x0004u16.to_le_bytes()); // Type: code import, name == symbol
    data.extend_from_slice(&strings);
    data
}

fn archive(members: &[Vec<u8>]) -> Vec<u8> {
    let mut out = b"!<arch>\n".to_vec();
    for m in members {
        out.extend_from_slice(m);
    }
    out
}

#[test]
fn reads_single_import() {
    let bytes = archive(&[member(
        "KERNEL32.dll/",
        &short_import("CreateFileW", "KERNEL32.dll"),
    )]);

    let imports = implib::read(&bytes).unwrap();
    assert_eq!(
        imports,
        vec![Import {
            symbol: "CreateFileW".to_string(),
            dll: "KERNEL32.dll".to_string(),
        }]
    );
}

#[test]
fn skips_bookkeeping_and_non_import_members() {
    // A realistic archive: the two linker members (`/`), the long-name table
    // (`//`), a full COFF object (the import descriptor — no short-import
    // signature), and two genuine short imports. Only the latter two count.
    let bytes = archive(&[
        member("/", &[0u8; 4]),
        member("/", &[0u8; 4]),
        member("//", b"KERNEL32.dll\0"),
        member("KERNEL32.dll/", &[0x64, 0x86, 0x01, 0x00, 0x11, 0x22]), // COFF object
        member(
            "KERNEL32.dll/",
            &short_import("CreateFileW", "KERNEL32.dll"),
        ),
        member(
            "KERNEL32.dll/",
            &short_import("CloseHandle", "KERNEL32.dll"),
        ),
    ]);

    let imports = implib::read(&bytes).unwrap();
    let names: Vec<&str> = imports.iter().map(|i| i.symbol.as_str()).collect();
    assert_eq!(names, ["CreateFileW", "CloseHandle"]);
    assert!(imports.iter().all(|i| i.dll == "KERNEL32.dll"));
}

#[test]
fn handles_odd_sized_member_padding() {
    // `AB` makes the first member's data an odd length, exercising the pad byte
    // between members; the following import must still parse.
    let bytes = archive(&[
        member("X/", &short_import("AB", "X.dll")),
        member("Y/", &short_import("CreateMutexW", "Y.dll")),
    ]);

    let imports = implib::read(&bytes).unwrap();
    assert_eq!(imports.len(), 2);
    assert_eq!(imports[0].symbol, "AB");
    assert_eq!(imports[0].dll, "X.dll");
    assert_eq!(imports[1].symbol, "CreateMutexW");
}

#[test]
fn long_name_member_is_not_skipped() {
    // A short import whose member name is stored in the long-name table appears
    // as `/<offset>`; it must be parsed, unlike the bare `/` and `//` members.
    let bytes = archive(&[member(
        "/42",
        &short_import("GetProcAddress", "KERNEL32.dll"),
    )]);

    let imports = implib::read(&bytes).unwrap();
    assert_eq!(imports.len(), 1);
    assert_eq!(imports[0].symbol, "GetProcAddress");
}

#[test]
fn rejects_non_archive() {
    assert!(implib::read(b"not an archive").is_err());
    assert!(implib::read(&[]).is_err());
}

/// Best-effort integration check against a real SDK import library. Skips
/// cleanly when the Windows SDK is not installed so it never fails on CI hosts
/// without it.
#[test]
#[cfg(windows)]
fn reads_real_kernel32_lib() {
    let Some(lib) = find_sdk_lib("kernel32.lib") else {
        eprintln!("skipping: no Windows SDK import library found");
        return;
    };

    let bytes = std::fs::read(&lib).unwrap();
    let imports = implib::read(&bytes).unwrap();

    assert!(
        imports.len() > 100,
        "expected many imports in kernel32.lib, got {}",
        imports.len()
    );
    let create_file = imports
        .iter()
        .find(|i| i.symbol == "CreateFileW")
        .expect("CreateFileW should be exported by kernel32.lib");
    assert!(create_file.dll.eq_ignore_ascii_case("kernel32.dll"));
}

#[cfg(windows)]
fn find_sdk_lib(name: &str) -> Option<std::path::PathBuf> {
    let root = std::path::Path::new(r"C:\Program Files (x86)\Windows Kits\10\Lib");
    let mut versions: Vec<_> = root.read_dir().ok()?.flatten().map(|e| e.path()).collect();
    versions.sort();
    versions
        .into_iter()
        .rev()
        .map(|version| version.join("um").join("x64").join(name))
        .find(|path| path.exists())
}
