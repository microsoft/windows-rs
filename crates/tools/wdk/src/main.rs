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

/// Where intermediate binary winmd artifacts (per-arch throwaways and the x64 scrape that
/// feeds arch-merge) are written. Under `target` and not tracked — regenerated on demand.
const OUT_DIR: &str = "target/wdk";

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

/// Arch-neutral clang arguments shared by every architecture pass. Parse as C++ (for
/// `extern "C"`, `__declspec`, SAL). The per-arch target triple and the arch-selection
/// macros are supplied separately (see [`Arch`]); the kernel-mode headers never pull
/// `windows.h`, so `ntdef.h` errors "No Target Architecture" without those macros the SDK
/// build normally sets. `NTDDI_VERSION` gates the API level.
const CLANG_ARGS: &[&str] = &["-x", "c++", "-DNTDDI_VERSION=0x0A000010"];

/// A target architecture: clang triple, the `SupportedArchitecture` bitmask (1=X86, 2=X64,
/// 4=Arm64), and the arch-selection preprocessor macros the kernel headers require in place
/// of the `windows.h` closure that would otherwise define them.
struct Arch {
    name: &'static str,
    triple: &'static str,
    bits: i32,
    defines: &'static [&'static str],
}
const X64: &Arch = &ARCHES[0];

