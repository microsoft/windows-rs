use std::path::PathBuf;
use windows_clang::*;

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

/// Where intermediate binary winmd artifacts (per-arch throwaways and the x64 scrape that
/// feeds arch-merge) are written. Under `target` and not tracked — regenerated on demand.
const OUT_DIR: &str = "target/wdk";

/// SAL capture shim, shared with `tool_win32` and force-included (`-include`) ahead of the
/// translation unit so the WDK routines' parameter direction/size annotations are captured
/// with the same fidelity as the Win32 surface. See the file header for the mechanism.
const SAL_SHIM: &str = "crates/tools/win32/src/sal.h";

/// Force-included (`-include`) prelude that supplies the handful of Win32 `um` typedefs
/// `offreg.h` needs (it is a user-mode API shipped in the WDK `km` folder). See the file
/// header for why this is needed and why none of it reaches the corpus.
const OFFREG_PRELUDE: &str = "crates/tools/wdk/src/offreg_prelude.h";

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

/// Arch-neutral clang arguments shared by every architecture pass. Parse as C++ (for
/// `extern "C"`, `__declspec`, SAL). The per-arch target triple and the arch-selection
/// macros are supplied separately (see [`arch_defines`]); the kernel-mode headers never pull
/// `windows.h`, so `ntdef.h` errors "No Target Architecture" without those macros the SDK
/// build normally sets. `NTDDI_VERSION` gates the API level.
const CLANG_ARGS: &[&str] = &["-x", "c++", "-DNTDDI_VERSION=0x0A000010"];

// The orchestration manifest, expressed as plain `const` slices (was `wdk.toml`). The WDK scrape
// is the *additive* companion to `tool_win32`: it parses the kernel-mode WDK headers and emits
// only the surface the WDK adds on top of Win32, into the same flat `Windows.Win32` namespace.
// Every entity Win32 already defines is skipped (see [`REFERENCE_WINMDS`]) and resolved by bare
// name once both winmds are loaded together. Like Win32 there is deliberately NO type-level
// curation — the only inputs are mechanical.

/// Root namespace; the WDK surface is emitted into the same flat `Windows.Win32` namespace as
/// Win32 so a WDK entity referencing a Win32 type just names it.
const ROOT: &str = "Windows.Win32";

/// In-scope header directory segments (`["km"]`): a declaration defined under the WDK kernel-mode
/// include folder is emitted unconditionally; the SDK `um`/`shared`/`ucrt` closure the translation
/// unit pulls in to compile is emitted only when a `km` declaration references it (and then dropped
/// anyway if Win32 already defines it), so the CRT/toolset noise never reaches the corpus.
const SCOPE: &[&str] = &["km"];

/// Architectures to scrape and arch-merge, mirroring `tool_win32`. x64 is always canonical; the
/// extra arches are folded in via `SupportedArchitecture` so a kernel type present on only a subset
/// of arches (`KUMS_CONTEXT_HEADER`, whose `PXMM_SAVE_AREA32` field is x64/arm64ec-only) is tagged
/// instead of emitted arch-neutral and breaking the pure-arm64 build.
const ARCHS: &[&str] = &["x64", "arm64", "x86"];

/// The WDK source headers, in include order. `ntifs.h` comes *before* `wdm.h`: it defines
/// `_NTIFS_INCLUDED_` and the `PEPROCESS`/`PETHREAD` opaque typedefs first, so including it second
/// would collide with `wdm.h`'s own forward declarations of the same names. `offreg.h` (the
/// offline-registry API) has no includes of its own and relies on the `DWORD`/`PCWSTR`/`HANDLE`
/// types the earlier headers bring in, so it is included last.
const SOURCE_HEADERS: &[&str] = &["ntifs.h", "wdm.h", "offreg.h"];

/// The in-house Win32 winmd(s), used as the scrape-time *exclusion* reference (already-defined
/// Win32 types are skipped rather than re-emitted, so this winmd holds only the WDK-net-new surface)
/// and the compile-time *resolution* reference (WDK types resolve their Win32 dependencies —
/// `NTSTATUS`, `IO_STATUS_BLOCK`, `GENERIC_READ`, … — by bare name once both winmds are loaded
/// together).
const REFERENCE_WINMDS: &[&str] = &["crates/libs/bindgen/default/Windows.Win32.winmd"];

/// Import libraries (bare names, resolved against the SDK and WDK x64 lib trees) read to recover
/// the faithful function → DLL mapping the headers do not carry: `ntdll.lib` (`NtReadFile`,
/// `RtlGetVersion`, …) from the SDK and `offreg.lib` (`ORCreateHive`, …) from the WDK. Combined with
/// lib-less dropping, a routine that resolves to no import library (kernel-only `ntoskrnl` exports)
/// is dropped.
const IMPORT_LIBS: &[&str] = &["ntdll.lib", "offreg.lib"];

