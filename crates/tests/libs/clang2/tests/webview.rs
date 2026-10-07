use windows_clang2::{Input, capture};
use windows_metadata::{
    Type,
    reader::{Index, Item},
};

#[allow(dead_code)]
#[path = "../sdk.rs"]
mod sdk;

fn snapshot(roots: &[&str], swapped: bool) -> windows_clang2::Snapshot {
    let headers = sdk::webview_headers();
    let mut arguments = sdk::arguments("--target=x86_64-pc-windows-msvc");
    for header in headers {
        arguments.push(format!("-I{}", header.parent().unwrap().display()));
    }
    let mut inputs = [
        Input::new(
            if swapped { "z.hpp" } else { "a.hpp" },
            "#include <WebView2.h>",
        ),
        Input::new(
            if swapped { "a.hpp" } else { "z.hpp" },
            "#include <WebView2Interop.h>",
        ),
    ];
    if swapped {
        inputs.reverse();
    }
    capture(
        inputs,
        &arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        roots,
    )
    .unwrap()
}

#[test]
fn core_interfaces_agree_across_main_and_interop_tus() {
    let roots = [
        "ICoreWebView2Deferral",
        "ICoreWebView2StringCollection",
        "ICoreWebView2HttpHeadersCollectionIterator",
    ];
    let options = sdk::webview_options();
    let out = std::path::Path::new(env!("OUT_DIR")).join("webview-test");
    std::fs::create_dir_all(&out).unwrap();
    let reference = out.join("reference.winmd");
    windows_rdl::reader()
        .input_text(include_str!("../input/webview_reference.rdl"))
        .reference_default()
        .output(&reference)
        .write()
        .unwrap();
    for swapped in [false, true] {
        let snapshot = snapshot(&roots, swapped);
        let resolved = snapshot.resolve().unwrap();
        assert_eq!(resolved.group_count(), 17);
        assert_eq!(resolved.report().observations, 48);
        let rdl = resolved.project(&options).unwrap().rdl();
        let expected = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("expected")
            .join("webview.rdl");
        if std::env::var_os("UPDATE_EXPECT").is_some() {
            std::fs::write(&expected, &rdl).unwrap();
        }
        assert_eq!(rdl, std::fs::read_to_string(expected).unwrap());
        let winmd = out.join("test.winmd");
        windows_rdl::reader()
            .input_text(&rdl)
            .reference_default()
            .reference(&reference)
            .output(&winmd)
            .write()
            .unwrap();
        let index = Index::read(winmd).unwrap();
        for (root, count) in roots.iter().zip([1, 2, 3]) {
            let Item::Type(ty) = index.expect_item("WebView2", root) else {
                panic!()
            };
            assert_eq!(ty.methods().count(), count);
            assert_eq!(
                ty.interface_impls().next().unwrap().interface(&[]),
                Type::class_named("Windows.Win32.System.Com", "IUnknown")
            );
            for method in ty.methods() {
                assert_eq!(
                    method.signature(&[]).return_type,
                    Type::value_named("Windows.Win32.Foundation", "HRESULT")
                );
            }
        }
    }
}

#[test]
fn interop_dependency_reports_unsupported_native_evidence() {
    let snapshot = snapshot(&["ICoreWebView2Interop2"], false);
    let error = snapshot.resolve().err().unwrap().to_string();
    assert!(error.contains("`tagVARIANT`"), "{error}");
    assert!(
        error.contains("anonymous aggregate capture is not implemented"),
        "{error}"
    );
}
