#![doc = include_str!("../readme.md")]

/// RDL source-emission primitives shared with the `windows-clang` scraper.
pub mod emit;
mod error;
/// Helpers for formatting generated RDL source.
pub mod formatter;
/// Reader for COFF import libraries (the SDK `.lib` archives), used to recover
/// the function -> DLL mapping that headers do not carry.
pub mod implib;
mod reader;
mod writer;

use emit::*;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};
use syn::spanned::Spanned;
use windows_metadata as metadata;

pub use error::{Diagnostic, Error, Label, LabelStyle, Position, Severity};
use proc_macro2::{Literal, Span, TokenStream};
use quote::quote;
pub use reader::Reader;
pub use writer::Writer;

/// The metadata namespace that owns the Win32 attribute vocabulary.
pub(crate) const METADATA_NAMESPACE: &str = "Windows.Win32.Metadata";

/// Short RDL attribute spelling and the metadata attribute it maps to.
pub(crate) struct PseudoAttr {
    pub short: &'static str,
    pub metadata: &'static str,
    /// Named property receiving a short-form positional argument, if any.
    pub prop: Option<&'static str>,
}

/// Pseudo-attribute table; order is metadata-significant for parameters.
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

/// Finds the short spelling for a metadata attribute, using property names to distinguish shared
/// attribute types such as `NativeArrayInfoAttribute`.
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

/// Parses one `.rdl` file and returns the items it defines under `namespace`.
pub fn item_names(path: impl AsRef<Path>, namespace: &str) -> Result<Vec<String>, Error> {
    reader::item_names(path, namespace)
}

/// Creates a [`Writer`] that converts `.winmd` metadata into RDL.
pub fn writer() -> Writer {
    Writer::new()
}

/// One architecture's RDL directory, compiled winmd, and architecture bitmask.
pub struct ArchInput {
    pub rdl_dir: PathBuf,
    pub winmd: PathBuf,
    pub bits: i32,
}

/// Arch-merges per-architecture scrapes and restores the per-header RDL partition.
pub fn merge_arch_rdl(
    inputs: &[ArchInput],
    seed: Option<&Path>,
    output_dir: impl AsRef<Path>,
) -> Result<(), Error> {
    let output_dir = output_dir.as_ref();

    if inputs.is_empty() {
        return Err(writer_err!(
            "merge_arch_rdl requires at least one arch input"
        ));
    }

    // `Writer` clears `*.rdl`; capture the seed first so it can be restored verbatim.
    let seed = seed
        .map(|seed| {
            let name = seed
                .file_name()
                .ok_or_else(|| writer_err!("invalid seed path `{}`", seed.display()))?
                .to_os_string();
            let text = std::fs::read(seed)
                .map_err(|e| writer_err!("failed to read seed `{}`: {e}", seed.display()))?;
            Ok::<_, Error>((name, seed.to_path_buf(), text))
        })
        .transpose()?;

    // Unique scratch dirs avoid collisions between concurrent arch merges.
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
    let mut merger = metadata::merge();
    for input in inputs {
        merger.arch_input(&input.winmd, input.bits);
    }
    merger
        .output(&merged)
        .merge()
        .map_err(|e| writer_err!("arch-merge failed: {e}"))?;

    // Recover item-name -> header-stem routing from the per-arch RDL partitions.
    let mut map = HashMap::<String, String>::new();
    for input in inputs {
        for entry in std::fs::read_dir(&input.rdl_dir)
            .map_err(|e| writer_err!("failed to read `{}`: {e}", input.rdl_dir.display()))?
            .flatten()
        {
            let path = entry.path();
            if path.extension().is_none_or(|x| x != "rdl")
                || path.file_name() == seed.as_ref().map(|(name, _, _)| name.as_os_str())
            {
                continue;
            }
            let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            for name in reader::item_names(&path, "Windows.Win32")? {
                map.entry(name).or_insert_with(|| stem.to_string());
            }
        }
    }

    writer()
        .input(&merged)
        .partition(map)
        .output(output_dir)
        .write()?;

    // Restore the hand-authored seed if this metadata set has one.
    if let Some((_, seed_path, seed_text)) = seed {
        write_to_file(seed_path, seed_text)?;
    }

    Ok(())
}

/// Removes a scratch directory on every return path.
struct ScratchDir(PathBuf);

impl Drop for ScratchDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

pub fn expand_input_paths<P: AsRef<Path>>(
    inputs: &[P],
    ext1: &str,
    ext2: &str,
) -> Result<(Vec<PathBuf>, Vec<PathBuf>), Error> {
    let mut paths1 = vec![];
    let mut paths2 = vec![];

    for input in inputs {
        let path = input.as_ref();
        let display = path.to_string_lossy();

        if path.is_dir() {
            let prev_total = paths1.len() + paths2.len();

            for entry_path in path
                .read_dir()
                .map_err(|_| Error::new("failed to read directory", &display, 0, 0))?
                .flatten()
                .map(|entry| entry.path())
            {
                if entry_path.is_file() {
                    if entry_path
                        .extension()
                        .is_some_and(|ext| ext.eq_ignore_ascii_case(ext1))
                    {
                        paths1.push(entry_path);
                    } else if entry_path
                        .extension()
                        .is_some_and(|ext| ext.eq_ignore_ascii_case(ext2))
                    {
                        paths2.push(entry_path);
                    }
                }
            }

            if paths1.len() + paths2.len() == prev_total {
                let message = if ext1 == ext2 {
                    format!("failed to find .{ext1} files in directory")
                } else {
                    format!("failed to find .{ext1} or .{ext2} files in directory")
                };
                return Err(Error::new(&message, &display, 0, 0));
            }
        } else if path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case(ext1))
        {
            paths1.push(path.to_path_buf());
        } else if path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case(ext2))
        {
            paths2.push(path.to_path_buf());
        } else {
            let message = if ext1 == ext2 {
                format!("expected .{ext1} file")
            } else {
                format!("expected .{ext1} or .{ext2} file")
            };
            return Err(Error::new(&message, &display, 0, 0));
        }
    }

    Ok((paths1, paths2))
}

/// Expands file and directory inputs containing one file type.
pub fn expand_input_files<P: AsRef<Path>>(
    inputs: &[P],
    extension: &str,
) -> Result<Vec<PathBuf>, Error> {
    Ok(expand_input_paths(inputs, extension, extension)?.0)
}

pub fn write_to_file<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> Result<(), Error> {
    let path = path.as_ref();
    let display = path.to_string_lossy();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|_| writer_err!("failed to create directory `{display}`"))?;
    }

    std::fs::write(path, contents).map_err(|_| writer_err!("failed to write file `{display}`"))
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

    #[test]
    fn property_less_pseudo_matches_by_name() {
        let pseudo = pseudo_for_metadata("RetValAttribute", &[])
            .expect("RetValAttribute should map to a pseudo");
        assert_eq!(pseudo.short, "retval");
    }
}
