#![doc = include_str!("../readme.md")]

/// RDL source-emission primitives shared with the `windows-clang` scraper.
pub mod emit;
mod error;
/// Helpers for formatting generated RDL source.
pub mod formatter;
/// Reader for COFF import libraries (the SDK `.lib` archives), used to recover
/// the faithful function → DLL mapping that headers do not carry.
pub mod implib;
mod reader;
mod writer;

use emit::*;
use std::collections::{BTreeMap, HashMap, HashSet};
use syn::spanned::Spanned;
use windows_metadata as metadata;

pub use error::Error;
use proc_macro2::{Literal, Span, TokenStream};
use quote::quote;
pub use reader::Reader;
pub use writer::Writer;

/// The metadata namespace that owns the Win32 attribute vocabulary.
pub(crate) const METADATA_NAMESPACE: &str = "Windows.Win32.Metadata";

/// A naturalized pseudo-attribute: the short SAL/IDL-style RDL spelling paired with the
/// `Windows.Win32.Metadata` attribute it maps to.
///
/// The RDL carries the concise source-style spelling (e.g. `#[len_param(2)]`, `#[reserved]`);
/// the metadata-vocabulary mapping (e.g. `NativeArrayInfoAttribute`, `ReservedAttribute`) lives
/// here in one place. The reader emits the metadata attribute from the short spelling and the
/// writer re-emits the short spelling from the metadata attribute, so the winmd is unaffected.
pub(crate) struct PseudoAttr {
    /// Short RDL spelling, e.g. `"len_param"`.
    pub short: &'static str,
    /// Metadata attribute type name, e.g. `"NativeArrayInfoAttribute"`.
    pub metadata: &'static str,
    /// When the short spelling takes a single positional argument that binds to a *named*
    /// metadata property (the attribute's constructor is parameterless), the property name —
    /// e.g. `len_param(2)` binds `2` to `NativeArrayInfoAttribute::CountParamIndex`. `None`
    /// means the pseudo is a bare marker or its argument maps to a positional constructor
    /// argument that passes through unchanged (e.g. `encoding("ansi")`).
    pub prop: Option<&'static str>,
}

/// The pseudo-attribute table. For parameters this is also the winmd custom-attribute emission
/// order (see `encode_params`), kept stable so the metadata layout is byte-identical.
pub(crate) const PSEUDO_ATTRS: &[PseudoAttr] = &[
    PseudoAttr {
        short: "retval",
        metadata: "RetValAttribute",
        prop: None,
    },
    PseudoAttr {
        short: "iid_is",
        metadata: "ComOutPtrAttribute",
        prop: None,
    },
    PseudoAttr {
        short: "len_param",
        metadata: "NativeArrayInfoAttribute",
        prop: Some("CountParamIndex"),
    },
    PseudoAttr {
        short: "len_const",
        metadata: "NativeArrayInfoAttribute",
        prop: Some("CountConst"),
    },
    PseudoAttr {
        short: "size_param",
        metadata: "MemorySizeAttribute",
        prop: Some("BytesParamIndex"),
    },
    PseudoAttr {
        short: "reserved",
        metadata: "ReservedAttribute",
        prop: None,
    },
    PseudoAttr {
        short: "noreturn",
        metadata: "DoesNotReturnAttribute",
        prop: None,
    },
    PseudoAttr {
        short: "scoped",
        metadata: "ScopedEnumAttribute",
        prop: None,
    },
    PseudoAttr {
        short: "encoding",
        metadata: "NativeEncodingAttribute",
        prop: None,
    },
];

pub(crate) fn pseudo_by_short(short: &str) -> Option<&'static PseudoAttr> {
    PSEUDO_ATTRS.iter().find(|p| p.short == short)
}

/// Finds the pseudo-attribute that renders the given metadata attribute. When several pseudos
/// share a metadata type — differing only by which named property they carry (e.g.
/// `NativeArrayInfoAttribute` → `len_param`/`len_const`) — `arg_names` disambiguates by the
/// property present on the attribute. Property-less pseudos match by metadata name alone.
///
/// A property-bound pseudo only matches when its property is the attribute's *sole* argument:
/// the short form emits its value positionally and the reader binds a single positional back to
/// that one property, so an instance carrying additional values must fall back to the
/// fully-qualified named spelling (which round-trips) rather than the short form (which would
/// not parse back).
pub(crate) fn pseudo_for_metadata(name: &str, arg_names: &[String]) -> Option<&'static PseudoAttr> {
    let mut fallback = None;
    for pseudo in PSEUDO_ATTRS.iter().filter(|p| p.metadata == name) {
        match pseudo.prop {
            Some(prop) if arg_names.len() == 1 && arg_names[0] == prop => return Some(pseudo),
            None => fallback = Some(pseudo),
            _ => {}
        }
    }
    fallback
}