fn main() {
    let time = std::time::Instant::now();

    for name in ARCHS {
        assert!(
            Arch::known(name).is_some(),
            "unknown architecture `{name}` in `ARCHS` (known: x64, arm64, x86)"
        );
    }

    // Provision + validate the pinned toolchain before scraping: fetch the pinned libclang
    // wheel on demand (shared cache with `tool_win32`) and assert the loaded version, so a
    // fresh checkout regenerates without a manual `pip install` / `LIBCLANG_PATH`. The pinned
    // SDK/WDK NuGet packages are likewise fetched on first use by `nuget_package`.
    ensure_libclang();
    assert_libclang_version();
    let include_args = include_args();
    let lib_dirs = lib_dirs();

    // The faithful symbol → DLL mapping the headers don't carry, resolved to absolute paths
    // from the pinned import libraries: `ntdll.lib` (`NtReadFile`, `RtlGetVersion`, …) and
    // `offreg.lib` (`ORCreateHive`, …). Combined with `drop_lib_less`, a routine that resolves
    // to no import library (kernel-only `ntoskrnl` exports) is dropped. The mapping is
    // arch-invariant, so the x64 libs serve every arch pass.
    let import_libs: Vec<String> = IMPORT_LIBS
        .iter()
        .map(|lib| resolve(lib, &lib_dirs))
        .collect();

    // The single kernel-mode translation unit: the source headers in include order, with no
    // `windows.h` prelude (the kernel headers do not pull it).
    let source: String = SOURCE_HEADERS
        .iter()
        .map(|h| format!("#include <{h}>\n"))
        .collect();

    // x64 is always canonical; any extra arch `ARCHS` lists is folded in via arch-merge. Each
    // arch carries the preprocessor macros the kernel headers require in place of the
    // `windows.h` closure that would otherwise define them.
    let extra: Vec<String> = ARCHS.iter().map(|name| name.to_string()).collect();
    let archs = Arch::canonical_plus(&extra, arch);

    // Configure the arch-invariant parse: C++ mode plus the API-level define, the shared SAL capture
    // shim and the `offreg.h` prelude force-included ahead of the TU, the WDK/SDK include dirs, and
    // the `km` reachability scope. The per-arch target/defines are set by `scrape`.
    let mut clang = clang();
    clang
        .args(CLANG_ARGS)
        .args(["-include", SAL_SHIM])
        .args(["-include", OFFREG_PRELUDE])
        .args(include_args)
        .drop_lib_less(true)
        .scope(SCOPE.iter().copied())
        .scope_headers(SOURCE_HEADERS.iter().copied());
    clang.input_str(&source);
    for lib in &import_libs {
        clang
            .import_library(lib)
            .unwrap_or_else(|e| panic!("failed to read import library `{lib}`: {e}"));
    }

    let summary = clang.scrape(&ScrapePlan {
        root: ROOT.to_string(),
        rdl_dir: RDL_DIR.to_string(),
        out_dir: OUT_DIR.to_string(),
        winmd: WINMD.to_string(),
        archs,
        reference_winmds: REFERENCE_WINMDS.iter().map(|s| s.to_string()).collect(),
        seed: None,
        parallel: true,
    });

    print!("{summary}");
    println!(
        "generated `{RDL_DIR}` ({} partition(s)) and `{WINMD}` in {:.2}s",
        summary.partitions,
        time.elapsed().as_secs_f32()
    );
}

/// The known arch plus the preprocessor defines the kernel-mode headers need for this target.
fn arch(name: &str) -> Arch {
    let mut arch = Arch::known(name).unwrap();
    arch.defines = arch_defines(name);
    arch
}

/// The arch-selection preprocessor macros the kernel headers require in place of the
/// `windows.h` closure that would otherwise define them.
fn arch_defines(name: &str) -> Vec<String> {
    let defines: &[&str] = match name {
        "x64" => &["-D_AMD64_", "-DAMD64", "-D_WIN64"],
        "arm64" => &["-D_ARM64_", "-DARM64", "-D_WIN64"],
        "x86" => &["-D_X86_", "-Di386=1"],
        _ => &[],
    };
    defines.iter().map(|s| s.to_string()).collect()
}

/// The `-isystem` include arguments: the WDK kernel-mode headers first (`km`, then the WDK's
/// own `shared`), then the shared Windows SDK headers (`shared`, `um`, `ucrt`) that complete
/// the translation unit. Order is fixed so the parse is deterministic.
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

/// The x64 import-library search directories: the SDK's `um` tree (`ntdll.lib`) and the WDK's
/// kernel-mode tree (`offreg.lib`). The symbol → DLL mapping is arch-invariant, so the x64 libs
/// serve the canonical corpus and every additional arch pass.
fn lib_dirs() -> Vec<String> {
    let sdk = nuget_package("microsoft.windows.sdk.cpp.x64", SDK_VERSION)
        .join("c")
        .join("um")
        .join("x64");
    let wdk = nuget_package("microsoft.windows.wdk.x64", WDK_VERSION)
        .join("c")
        .join("Lib")
        .join(INCLUDE_DIR)
        .join("km")
        .join("x64");
    [sdk, wdk]
        .iter()
        .map(|dir: &PathBuf| dir.to_string_lossy().replace('\\', "/"))
        .collect()
}

fn resolve(name: &str, dirs: &[String]) -> String {
    find_in_dirs(name, dirs).unwrap_or_else(|| {
        panic!("import library `{name}` not found in any pinned SDK/WDK lib directory")
    })
}
