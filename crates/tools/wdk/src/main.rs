use std::path::PathBuf;
use windows_clang::*;
use windows_rdl::*;

/// The committed, canonical WDK winmd. Like the Win32 winmd this is checked in as
/// `windows-bindgen`'s default WDK metadata (alongside the `metadata/wdk` RDL corpus) so
/// downstream `tool_bindings` filters can point `--in` at a stable in-repo winmd without
/// re-running this tool. It is *additive*: it carries only the surface the WDK adds on top
/// of Win32, in the same flat namespace, so a filter loads both winmds together and every
/// symbol resolves by bare name. Re-derived from the RDL on every run; treat it as
/// generated output.
const WINMD: &str = "crates/libs/bindgen/default/Windows.Wdk.winmd";

/// Where the WDK RDL snapshot is written (committed, human-reviewable), one file per
/// defining header (`wdm.rdl`, `ntifs.rdl`, …) exactly like `metadata/win32`. Regenerated
/// by running this tool; treat it as generated output, not a place to hand-edit.
const RDL_DIR: &str = "metadata/wdk";

/// The in-house Win32 winmd, fed as the *exclusion reference*. Every entity it already
/// defines (the shared closure — `NTSTATUS`, `IO_STATUS_BLOCK`, `GENERIC_READ`, …) is
/// skipped rather than re-emitted here, so this winmd holds only the WDK-net-new surface.
/// Those skipped references resolve against the Win32 winmd by bare name once both are
/// loaded together, and at compile time below where it is passed to `reader()`.
const WIN32_WINMD: &str = "crates/libs/bindgen/default/Windows.Win32.winmd";

/// Root namespace for the scrape. The WDK surface is emitted into the *same* flat
/// `Windows.Win32` namespace as Win32 (which is really "the global, not-WinRT namespace"),
/// so a WDK entity referencing a Win32 type just names it and bindgen resolves it by bare
/// name once both winmds are loaded — no cross-namespace resolution. Duplicates against
/// Win32 are dropped via [`WIN32_WINMD`], so only the net-new surface lands here.
const ROOT: &str = "Windows.Win32";

/// SAL capture shim, shared with `tool_win32` and force-included (`-include`) ahead of the
/// translation unit so the WDK routines' parameter direction/size annotations are captured
/// with the same fidelity as the Win32 surface. See the file header for the mechanism.
const SAL_SHIM: &str = "crates/tools/win32/src/sal.h";

/// Force-included (`-include`) prelude that supplies the handful of Win32 `um` typedefs
/// `offreg.h` needs (it is a user-mode API shipped in the WDK `km` folder). See the file
/// header for why this is needed and why none of it reaches the corpus.
const OFFREG_PRELUDE: &str = "crates/tools/wdk/src/offreg_prelude.h";

/// In-scope header directory segment: the WDK kernel-mode headers live under `km`. A
/// declaration defined there is emitted unconditionally; the SDK `um`/`shared`/`ucrt`
/// closure the translation unit pulls in to compile is emitted only when a `km`
/// declaration references it (and is then dropped anyway if Win32 already defines it),
/// so the CRT/toolset noise never reaches the corpus.
const SCOPE: &[&str] = &["km"];

/// Pinned WDK version. The corpus is generated against the `Microsoft.Windows.WDK.x64`
/// NuGet package at this exact version, restored into the NuGet global cache. This is the
/// latest servicing build of the `10.0.28000` marketing line that matches [`SDK_VERSION`];
/// the WDK's servicing build lags the SDK's, so it is pinned independently.
const WDK_VERSION: &str = "10.0.28000.1839";

/// Pinned Windows SDK version, shared with `tool_win32` (its `um`/`shared`/`ucrt` headers
/// and `ntdll.lib` complete the WDK translation unit). Must match `tool_win32`'s
/// `SDK_VERSION` so the shared types the exclusion reference resolves against are the same
/// ones this scrape sees.
const SDK_VERSION: &str = "10.0.28000.2270";

/// The marketing SDK folder nested inside both package `c/Include` trees.
const INCLUDE_DIR: &str = "10.0.28000.0";

/// Clang arguments: parse as C++ (for `extern "C"`, `__declspec`, SAL) targeting x64.
/// The kernel-mode headers never pull `windows.h`, so `ntdef.h` errors "No Target
/// Architecture" without the arch selection macros the SDK build normally sets — define
/// them explicitly. `NTDDI_VERSION` gates the API level.
const CLANG_ARGS: &[&str] = &[
    "-x",
    "c++",
    "--target=x86_64-pc-windows-msvc",
    "-D_AMD64_",
    "-DAMD64",
    "-D_WIN64",
    "-DNTDDI_VERSION=0x0A000010",
];

/// The WDK source headers, in include order. `ntifs.h` comes *before* `wdm.h`: it defines
/// `_NTIFS_INCLUDED_` and the `PEPROCESS`/`PETHREAD` opaque typedefs first, so including it
/// second would collide with `wdm.h`'s own forward declarations of the same names. `offreg.h`
/// (the offline-registry API) has no includes of its own and relies on the `DWORD`/`PCWSTR`/
/// `HANDLE` types the earlier headers bring in, so it is included last.
const SOURCE_HEADERS: &[&str] = &["ntifs.h", "wdm.h", "offreg.h"];

