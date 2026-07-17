//! Downstream validation for the in-house Win32 metadata.
//!
//! `tool_win32` scrapes the Windows SDK headers into the committed
//! `metadata/win32/*.rdl` corpus (flat `Windows.Win32` namespace, one file per
//! defining header). This test feeds that committed corpus back through
//! `windows-bindgen` for a bounded, self-contained slice and writes the result
//! to `expected/slice.rs`, which `build.rs` then compile-checks. It keeps header
//! surface growth honest: if a scrape change produces metadata that no longer
//! lowers to compilable bindings, this fails on the regenerated golden.
//!
//! The slice is deliberately narrow. The full closure contains ~4.5k functions
//! whose exporting `.lib` is not in the dev `import-libs` set; those get an empty
//! `link!("")` (`E0454`) by design, so a whole-namespace filter would not
//! compile. The names below are chosen to (a) resolve to a real DLL the dev
//! manifest lists (`gdi32`) and (b) exercise the converter paths most prone to
//! regression: forced over-alignment, `packed` + nested `align`, `typedef void`,
//! opaque handles, and `link!` lowering.

/// Regenerate `expected/slice.rs` from the committed `metadata/win32` corpus.
#[test]
fn slice() {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let rdl_dir = format!("{manifest}/../../../../metadata/win32");
    let seed = format!("{manifest}/../../../../metadata/metadata.rdl");
    // The corpus carries cross-winmd references (WinRT interop APIs name true `Windows.*` types),
    // so `Windows.winmd` is supplied as a resolution reference — exactly as the scrape does via its
    // `RESOLUTION_WINMDS`.
    let winrt = format!("{manifest}/../../../../crates/libs/bindgen/default/Windows.winmd");
    let scratch = format!("{}/win32", env!("OUT_DIR"));
    std::fs::create_dir_all(&scratch).unwrap();

    // Compile the whole committed corpus to one winmd (no libclang/SDK needed). The
    // hand-authored seed (which defines the `Windows.Win32.Metadata` attribute types the
    // corpus references) lives outside `rdl_dir`, so it is fed in explicitly.
    let winmd = format!("{scratch}/Windows.Win32.winmd");
    windows_rdl::reader()
        .input(&rdl_dir)
        .input(&seed)
        .input(&winrt)
        .output(&winmd)
        .write()
        .unwrap();

    // Lower the bounded slice to flat `--sys` bindings, written straight to the
    // golden so `git diff` surfaces any change.
    let out_rs = format!("{manifest}/expected/slice.rs");
    let mut cli: Vec<String> = vec!["--in".into(), winmd, "--out".into(), out_rs];
    cli.push("--filter".into());
    cli.extend(FILTER.iter().map(|name| (*name).to_string()));
    cli.push("--sys".into());
    cli.push("--flat".into());

    windows_bindgen::bindgen(cli);
}

/// A self-contained slice: every function resolves to `gdi32` or `user32` (both
/// listed `import-libs`) and the structs/typedefs/callbacks cover the converter
/// edges most prone to regression.
const FILTER: &[&str] = &[
    // Forced over-alignment (`__declspec(align(16))`) — must keep `#[repr(align(16))]`.
    "M128A",
    // CONTEXT (#3761): x64 inherits align 16 from its M128A members; layout must be
    // 1232 bytes / align 16. Asserted in lib.rs.
    "CONTEXT",
    // `#[repr(C, packed(1))]` transitively containing the 8-aligned `STRRET`
    // union — a regression here (spurious `#[align]` on `STRRET`) makes Rust
    // reject the packed parent (`E0588`).
    "SHELLDETAILS",
    // `typedef void` — must lower to `core::ffi::c_void`, not a by-value struct.
    "MENUTEMPLATEA",
    // A callback/function-pointer typedef embedded in a struct field
    // (`WNDCLASSEXA::lpfnWndProc: WNDPROC`) alongside a wide spread of opaque
    // handles (`HINSTANCE`/`HICON`/`HCURSOR`/`HBRUSH`).
    "WNDCLASSEXA",
    // A nested struct (`MSG` embeds `POINT`) reached as an out-pointer param.
    "MSG",
    // `gdi32` functions: exercise `link!` lowering and opaque handle params
    // (`HDC`/`HGDIOBJ`/`HBITMAP` → `*mut c_void`).
    "BitBlt",
    "CreateCompatibleBitmap",
    "CreateCompatibleDC",
    "DeleteDC",
    "DeleteObject",
    "SelectObject",
    // A counted-array parameter: `Polyline`'s `apt` carries a SAL-derived
    // `NativeArrayInfo(CountParamIndex = 2)` (count given by `cpt`). Exercises the
    // bindgen array-lowering path end to end and the `NativeArrayInfoAttribute`
    // metadata-seed type, both of which the in-house SAL capture now drives.
    "Polyline",
    // `user32` functions: confirm the DLL resolves after adding `user32.lib`.
    "MessageBoxA",
    "GetMessageA",
    // Nested handle-cast `#define` constants (`parse_nested_cast`): a pointer
    // typedef'd const whose value is the innermost signed scalar reinterpretation
    // — must render as `<signed int> as _` so the pointer sign-extends.
    "HKEY_LOCAL_MACHINE",
    "INVALID_HANDLE_VALUE",
    // A `DEFINE_GUID` named GUID constant (`parse_define_guid_tokens`): the value
    // lives only in the macro arguments (no `INITGUID`), captured as a `GUID`-typed
    // field carrying a `GuidAttribute`. Must lower to a `GUID` const initializer.
    "GUID_DEVINTERFACE_CDROM",
    // A struct using the Win32 `*EX` C++ base-class idiom
    // (`tagMONITORINFOEXA : public tagMONITORINFO`): the base subobject must be
    // captured as a leading `Base: MONITORINFO` field, or the struct is too small.
    "MONITORINFOEXA",
];
