#![cfg(target_pointer_width = "64")]

use windows_rdl::*;

#[test]
fn roundtrip() {
    let paths = std::fs::read_dir("roundtrip")
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("h"));

    for h in paths {
        let rdl = h.with_extension("rdl").to_str().unwrap().to_string();
        let winmd = h.with_extension("winmd").to_str().unwrap().to_string();
        let h = h.to_str().unwrap().to_string();
        let reference = "../../../libs/bindgen/default";

        clang()
            .args([
                "-x",
                "c++",
                "--target=x86_64-pc-windows-msvc",
                "-fms-extensions",
            ])
            .input(&h)
            .input(reference)
            .output(&rdl)
            .namespace("Test")
            .library("test.dll")
            .write()
            .unwrap();

        reader()
            .input(&rdl)
            .input(reference)
            .output(&winmd)
            .write()
            .unwrap();

        writer()
            .input(&winmd)
            .input(reference)
            .output(&rdl)
            .filter("Test")
            .write()
            .unwrap();
    }
}

/// Verifies that `typedef struct _TAG {} ALIAS, *PALIAS;` correctly uses
/// `ALIAS` as the struct name in the generated RDL instead of `_TAG`, and
/// that pointer typedefs (like `PALIAS`) reference `ALIAS` not `_TAG`.
#[test]
fn typedef_struct_rename() {
    let output = "tests/typedef_struct_rename.rdl";
    windows_rdl::clang()
        .args(["-x", "c++"])
        .input_str(
            r#"
typedef struct _TEST
{
    int value;
} TEST, *PTEST;

struct TEST2
{
    TEST value;
    PTEST pointer;
};
"#,
        )
        .output(output)
        .namespace("Test")
        .write()
        .unwrap();

    let rdl = std::fs::read_to_string(output).unwrap();
    // The internal tag name must never appear in the RDL.
    assert!(
        !rdl.contains("_TEST"),
        "_TEST tag must not appear in the RDL; got:\n{rdl}"
    );
    // The public typedef alias must be used as the struct name.
    assert!(
        rdl.contains("struct TEST"),
        "struct TEST must be present in the RDL; got:\n{rdl}"
    );
    // The pointer typedef must reference the public alias, not the internal tag.
    assert!(
        rdl.contains("*mut TEST"),
        "PTEST must be typed as *mut TEST (not *mut _TEST); got:\n{rdl}"
    );
}