/// Creates a [`Reader`] that compiles RDL files into `.winmd` metadata.
pub fn reader() -> Reader {
    Reader::new()
}

/// Parses a single `.rdl` file and returns the names of every type, function, and constant
/// it defines under `namespace`. This is a pure syntactic walk — cross-file references are
/// not resolved — so it succeeds even on a partition that references types defined elsewhere.
///
/// This is the routing signal for the downstream namespace map: each canonical `.rdl` file
/// corresponds to one defining header, so its item names identify which types/functions/
/// constants a header-based namespace should own.
pub fn item_names(path: &str, namespace: &str) -> Result<Vec<String>, Error> {
    reader::item_names(path, namespace)
}

/// Creates a [`Writer`] that converts `.winmd` metadata into RDL.
pub fn writer() -> Writer {
    Writer::new()
}

/// One architecture's scrape outputs: the per-header RDL directory it wrote, the winmd
/// compiled from that directory, and the `SupportedArchitecture` bitmask for the arch
/// (1=X86, 2=X64, 4=Arm64).
pub struct ArchInput {
    /// Directory of per-header `<stem>.rdl` files for this architecture.
    pub rdl_dir: String,
    /// The winmd compiled from `rdl_dir` (used as the merge input).
    pub winmd: String,
    /// Architecture bitmask: 1=X86, 2=X64, 4=Arm64.
    pub bits: i32,
}

/// Arch-merges the per-architecture scrape outputs into a single merged RDL directory that
/// faithfully describes every architecture, then leaves the unified winmd derivable by
/// re-reading that directory.
///
/// The winmd carries no defining-header information, so the per-header file layout is
/// recovered from the per-arch RDL directories: each item is routed back to the
/// `<stem>.rdl` it came from, with `SupportedArchitecture` attributes on items that are
/// absent on (or differ across) some architectures. `seed` is the hand-authored metadata
/// vocabulary `.rdl` (attribute definitions); it is preserved verbatim in `output_dir` and
/// is needed to compile each partition. The heavy lifting is done by [`Reader`], [`Writer`]
/// and [`metadata::merge()`]; the per-partition name discovery runs in parallel.
pub fn merge_arch_rdl(inputs: &[ArchInput], seed: &str, output_dir: &str) -> Result<(), Error> {
    if inputs.is_empty() {
        return Err(writer_err!(
            "merge_arch_rdl requires at least one arch input"
        ));
    }

    // Preserve the hand-authored seed: the Writer clears `*.rdl` from the output directory
    // (which is typically the committed corpus that already holds the seed) before writing.
    let seed_name = std::path::Path::new(seed)
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| writer_err!("invalid seed path `{seed}`"))?
        .to_string();
    let seed_text =
        std::fs::read(seed).map_err(|e| writer_err!("failed to read seed `{seed}`: {e}"))?;

    // 1. Arch-merge the per-arch winmds into one merged winmd with SupportedArchitecture.
    //    The scratch dir is uniquely named (pid + nanos) so concurrent merges never share it,
    //    and a `Drop` guard removes it on every return path — including the `?` early-returns
    //    below, which the previous end-of-function cleanup missed.
    let temp = std::env::temp_dir().join(format!(
        "win32-arch-merge-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_nanos())
    ));
    std::fs::create_dir_all(&temp)
        .map_err(|e| writer_err!("failed to create temp dir `{}`: {e}", temp.display()))?;
    let _scratch = ScratchDir(temp.clone());
    let merged = temp.join("Windows.Win32.merged.winmd");
    let merged = merged.to_string_lossy().to_string();
    let mut merger = metadata::merge();
    for input in inputs {
        merger.arch_input(&input.winmd, input.bits);
    }
    merger
        .output(&merged)
        .merge()
        .map_err(|e| writer_err!("arch-merge failed: {e}"))?;

    // 2. Build the item-name -> defining-header-stem map by parsing each per-arch
    //    `<stem>.rdl` and reading back the names it defines. This is a pure syntactic walk
    //    (cross-header references are not resolved), so a partition that references types
    //    defined in another partition still parses cleanly. Names are unioned across
    //    architectures (arch-divergent headers such as winnt.rdl define different names per
    //    arch but share the stem).
    let mut map = HashMap::<String, String>::new();
    for input in inputs {
        for entry in std::fs::read_dir(&input.rdl_dir)
            .map_err(|e| writer_err!("failed to read `{}`: {e}", input.rdl_dir))?
            .flatten()
        {
            let path = entry.path();
            if !path.extension().is_some_and(|x| x == "rdl")
                || path.file_name().and_then(|n| n.to_str()) == Some(seed_name.as_str())
            {
                continue;
            }
            let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            let rdl_path = path.to_string_lossy().to_string();
            for name in reader::item_names(&rdl_path, "Windows.Win32")? {
                map.entry(name).or_insert_with(|| stem.to_string());
            }
        }
    }

    // 3. Decompile the merged winmd into per-stem RDL files using the name->stem map.
    writer()
        .input(&merged)
        .partition(map)
        .output(output_dir)
        .write()?;

    // 4. Restore the hand-authored seed verbatim.
    write_to_file(
        std::path::Path::new(output_dir)
            .join(&seed_name)
            .to_str()
            .ok_or_else(|| writer_err!("output path contains non-UTF-8 characters"))?,
        seed_text,
    )?;

    Ok(())
}

