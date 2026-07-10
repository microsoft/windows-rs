#![cfg(target_pointer_width = "64")]

// Flat-namespace, per-header-file partitioning end-to-end test.
//
// `a.h` and `b.h` both `#include "shared.inl"`. With `write_by_header` the canonical
// RDL is the single flat root namespace (`Test`); each entity is emitted exactly once,
// in the file named after its defining header, and cross-header references resolve by
// bare name:
//   * `shared.inl` → `shared.rdl`: owns the opaque handle `HFOO`, the pointer alias
//     `PSHARED`, and the scalar typedef `LRESULT` (each canonically deduplicated, so the
//     copy `b.h` also sees is never re-emitted).
//   * `a.h` → `a.rdl`: `AThing(PSHARED)` — references `PSHARED` by bare name.
//   * `b.h` → `b.rdl`: `BThing(HFOO, PSHARED)` and `BReturn() -> LRESULT` — all by name.
//
// This is the faithful, source-expressed metadata that replaced the editorial namespace
// machinery: one flat namespace, ownership from the clang cursor's defining header (which
// only selects the file), and duplicates weeded out by USR.

fn read(dir: &str, leaf: &str) -> String {
    std::fs::read_to_string(format!("{dir}/{leaf}.rdl")).unwrap()
}

#[test]
fn partition_by_defining_header() {
    let scratch = format!("{}/header_partition", env!("OUT_DIR"));
    std::fs::create_dir_all(&scratch).unwrap();

    let mut clang = windows_clang::clang();
    clang
        .args([
            "-x",
            "c++",
            "--target=x86_64-pc-windows-msvc",
            "-fms-extensions",
        ])
        .library("test.dll")
        .input("partition_input/a.h")
        .input("partition_input/b.h");

    clang.write_by_header("Test", &[], &scratch).unwrap();

    let shared = read(&scratch, "shared");
    let a = read(&scratch, "a");
    let b = read(&scratch, "b");

    // Every file emits the single flat root namespace.
    assert!(shared.contains("mod Test {"), "shared.rdl:\n{shared}");
    assert!(a.contains("mod Test {"), "a.rdl:\n{a}");

    // The shared header owns all shared types, emitted exactly once.
    assert!(
        shared.contains("type HFOO = *mut void"),
        "shared.rdl:\n{shared}"
    );
    assert!(shared.contains("type PSHARED"), "shared.rdl:\n{shared}");

    // The function files never re-emit the shared types (canonical dedup).
    assert!(!a.contains("type HFOO"), "a.rdl:\n{a}");
    assert!(!a.contains("type PSHARED"), "a.rdl:\n{a}");
    assert!(!b.contains("type HFOO"), "b.rdl:\n{b}");
    assert!(!b.contains("type PSHARED"), "b.rdl:\n{b}");

    // The functions are emitted in their own header files.
    assert!(a.contains("fn AThing"), "a.rdl:\n{a}");
    assert!(b.contains("fn BThing"), "b.rdl:\n{b}");

    // Cross-header references resolve by bare name in the flat namespace — never a
    // `super::<Header>::` qualified path.
    assert!(a.contains("PSHARED"), "a.rdl:\n{a}");
    assert!(b.contains("HFOO"), "b.rdl:\n{b}");
    assert!(!a.contains("super::"), "a.rdl:\n{a}");
    assert!(!b.contains("super::"), "b.rdl:\n{b}");

    // A scalar typedef stays faithful: emitted as a named type in its defining header
    // and referenced by bare name, never collapsed to the primitive nor re-emitted.
    assert!(
        shared.contains("type LRESULT = i32"),
        "shared.rdl:\n{shared}"
    );
    assert!(b.contains("-> LRESULT"), "b.rdl:\n{b}");
    assert!(!b.contains("type LRESULT"), "b.rdl:\n{b}");

    // Pointer-sized ABI typedefs collapse to `usize`/`isize` at every reference and are
    // never emitted as named `type` items — exactly like the fixed-width portability
    // aliases (`DWORD` -> u32). `ULONG_PTR`/`SIZE_T` carry no meaning beyond being
    // pointer-sized, the reference metadata has no such types, and collapsing keeps them
    // architecture-neutral (no per-arch `u32`-vs-`usize` split). `BSize(SIZE_T) -> SIZE_T`
    // therefore reads as the bare primitive.
    assert!(!shared.contains("ULONG_PTR"), "shared.rdl:\n{shared}");
    assert!(!shared.contains("SIZE_T"), "shared.rdl:\n{shared}");
    assert!(!b.contains("SIZE_T"), "b.rdl:\n{b}");
    assert!(b.contains("count: usize"), "b.rdl:\n{b}");
    assert!(b.contains("-> usize"), "b.rdl:\n{b}");
}

