//! Downstream validation for the in-house Win32 metadata.
//!
//! `tool-win32` scrapes the Windows SDK headers into the committed
//! `metadata/win32/*.rdl` (flat `Windows.Win32` namespace, one file per
//! defining header). This test feeds that committed RDL back through
//! `windows-bindgen` for a bounded, self-contained slice and writes the result
//! to `expected/slice.rs`, which `build.rs` then compile-checks. It keeps header
//! surface growth honest: if a scrape change produces metadata that no longer
//! lowers to compilable bindings, this fails on the regenerated golden.
//!
//! The slice covers native alignment, packed containment, void typedefs, opaque handles and
//! imported functions. DispatcherQueue metadata is checked separately to retain its WinRT
//! reference without adding a CoreMessaging runtime dependency.

/// Regenerate `expected/slice.rs` from the committed `metadata/win32` RDL.
#[test]
fn slice() {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let rdl_dir = format!("{manifest}/../../../../metadata/win32");
    let seed = format!("{manifest}/../../../../metadata/metadata.rdl");
    // The RDL carries cross-winmd references (WinRT interop APIs name true `Windows.*` types),
    // so `Windows.winmd` is supplied as a resolution reference, as in the native pipeline.
    let winrt = format!("{manifest}/../../../../crates/libs/default/Windows.winmd");
    let scratch = format!("{}/win32", env!("OUT_DIR"));
    std::fs::create_dir_all(&scratch).unwrap();

    // Compile the whole committed RDL to one winmd (no libclang/SDK needed). The
    // hand-authored seed (which defines the `Windows.Win32.Metadata` attribute types the
    // RDL references) lives outside `rdl_dir`, so it is fed in explicitly.
    let winmd = format!("{scratch}/Windows.Win32.winmd");
    windows_rdl::reader()
        .input(&rdl_dir)
        .input(&seed)
        .reference(&winrt)
        .output(&winmd)
        .write()
        .unwrap();

    // Lower the bounded slice to flat `--sys` bindings, written straight to the
    // golden so `git diff` surfaces any change.
    let out_rs = format!("{manifest}/expected/slice.rs");
    let mut cli: Vec<String> = vec!["--in".into(), winmd.clone(), "--out".into(), out_rs];
    cli.push("--filter".into());
    cli.extend(FILTER.iter().map(|name| (*name).to_string()));
    cli.push("--sys".into());
    cli.push("--flat".into());

    windows_bindgen::bindgen(cli);

    assert_dispatcher_queue_function(&winmd);
}

/// A self-contained slice: every function resolves to `gdi32` or `user32` (both
/// listed `import-libs`) and the structs/typedefs/callbacks cover the converter
/// edges most prone to regression.
const FILTER: &[&str] = &[
    // Forced over-alignment (`__declspec(align(16))`) - must keep `#[repr(align(16))]`.
    "M128A",
    // CONTEXT (#3761): x64 inherits align 16 from its M128A members; layout must be
    // 1232 bytes / align 16. Asserted in lib.rs.
    "CONTEXT",
    // `#[repr(C, packed(1))]` transitively containing the 8-aligned `STRRET`
    // union - a regression here (spurious `#[align]` on `STRRET`) makes Rust
    // reject the packed parent (`E0588`).
    "SHELLDETAILS",
    // `typedef void` - must lower to `core::ffi::c_void`, not a by-value struct.
    "MENUTEMPLATEA",
    // A callback/function-pointer typedef embedded in a struct field
    // (`WNDCLASSEXA::lpfnWndProc: WNDPROC`) alongside a wide spread of opaque
    // handles (`HINSTANCE`/`HICON`/`HCURSOR`/`HBRUSH`) whose declared types survive.
    "WNDCLASSEXA",
    // A nested struct (`MSG` embeds `POINT`) reached as an out-pointer param.
    "MSG",
    // `gdi32` functions: exercise `link!` lowering and opaque handle parameters.
    "BitBlt",
    "CreateCompatibleBitmap",
    "CreateCompatibleDC",
    "DeleteDC",
    "DeleteObject",
    "SelectObject",
    // A counted-array parameter: Polyline's input buffer has a readable extent given by cpt.
    "Polyline",
    // `user32` functions: confirm the DLL resolves after adding `user32.lib`.
    "MessageBoxA",
    "GetMessageA",
    // `DispatcherQueue.h`: the by-value options struct and enums used by windows-composition.
    // The function's WinRT interop pointer and CoreMessaging import are checked separately below.
    "DISPATCHERQUEUE_THREAD_APARTMENTTYPE",
    "DISPATCHERQUEUE_THREAD_TYPE",
    "DispatcherQueueOptions",
    // Pointer constants retain the compiler's sign-extended native address.
    "HKEY_LOCAL_MACHINE",
    "INVALID_HANDLE_VALUE",
    // A compiler-evaluated GUID definition must lower to a GUID const initializer.
    "GUID_DEVINTERFACE_CDROM",
    // A struct using the Win32 `*EX` C++ base-class idiom
    // (`tagMONITORINFOEXA : public tagMONITORINFO`): the base subobject must be
    // captured as a leading `base: MONITORINFO` field, or the struct is too small.
    "MONITORINFOEXA",
];

fn assert_dispatcher_queue_function(winmd: &str) {
    use windows_metadata::Type;
    use windows_metadata::reader::Item;

    let index = windows_metadata::reader::Index::read(winmd).unwrap();
    let Item::Fn(function) = index.expect_item("Windows.Win32", "CreateDispatcherQueueController")
    else {
        panic!("CreateDispatcherQueueController is not a function");
    };

    let import = function.impl_map().unwrap();
    assert_eq!(import.import_name(), "CreateDispatcherQueueController");
    assert_eq!(import.import_scope().name(), "CoreMessaging.dll");
    assert_eq!(function.calling_convention(), "system");

    let signature = function.signature(&[]);
    assert_eq!(
        signature.return_type,
        Type::value_named("Windows.Win32", "HRESULT")
    );
    assert_eq!(
        signature.types,
        [
            Type::value_named("Windows.Win32", "DispatcherQueueOptions"),
            Type::PtrMut(
                Box::new(Type::value_named(
                    "Windows.Win32",
                    "PDISPATCHERQUEUECONTROLLER"
                )),
                1
            ),
        ]
    );
    assert_eq!(
        index.expect("Windows.Win32", "HRESULT").underlying_type(),
        Some(Type::I32)
    );
    assert_eq!(
        index
            .expect("Windows.Win32", "PDISPATCHERQUEUECONTROLLER")
            .underlying_type(),
        Some(Type::class_named(
            "Windows.System",
            "IDispatcherQueueController"
        ))
    );
}