/// Removes a scratch directory when dropped, so it is cleaned up on every return path
/// (including error `?` early-returns), not just a single end-of-function call.
struct ScratchDir(std::path::PathBuf);

impl Drop for ScratchDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

pub fn expand_input_paths(
    inputs: &[String],
    ext1: &str,
    ext2: &str,
) -> Result<(Vec<String>, Vec<String>), Error> {
    let mut paths1 = vec![];
    let mut paths2 = vec![];

    for input in inputs {
        let path = std::path::Path::new(input);

        if path.is_dir() {
            let prev_total = paths1.len() + paths2.len();

            for entry_path in path
                .read_dir()
                .map_err(|_| Error::new("failed to read directory", input, 0, 0))?
                .flatten()
                .map(|entry| entry.path())
            {
                if entry_path.is_file() {
                    if entry_path
                        .extension()
                        .is_some_and(|ext| ext.eq_ignore_ascii_case(ext1))
                    {
                        paths1.push(entry_path.to_string_lossy().replace('\\', "/"));
                    } else if entry_path
                        .extension()
                        .is_some_and(|ext| ext.eq_ignore_ascii_case(ext2))
                    {
                        paths2.push(entry_path.to_string_lossy().replace('\\', "/"));
                    }
                }
            }

            if paths1.len() + paths2.len() == prev_total {
                return Err(Error::new(
                    &format!("failed to find .{ext1} or .{ext2} files in directory"),
                    input,
                    0,
                    0,
                ));
            }
        } else if path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case(ext1))
        {
            paths1.push(input.clone());
        } else if path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case(ext2))
        {
            paths2.push(input.clone());
        } else {
            return Err(Error::new(
                &format!("expected .{ext1} or .{ext2} file"),
                input,
                0,
                0,
            ));
        }
    }

    Ok((paths1, paths2))
}

pub fn write_to_file<C: AsRef<[u8]>>(path: &str, contents: C) -> Result<(), Error> {
    if let Some(parent) = std::path::Path::new(path).parent() {
        std::fs::create_dir_all(parent)
            .map_err(|_| writer_err!("failed to create directory `{path}`"))?;
    }

    std::fs::write(path, contents).map_err(|_| writer_err!("failed to write file `{path}`"))
}

macro_rules! writer_err {
    ($($arg:tt)*) => {
        Error::new(&format!($($arg)*), "", 0, 0)
    };
}

use writer_err;

#[cfg(test)]
mod tests {
    use super::*;

    /// A property-bound pseudo (`NativeArrayInfoAttribute` → `len_param` via `CountParamIndex`)
    /// is only used when its property is the attribute's sole argument. The short form emits its
    /// value positionally and the reader binds a single positional back to that one property, so
    /// a multi-valued instance must fall back to the fully-qualified spelling (returns `None`)
    /// rather than a short form that would not parse back.
    #[test]
    fn prop_bound_pseudo_requires_sole_argument() {
        let sole = ["CountParamIndex".to_string()];
        let pseudo = pseudo_for_metadata("NativeArrayInfoAttribute", &sole)
            .expect("single-property NativeArrayInfo should map to a short pseudo");
        assert_eq!(pseudo.short, "len_param");

        let extra = ["CountParamIndex".to_string(), "CountConst".to_string()];
        assert!(
            pseudo_for_metadata("NativeArrayInfoAttribute", &extra).is_none(),
            "a multi-valued property-bound attribute must fall back to the fully-qualified spelling"
        );
    }

    /// Property-less pseudos match by metadata name regardless of argument count.
    #[test]
    fn property_less_pseudo_matches_by_name() {
        let pseudo = pseudo_for_metadata("RetValAttribute", &[])
            .expect("RetValAttribute should map to a pseudo");
        assert_eq!(pseudo.short, "retval");
    }
}
