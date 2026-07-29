//! Shared packaging logic for the published `windows` / `windows-sys` crates.
//!
//! The header-based namespace [`remap`] is exposed as a library so `tool_features` can derive the
//! exact same header-stem features the published crates ship, reusing this routing rather than
//! duplicating the fold rules (which would silently drift out of sync).

pub mod remap;

use remap::Corpus;

/// The already-namespaced WinRT metadata, projected alongside the remapped Win32/WDK partition.
pub const WINRT_WINMD: &str = "crates/libs/bindgen/default/Windows.winmd";

/// The flat canonical Win32/WDK corpora — the committed per-header RDL (the routing signal) and the
/// single flat winmd compiled from them — that [`remap`] partitions into header namespaces. Both
/// share the single `Windows.Win32` root (the WDK headers are additive kernel-mode surface in the
/// same global non-WinRT namespace) and point at the same merged winmd (`tool_wdk` merges the um
/// and km surfaces, unioning same-named enums). They are remapped together, so a header stem in
/// either RDL corpus routes its types to its own namespace and WDK references to Win32 types
/// resolve to the remapped Win32 namespaces. The shared winmd is loaded once (see [`remap::run`]).
pub fn corpora() -> [Corpus; 2] {
    [
        Corpus {
            rdl_dir: "metadata/win32",
            winmd: "crates/libs/bindgen/default/Windows.Win32.winmd",
            root: "Windows.Win32",
        },
        Corpus {
            rdl_dir: "metadata/wdk",
            winmd: "crates/libs/bindgen/default/Windows.Win32.winmd",
            root: "Windows.Win32",
        },
    ]
}
