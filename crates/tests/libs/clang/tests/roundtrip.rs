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

/// Verifies that a pure-virtual method preceded by a `/* ... [propget] ... */`
/// block comment emits `#[special]`, while methods with no such comment do not.
#[test]
fn propget() {
    let output = "tests/propget.rdl";
    windows_rdl::clang()
        .args([
            "-x",
            "c++",
            "--target=x86_64-pc-windows-msvc",
            "-fms-extensions",
        ])
        .input_str(
            r#"
struct __declspec(uuid("cd33ad7d-cb91-471d-a494-6a178012a31f"))
IFoo {
    virtual /* [id][helpstring][propget] */ unsigned int __stdcall get_Count() = 0;
    virtual /* [helpstring] */ unsigned int __stdcall get_Other() = 0;
    virtual unsigned int __stdcall get_NoComment() = 0;
};
"#,
        )
        .output(output)
        .namespace("Test")
        .write()
        .unwrap();

    let rdl = std::fs::read_to_string(output).unwrap();
    // get_Count has [propget] → must emit #[special].
    assert!(
        rdl.contains("#[special]"),
        "#[special] must appear in the RDL; got:\n{rdl}"
    );
    // get_Other has no [propget] → must not emit #[special] before get_Other.
    assert!(
        !rdl.contains("#[special]\n        fn get_Other"),
        "get_Other must not have #[special]; got:\n{rdl}"
    );
    // get_NoComment has no comment at all → must not emit #[special] before get_NoComment.
    assert!(
        !rdl.contains("#[special]\n        fn get_NoComment"),
        "get_NoComment must not have #[special]; got:\n{rdl}"
    );
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
