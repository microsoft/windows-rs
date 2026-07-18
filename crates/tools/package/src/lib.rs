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
/// flat winmd compiled from it — that [`remap`] partitions into header namespaces. Both share the
/// single `Windows.Win32` root (the WDK headers are additive kernel-mode surface in the same global
/// non-WinRT namespace) and are remapped together so WDK's references to Win32 types resolve to the
/// remapped Win32 namespaces.
pub fn corpora() -> [Corpus; 2] {
    [
        Corpus {
            rdl_dir: "metadata/win32",
            winmd: "crates/libs/bindgen/default/Windows.Win32.winmd",
            root: "Windows.Win32",
        },
        Corpus {
            rdl_dir: "metadata/wdk",
            winmd: "crates/libs/bindgen/default/Windows.Wdk.winmd",
            root: "Windows.Win32",
        },
    ]
}
