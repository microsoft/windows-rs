#![allow(non_upper_case_globals)]

use super::*;
use clang_sys::*;
use native::text;
use std::ffi::CString;
use std::sync::{Arc, Mutex};

static BATCH: Mutex<()> = Mutex::new(());

/// Captures each input as a separate TU, returning target-major, input-order snapshots.
///
/// Indices are created before workers start and reused across all inputs for their target.
/// Batches from this crate are serialized. Other libclang users must not create indices
/// during capture. No compiler handles escape.
pub fn capture(inputs: &[Input], targets: &[Target]) -> Result<Vec<Snapshot>, Error> {
    if inputs.is_empty() || targets.is_empty() {
        return Err(Error("capture requires inputs and targets".into()));
    }
    let mut names = HashSet::new();
    let sources = inputs
        .iter()
        .map(|input| {
            if !names.insert(&input.name) {
                return Err(Error(format!("duplicate input name `{}`", input.name)));
            }
            Ok((
                CString::new(input.name.as_str()).map_err(|error| Error(error.to_string()))?,
                CString::new(input.source.as_str()).map_err(|error| Error(error.to_string()))?,
            ))
        })
        .collect::<Result<Vec<_>, Error>>()?;
    let _batch = BATCH.lock().map_err(|error| Error(error.to_string()))?;
    let library = Arc::new(load_manually().map_err(Error)?);
    let _binding = Binding::new(Arc::clone(&library));
    let compiler = text(unsafe { clang_getClangVersion() });
    let indices = targets
        .iter()
        .map(|_| {
            let handle = unsafe { clang_createIndex(0, 0) };
            if handle.is_null() {
                Err(Error("clang_createIndex failed".into()))
            } else {
                Ok(Index(handle))
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    let (indices, results): (Vec<_>, Vec<_>) = std::thread::scope(|scope| {
        let tasks: Vec<_> = indices
            .into_iter()
            .zip(targets)
            .map(|(index, &target)| {
                let library = Arc::clone(&library);
                let compiler = &compiler;
                let sources = &sources;
                scope.spawn(move || {
                    let _binding = Binding::new(library);
                    // Dispose before restoring bindings if this worker unwinds.
                    let owned_index = index;
                    let results: Vec<_> = sources
                        .iter()
                        .map(|(name, source)| parse(&owned_index, name, source, target, compiler))
                        .collect();
                    (owned_index, results)
                })
            })
            .collect();
        tasks.into_iter().map(|task| task.join().unwrap()).unzip()
    });
    drop(indices);
    let mut snapshots = Vec::new();
    let mut errors = Vec::new();
    for result in results.into_iter().flatten() {
        match result {
            Ok(snapshot) => snapshots.push(snapshot),
            Err(error) => errors.push(error.to_string()),
        }
    }
    if errors.is_empty() {
        Ok(snapshots)
    } else {
        Err(Error(errors.join("\n")))
    }
}

struct Binding(Option<Arc<SharedLibrary>>);

impl Binding {
    fn new(library: Arc<SharedLibrary>) -> Self {
        Self(set_library(Some(library)))
    }
}

impl Drop for Binding {
    fn drop(&mut self) {
        set_library(self.0.take());
    }
}

struct Index(CXIndex);

// An index is transferred to one worker and never accessed concurrently.
unsafe impl Send for Index {}

impl Drop for Index {
    fn drop(&mut self) {
        unsafe { clang_disposeIndex(self.0) };
    }
}

struct Unit(CXTranslationUnit);

impl Drop for Unit {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { clang_disposeTranslationUnit(self.0) };
        }
    }
}

fn parse(
    index: &Index,
    name: &CString,
    source: &CString,
    target: Target,
    compiler: &str,
) -> Result<Snapshot, Error> {
    let arguments = vec![
        "-x".into(),
        "c++".into(),
        "-std=c++20".into(),
        format!("--target={}", target.triple()),
    ];
    let context = format!(
        "{} [{}] {}",
        name.to_string_lossy(),
        target.triple(),
        arguments.join(" ")
    );
    let parse = || {
        let args: Vec<_> = arguments
            .iter()
            .map(|arg| CString::new(arg.as_str()).unwrap())
            .collect();
        let pointers: Vec<_> = args.iter().map(|arg| arg.as_ptr()).collect();
        let mut unsaved = CXUnsavedFile {
            Filename: name.as_ptr(),
            Contents: source.as_ptr(),
            Length: source
                .as_bytes()
                .len()
                .try_into()
                .map_err(|_| Error("source too large".into()))?,
        };
        let mut unit = Unit(std::ptr::null_mut());
        let status = unsafe {
            clang_parseTranslationUnit2(
                index.0,
                name.as_ptr(),
                pointers.as_ptr(),
                pointers.len().try_into().unwrap(),
                &raw mut unsaved,
                1,
                0,
                &raw mut unit.0,
            )
        };
        if unit.0.is_null() {
            return Err(Error(format!("libclang parse failed: {status}")));
        }
        let mut diagnostics = Vec::new();
        let mut failed = status != CXError_Success;
        unsafe {
            for id in 0..clang_getNumDiagnostics(unit.0) {
                let diagnostic = clang_getDiagnostic(unit.0, id);
                failed |= clang_getDiagnosticSeverity(diagnostic) >= CXDiagnostic_Error;
                diagnostics.push(text(clang_formatDiagnostic(
                    diagnostic,
                    clang_defaultDiagnosticDisplayOptions(),
                )));
                clang_disposeDiagnostic(diagnostic);
            }
        }
        if failed {
            return Err(Error(format!(
                "libclang parse failed ({status}):\n{}",
                diagnostics.join("\n")
            )));
        }
        let mut snapshot = Snapshot {
            input: name.to_string_lossy().into_owned(),
            target,
            compiler: compiler.to_string(),
            arguments: arguments.clone(),
            diagnostics,
            files: Vec::new(),
            records: Vec::new(),
            observations: Vec::new(),
            types: Vec::new(),
        };
        capture::read(unit.0, &mut snapshot)?;
        Ok(snapshot)
    };
    parse().map_err(|error| Error(format!("{context}\n{error}")))
}