// Reachability-by-reference scope sweep. `scope_api/api.h` is in scope; it `#include`s
// the out-of-scope `scope_crt/crt.h` and references `APITYPE` from it. With
// `scope(["scope_api"])`, every in-scope declaration is emitted, but an out-of-scope
// declaration survives only when an in-scope declaration transitively references it:
//   * `APITYPE` — referenced by the in-scope `ApiCall`, so it is kept (the genuine
//     cross-over type, like `va_list`/`EXCEPTION_DISPOSITION` in the real corpus).
//   * `CRTNOISE` / `CrtOnly` — referenced by nothing in scope, so they are swept (the
//     C-runtime noise `windows.h` drags in but the Windows API never references).
#[test]
fn scope_sweeps_unreferenced_out_of_scope() {
    let scratch = format!("{}/header_scope", env!("OUT_DIR"));
    std::fs::create_dir_all(&scratch).unwrap();

    let mut clang = windows_clang::clang();
    clang
        .args([
            "-x",
            "c++",
            "--target=x86_64-pc-windows-msvc",
            "-fms-extensions",
        ])
        .library("test.dll")
        .scope(["scope_api"])
        .input("partition_input/scope_api/api.h");

    clang.write_by_header("Test", &[], &scratch).unwrap();

    let api = read(&scratch, "api");
    let crt = read(&scratch, "crt");

    // The in-scope API surface is emitted and references the cross-over type by name.
    assert!(api.contains("fn ApiCall"), "api.rdl:\n{api}");
    assert!(api.contains("APITYPE"), "api.rdl:\n{api}");

    // The referenced out-of-scope type survives the sweep, in its own (out-of-scope) file.
    assert!(crt.contains("type APITYPE = i32"), "crt.rdl:\n{crt}");

    // The unreferenced out-of-scope declarations are swept.
    assert!(!crt.contains("CRTNOISE"), "crt.rdl:\n{crt}");
    assert!(!crt.contains("CrtOnly"), "crt.rdl:\n{crt}");
}

// A dotted header file name (the WinRT interop headers, e.g.
// `Windows.Devices.Display.Core.Interop.h`) must collapse to a single flat partition
// leaf, not a nested namespace/module tree under the root. The leftover dots in the
// stem are stripped so `Dotted.Name.Interop.h` → `dottednameinterop.rdl` in the flat
// root namespace.
#[test]
fn dotted_header_flattens_to_single_partition() {
    let scratch = format!("{}/header_dotted", env!("OUT_DIR"));
    std::fs::create_dir_all(&scratch).unwrap();

    let mut clang = windows_clang::clang();
    clang
        .args([
            "-x",
            "c++",
            "--target=x86_64-pc-windows-msvc",
            "-fms-extensions",
        ])
        .library("test.dll")
        .input("partition_input/Dotted.Name.Interop.h");

    clang.write_by_header("Test", &[], &scratch).unwrap();

    // The partition leaf has its dots stripped to a single flat segment; no nested
    // `dotted.name.interop.rdl` or `Dotted/Name/Interop` tree is produced.
    let dotted = read(&scratch, "dottednameinterop");
    assert!(
        !std::path::Path::new(&format!("{scratch}/dotted.name.interop.rdl")).exists(),
        "a dotted-leaf rdl must not be produced"
    );

    // It emits the single flat root namespace and owns its function directly.
    assert!(dotted.contains("mod Test {"), "dotted.rdl:\n{dotted}");
    assert!(dotted.contains("fn DottedThing"), "dotted.rdl:\n{dotted}");
}

