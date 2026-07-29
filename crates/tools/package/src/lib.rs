//! Shared packaging logic for the published `windows` / `windows-sys` crates.
//!
//! The header-based namespace [`remap`] is exposed as a library so `tool_features` can derive the
//! exact same header-stem features the published crates ship, reusing this routing rather than
//! duplicating the fold rules (which would silently drift out of sync).

pub mod remap;

use remap::RemapPlan;

/// The already-namespaced WinRT metadata, projected alongside the remapped Win32/WDK partition.
pub const WINRT_WINMD: &str = "crates/libs/bindgen/default/Windows.winmd";

/// The header remap plan for the published `windows` / `windows-sys` crates: the committed
/// per-header RDL directories (the routing signal) and the single flat winmd compiled from them,
/// which [`remap`] partitions into header namespaces. The Win32 and WDK RDL directories share the
/// one `Windows.Win32` root (the WDK headers are additive kernel-mode surface in the same global
/// non-WinRT namespace) and the one merged winmd (`tool_wdk` merges the um and km surfaces,
/// unioning same-named enums). They are read together, so a header stem in either directory routes
/// its types to its own namespace and WDK references to Win32 types resolve to the remapped Win32
/// namespaces.
pub fn remap_plan() -> RemapPlan {
    RemapPlan {
        rdl_dirs: &["metadata/win32", "metadata/wdk"],
        winmd: "crates/libs/bindgen/default/Windows.Win32.winmd",
        root: "Windows.Win32",
    }
}