fn main() {
    let time = std::time::Instant::now();

    // Provision + validate the pinned toolchain before scraping: fetch the pinned libclang
    // wheel on demand (shared cache with `tool_win32`) and assert the loaded version, so a
    // fresh checkout regenerates without a manual `pip install` / `LIBCLANG_PATH`. The pinned
    // SDK/WDK NuGet packages are likewise fetched on first use by `nuget_package`.
    ensure_libclang();
    assert_libclang_version();
    let include_args = include_args();
    let ntdll_lib = sdk_lib_dir()
        .join("ntdll.lib")
        .to_string_lossy()
        .replace('\\', "/");

    // Regenerate the committed RDL directory from scratch: create it, then clear any stale
    // `.rdl` so a header that stops emitting a partition does not leave an orphaned file.
    std::fs::create_dir_all(RDL_DIR)
        .unwrap_or_else(|e| panic!("failed to create `{RDL_DIR}`: {e}"));
    for entry in
        std::fs::read_dir(RDL_DIR).unwrap_or_else(|e| panic!("failed to read `{RDL_DIR}`: {e}"))
    {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|x| x == "rdl") {
            std::fs::remove_file(&path)
                .unwrap_or_else(|e| panic!("failed to remove `{}`: {e}", path.display()));
        }
    }

    let mut clang = clang();
    let source: String = SOURCE_HEADERS
        .iter()
        .map(|h| format!("#include <{h}>\n"))
        .collect();
    clang
        .args(CLANG_ARGS)
        .args(["-include", SAL_SHIM])
        .args(["-include", OFFREG_PRELUDE])
        .args(include_args.iter().map(String::as_str))
        .input_str(&source)
        .input(WIN32_WINMD)
        .drop_lib_less(true)
        .scope(SCOPE)
        .scope_headers(SOURCE_HEADERS.iter().copied());

    // `ntdll.lib` supplies the faithful symbol → DLL mapping. Combined with `drop_lib_less`,
    // it also scopes the emitted functions to the user-callable native surface: a routine
    // exported from `ntdll.dll` (`NtReadFile`, `RtlGetVersion`, …) is stamped and kept,
    // while a kernel-only routine (exported from `ntoskrnl`, absent from any user-mode
    // import library) resolves to an empty library and is dropped.
    clang
        .import_library(&ntdll_lib)
        .unwrap_or_else(|e| panic!("failed to read import library `{ntdll_lib}`: {e}"));

    // `offreg.lib` maps the offline-registry routines (`ORCreateHive`, `ORCloseHive`, …) to
    // `offreg.dll`; without it those functions resolve to an empty library and `drop_lib_less`
    // discards them.
    let offreg_lib = wdk_lib_dir()
        .join("offreg.lib")
        .to_string_lossy()
        .replace('\\', "/");
    clang
        .import_library(&offreg_lib)
        .unwrap_or_else(|e| panic!("failed to read import library `{offreg_lib}`: {e}"));

    clang
        .write_by_header(ROOT, &[], RDL_DIR)
        .unwrap_or_else(|e| panic!("failed to generate partitions in `{RDL_DIR}`: {e}"));

    let mut rdl_paths: Vec<String> = std::fs::read_dir(RDL_DIR)
        .unwrap_or_else(|e| panic!("failed to read `{RDL_DIR}`: {e}"))
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|x| x == "rdl"))
        .map(|path| path.to_string_lossy().replace('\\', "/"))
        .collect();
    rdl_paths.sort();

    reader()
        .inputs(&rdl_paths)
        .input(WIN32_WINMD)
        .output(WINMD)
        .write()
        .unwrap_or_else(|e| panic!("failed to compile `{RDL_DIR}` into `{WINMD}`: {e}"));

    println!(
        "generated `{RDL_DIR}` ({} partition(s)) and `{WINMD}` in {:?}",
        rdl_paths.len(),
        time.elapsed()
    );
}

/// The `-isystem` include arguments: the WDK kernel-mode headers first (`km`, then the
/// WDK's own `shared`), then the shared Windows SDK headers (`shared`, `um`, `ucrt`) that
/// complete the translation unit. Order is fixed so the parse is deterministic.
fn include_args() -> Vec<String> {
    let wdk = nuget_package("microsoft.windows.wdk.x64", WDK_VERSION)
        .join("c")
        .join("Include")
        .join(INCLUDE_DIR);
    let sdk = nuget_package("microsoft.windows.sdk.cpp", SDK_VERSION)
        .join("c")
        .join("Include")
        .join(INCLUDE_DIR);
    let dirs = [
        wdk.join("km"),
        wdk.join("shared"),
        sdk.join("shared"),
        sdk.join("um"),
        sdk.join("ucrt"),
    ];
    dirs.iter()
        .flat_map(|dir| {
            [
                "-isystem".to_string(),
                dir.to_string_lossy().replace('\\', "/"),
            ]
        })
        .collect()
}

/// The pinned SDK x64 import-library directory (`ntdll.lib` lives here). The
/// function → DLL mapping is arch-invariant, so the x64 lib serves the canonical corpus.
fn sdk_lib_dir() -> PathBuf {
    nuget_package("microsoft.windows.sdk.cpp.x64", SDK_VERSION)
        .join("c")
        .join("um")
        .join("x64")
}

/// The pinned WDK x64 kernel-mode import-library directory (`offreg.lib` lives here). Like
/// the SDK libs the symbol → DLL mapping is arch-invariant, so the x64 lib serves the
/// canonical corpus.
fn wdk_lib_dir() -> PathBuf {
    nuget_package("microsoft.windows.wdk.x64", WDK_VERSION)
        .join("c")
        .join("Lib")
        .join(INCLUDE_DIR)
        .join("km")
        .join("x64")
}