/// Architectures scraped and arch-merged, mirroring `tool_win32`. The x64 pass writes the
/// committed RDL snapshot; each additional arch is scraped to a throwaway winmd and folded
/// in via `SupportedArchitecture`, so a kernel type that exists on only a subset of arches
/// (`KUMS_CONTEXT_HEADER`, whose `PXMM_SAVE_AREA32` field is x64/arm64ec-only) is tagged
/// instead of emitted arch-neutral and breaking the pure-arm64 build.
const ARCHES: [Arch; 3] = [
    Arch {
        name: "x64",
        triple: "x86_64-pc-windows-msvc",
        bits: 2,
        defines: &["-D_AMD64_", "-DAMD64", "-D_WIN64"],
    },
    Arch {
        name: "arm64",
        triple: "aarch64-pc-windows-msvc",
        bits: 4,
        defines: &["-D_ARM64_", "-DARM64", "-D_WIN64"],
    },
    Arch {
        name: "x86",
        triple: "i686-pc-windows-msvc",
        bits: 1,
        defines: &["-D_X86_", "-Di386=1"],
    },
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

    // The faithful symbol → DLL mapping the headers don't carry, recovered from the pinned
    // import libraries: `ntdll.lib` (`NtReadFile`, `RtlGetVersion`, …) and `offreg.lib`
    // (`ORCreateHive`, …). Combined with `drop_lib_less`, a routine that resolves to no
    // import library (kernel-only `ntoskrnl` exports) is dropped. The mapping is
    // arch-invariant, so the x64 libs serve every arch pass.
    let import_libs = [
        sdk_lib_dir()
            .join("ntdll.lib")
            .to_string_lossy()
            .replace('\\', "/"),
        wdk_lib_dir()
            .join("offreg.lib")
            .to_string_lossy()
            .replace('\\', "/"),
    ];

    std::fs::create_dir_all(OUT_DIR)
        .unwrap_or_else(|e| panic!("failed to create `{OUT_DIR}`: {e}"));

    let source: String = SOURCE_HEADERS
        .iter()
        .map(|h| format!("#include <{h}>\n"))
        .collect();

    // Assemble every architecture pass as an independent job. The x64 pass writes the
    // committed RDL snapshot; each extra arch writes a throwaway RDL dir + winmd under
    // `OUT_DIR`. The x64 winmd is an arch-merge input; the committed final winmd is
    // re-derived from the merged RDL corpus below.
    let x64_winmd = format!("{OUT_DIR}/Windows.Wdk.winmd");
    let extra: Vec<&Arch> = ARCHES.iter().filter(|a| a.bits != X64.bits).collect();
    // The non-x64 passes need clang's version-matched builtin resource headers so MSVC's
    // `intrin.h` reconciles with the target's compiler builtins. Resolved (and fetched on
    // first use) once, before the parallel section, so workers never race the download.
    let resource_dir = (!extra.is_empty()).then(clang_resource_dir);

    struct Job {
        arch: &'static Arch,
        rdl_dir: String,
        winmd: String,
    }
    let mut jobs = vec![Job {
        arch: X64,
        rdl_dir: RDL_DIR.to_string(),
        winmd: x64_winmd.clone(),
    }];
    for arch in &extra {
        jobs.push(Job {
            arch,
            rdl_dir: format!("{OUT_DIR}/{}", arch.name),
            winmd: format!("{OUT_DIR}/Windows.Wdk.{}.winmd", arch.name),
        });
    }

    for job in &jobs {
        let resource = (job.arch.bits != X64.bits).then(|| resource_dir.as_deref().unwrap());
        scrape_to_winmd(
            job.arch,
            &source,
            &include_args,
            &import_libs,
            &job.rdl_dir,
            &job.winmd,
            resource,
        );
    }

    // Arch-merge the per-arch winmds so a type present on (or differing across) only a subset
    // of arches gains `SupportedArchitecture`, route each item back to its defining-header
    // `<stem>.rdl`, and re-derive the unified winmd from the merged corpus. The WDK carries
    // no metadata seed of its own — its `SupportedArchitecture` (and every other metadata
    // attribute) resolves against the Win32 winmd reference at compile time below.
    if extra.is_empty() {
        std::fs::copy(&x64_winmd, WINMD)
            .unwrap_or_else(|e| panic!("failed to publish winmd to `{WINMD}`: {e}"));
    } else {
        let arch_inputs: Vec<ArchInput> = jobs
            .iter()
            .map(|j| ArchInput {
                rdl_dir: j.rdl_dir.clone(),
                winmd: j.winmd.clone(),
                bits: j.arch.bits,
            })
            .collect();
        merge_arch_rdl(&arch_inputs, None, RDL_DIR)
            .unwrap_or_else(|e| panic!("arch-merge failed: {e}"));
    }

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

/// Scrapes the WDK translation unit for one architecture into `rdl_dir` (clearing any stale
/// `.rdl` first) and compiles those partitions into `winmd`, with the Win32 winmd supplied
/// as both the scrape-time exclusion reference (already-defined Win32 types are skipped) and
/// the compile-time reference (WDK types resolve their Win32 dependencies by bare name).
fn scrape_to_winmd(
    arch: &Arch,
    source: &str,
    include_args: &[String],
    import_libs: &[String],
    rdl_dir: &str,
    winmd: &str,
    resource_dir: Option<&str>,
) {
    std::fs::create_dir_all(rdl_dir)
        .unwrap_or_else(|e| panic!("failed to create `{rdl_dir}`: {e}"));
    for entry in
        std::fs::read_dir(rdl_dir).unwrap_or_else(|e| panic!("failed to read `{rdl_dir}`: {e}"))
    {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|x| x == "rdl") {
            std::fs::remove_file(&path)
                .unwrap_or_else(|e| panic!("failed to remove `{}`: {e}", path.display()));
        }
    }

    let mut clang = clang();
    clang
        .target(arch.triple)
        .args(CLANG_ARGS)
        .args(arch.defines)
        .args(["-include", SAL_SHIM])
        .args(["-include", OFFREG_PRELUDE])
        .args(include_args.iter().map(String::as_str))
        .input_str(source)
        .input(WIN32_WINMD)
        .drop_lib_less(true)
        .scope(SCOPE)
        .scope_headers(SOURCE_HEADERS.iter().copied());
    if arch.bits != X64.bits
        && let Some(dir) = resource_dir
    {
        clang.args(["-resource-dir", dir]);
    }

    for lib in import_libs {
        clang
            .import_library(lib)
            .unwrap_or_else(|e| panic!("failed to read import library `{lib}`: {e}"));
    }

    clang
        .write_by_header(ROOT, &[], rdl_dir)
        .unwrap_or_else(|e| panic!("failed to generate partitions in `{rdl_dir}`: {e}"));

    let mut rdl_paths: Vec<String> = std::fs::read_dir(rdl_dir)
        .unwrap_or_else(|e| panic!("failed to read `{rdl_dir}`: {e}"))
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|x| x == "rdl"))
        .map(|path| path.to_string_lossy().replace('\\', "/"))
        .collect();
    rdl_paths.sort();

    reader()
        .inputs(&rdl_paths)
        .input(WIN32_WINMD)
        .output(winmd)
        .write()
        .unwrap_or_else(|e| panic!("failed to compile `{rdl_dir}` into `{winmd}`: {e}"));
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
