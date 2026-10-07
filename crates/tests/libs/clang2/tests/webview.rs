use windows_metadata::{
    Type,
    reader::{Index, Item, ParamDirection},
};

#[allow(dead_code)]
#[path = "../sdk.rs"]
mod sdk;

#[cfg(target_env = "msvc")]
#[allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    clippy::upper_case_acronyms,
    clippy::missing_transmute_annotations
)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/webview.rs"));
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
        let snapshot = sdk::capture_webview("--target=x86_64-pc-windows-msvc", &roots, swapped);
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
fn consumer_dependencies_project_across_targets_and_input_orders() {
    let roots = sdk::webview_roots();
    assert_eq!(roots.len(), 79);
    for target in [
        "--target=x86_64-pc-windows-msvc",
        "--target=i686-pc-windows-msvc",
        "--target=aarch64-pc-windows-msvc",
    ] {
        let mut expected = None;
        for swapped in [false, true] {
            let snapshot = sdk::capture_webview(target, &roots, swapped);
            let resolved = snapshot.resolve().unwrap();
            assert!(resolved.report().incomplete.is_empty());
            let rdl = resolved.project(&sdk::webview_options()).unwrap().rdl();
            std::fs::write(
                std::path::Path::new(env!("OUT_DIR")).join(format!(
                    "consumer-{}.rdl",
                    target.trim_start_matches("--target=")
                )),
                &rdl,
            )
            .unwrap();
            if let Some(expected) = &expected {
                assert_eq!(&rdl, expected, "{target}: swapped={swapped}");
            } else {
                expected = Some(rdl);
            }
        }
    }
}

#[cfg(target_env = "msvc")]
#[test]
fn consumer_input_contracts_cutover_gate() {
    let index = Index::read(std::path::Path::new(env!("OUT_DIR")).join("webview.winmd")).unwrap();
    let mut failures = vec![];
    for (owner, method, position) in [
        ("ICoreWebView2", "AddHostObjectToScript", 1),
        ("ICoreWebView2Controller", "put_ParentWindow", 0),
        (
            "ICoreWebView2Environment",
            "CreateCoreWebView2Controller",
            0,
        ),
        (
            "ICoreWebView2Environment10",
            "CreateCoreWebView2ControllerWithOptions",
            0,
        ),
        (
            "ICoreWebView2Environment10",
            "CreateCoreWebView2CompositionControllerWithOptions",
            0,
        ),
        (
            "ICoreWebView2Environment3",
            "CreateCoreWebView2CompositionController",
            0,
        ),
        (
            "ICoreWebView2Environment4",
            "GetAutomationProviderForWindow",
            0,
        ),
        ("ICoreWebView2Frame", "AddHostObjectToScriptWithOrigins", 1),
    ] {
        let Item::Type(ty) = index.expect_item("WebView2", owner) else {
            panic!()
        };
        let method = ty
            .methods()
            .find(|candidate| candidate.name() == method)
            .unwrap();
        let parameters = method
            .params_by_sequence(method.signature(&[]).types.len())
            .unwrap();
        let direction = parameters.params()[position].unwrap().direction();
        if direction != ParamDirection::Input {
            failures.push(format!(
                "{owner}::{} parameter {position}: {direction:?}",
                method.name()
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[cfg(target_env = "msvc")]
#[test]
fn consumer_wrappers_preserve_public_parameter_shapes() {
    use bindings::{HWND, ICoreWebView2, ICoreWebView2Controller, VARIANT};
    use windows_core::{HRESULT, PCWSTR, PWSTR, Result};
    let _: unsafe fn(&ICoreWebView2Controller, HWND) -> HRESULT =
        ICoreWebView2Controller::put_ParentWindow;
    let _: unsafe fn(&ICoreWebView2Controller) -> Result<HWND> =
        ICoreWebView2Controller::get_ParentWindow;
    let _: unsafe fn(&ICoreWebView2, PCWSTR, *const VARIANT) -> HRESULT =
        ICoreWebView2::AddHostObjectToScript::<PCWSTR>;
    let _: unsafe fn(&ICoreWebView2) -> Result<PWSTR> = ICoreWebView2::get_Source;
}
