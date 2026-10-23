#![allow(non_upper_case_globals)]

use super::*;
use native::text;
use std::ffi::CString;
use std::sync::Mutex;
use windows_clang_sys::*;

mod macros;
mod profile;

static BATCH: Mutex<()> = Mutex::new(());

/// Captures each input as a separate TU, returning target-major, input-order snapshots.
///
/// Indices are created before workers start and reused across all inputs for their target.
/// Batches from this crate are serialized. Other libclang users must not create indices
/// during capture. No compiler handles escape.
pub fn capture(inputs: &[Input], targets: &[Target]) -> Result<Vec<Snapshot>, Error> {
    Ok(capture_impl(inputs, targets, false)?.0)
}

#[derive(Debug)]
pub struct PartialCapture {
    pub snapshots: Vec<Snapshot>,
    pub omissions: Vec<String>,
}

/// Captures supported declarations and reports omissions, including affected dependents.
/// Successfully captured observations are reconciled before exclusions remove their dependents.
/// Compiler errors and native conflicts remain fatal. Failed observations are not validated;
/// their declarations are removed across all targets and inputs.
pub fn capture_partial(inputs: &[Input], targets: &[Target]) -> Result<PartialCapture, Error> {
    let _profile = profile::begin("all-targets".into());
    let workers = profile::phase("workers");
    let (mut snapshots, mut issues) = capture_impl(inputs, targets, true)?;
    drop(workers);
    let validation = profile::phase("validation");
    merge(&snapshots)?;
    drop(validation);
    let pruning = profile::phase("pruning");
    partial::prune(&mut snapshots, &mut issues);
    drop(pruning);
    Ok(PartialCapture {
        snapshots,
        omissions: issues.finish(),
    })
}

fn capture_impl(
    inputs: &[Input],
    targets: &[Target],
    partial: bool,
) -> Result<(Vec<Snapshot>, partial::Issues), Error> {
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
            let mut macros = HashSet::new();
            for name in &input.macros {
                if name.is_empty()
                    || !name.bytes().enumerate().all(|(index, value)| {
                        value == b'_'
                            || value.is_ascii_alphabetic()
                            || index != 0 && value.is_ascii_digit()
                    })
                    || !macros.insert(name)
                {
                    return Err(Error(format!("invalid or duplicate macro name `{name}`")));
                }
            }
            for header in &input.macro_headers {
                if std::path::Path::new(header)
                    .file_name()
                    .and_then(|name| name.to_str())
                    != Some(header)
                {
                    return Err(Error(format!(
                        "macro header must be a filename: `{header}`"
                    )));
                }
            }
            macro_source(input, &input.macros, "__windows_clang_macro", None)
        })
        .collect::<Result<Vec<_>, Error>>()?;
    let _batch = BATCH.lock().map_err(|error| Error(error.to_string()))?;
    load().map_err(Error)?;
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
                let compiler = &compiler;
                let sources = &sources;
                scope.spawn(move || {
                    let mut results = Batch::default();
                    for (source, input) in sources.iter().zip(inputs) {
                        results.push(parse(&index, source, input, target, compiler, partial));
                    }
                    (index, results)
                })
            })
            .collect();
        tasks.into_iter().map(|task| task.join().unwrap()).unzip()
    });
    drop(indices);
    let mut batch = Batch::default();
    for result in results {
        batch.snapshots.extend(result.snapshots);
        batch.errors.extend(result.errors);
        batch.issues.extend(result.issues);
    }
    if batch.errors.is_empty() {
        Ok((batch.snapshots, batch.issues))
    } else {
        Err(Error(batch.errors.join("\n")))
    }
}

struct Source {
    name: CString,
    text: CString,
    probes: Vec<capture::MacroProbe>,
}

fn macro_source(
    input: &Input,
    macros: &[String],
    prefix: &str,
    functions: Option<&HashSet<String>>,
) -> Result<Source, Error> {
    let mut source = input.source.clone();
    let mut probes = Vec::new();
    for (index, name) in macros.iter().enumerate() {
        let start = source
            .len()
            .try_into()
            .map_err(|_| Error("source too large".into()))?;
        let function = functions.is_some_and(|functions| functions.contains(name));
        if functions.is_none() {
            source.push_str("\nusing ");
        } else if function {
            source.push_str(&format!("\nstatic __typeof__(({name})) &"));
        } else {
            source.push_str(&format!("\nstatic const __typeof__(({name})) "));
        }
        let declaration = source
            .len()
            .try_into()
            .map_err(|_| Error("source too large".into()))?;
        // Parentheses would turn initializer fragments into comma expressions.
        if functions.is_none() {
            source.push_str(&format!("{prefix}_type_{index} = __typeof__(("));
        } else {
            source.push_str(&format!("{prefix}_{index} = "));
        }
        let expansion = source
            .len()
            .try_into()
            .map_err(|_| Error("source too large".into()))?;
        source.push_str(name);
        source.push_str(if functions.is_none() { "));\n" } else { ";\n" });
        probes.push(capture::MacroProbe {
            name: name.clone(),
            start,
            declaration,
            expansion,
            function,
        });
    }
    Ok(Source {
        name: CString::new(input.name.as_str()).map_err(|error| Error(error.to_string()))?,
        text: CString::new(source).map_err(|error| Error(error.to_string()))?,
        probes,
    })
}

#[derive(Default)]
struct Batch {
    snapshots: Vec<Snapshot>,
    errors: Vec<String>,
    issues: partial::Issues,
}

impl Batch {
    fn push(&mut self, result: Result<(Snapshot, partial::Issues), Error>) {
        match result {
            Ok((snapshot, captured)) => {
                self.snapshots.push(snapshot);
                self.issues.extend(captured);
            }
            Err(error) => self.errors.push(error.to_string()),
        }
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
            let started = std::time::Instant::now();
            unsafe { clang_disposeTranslationUnit(self.0) };
            profile::record("dispose", started.elapsed());
        }
    }
}

struct Parsed {
    unit: Unit,
    diagnostics: Vec<String>,
    failed: bool,
}

impl Parsed {
    fn check(&self) -> Result<(), Error> {
        if self.failed {
            return Err(Error(format!(
                "libclang parse failed:\n{}",
                self.diagnostics.join("\n")
            )));
        }
        Ok(())
    }
}

fn parse_unit(
    index: &Index,
    sources: &[&Source],
    arguments: &[String],
    flags: CXTranslationUnit_Flags,
) -> Result<Parsed, Error> {
    let args = arguments
        .iter()
        .map(|arg| CString::new(arg.as_str()).map_err(|error| Error(error.to_string())))
        .collect::<Result<Vec<_>, _>>()?;
    let pointers: Vec<_> = args.iter().map(|arg| arg.as_ptr()).collect();
    let mut unsaved: Vec<_> = sources
        .iter()
        .map(|source| {
            Ok(CXUnsavedFile {
                Filename: source.name.as_ptr(),
                Contents: source.text.as_ptr(),
                Length: source
                    .text
                    .as_bytes()
                    .len()
                    .try_into()
                    .map_err(|_| Error("source too large".into()))?,
            })
        })
        .collect::<Result<_, Error>>()?;
    let mut unit = Unit(std::ptr::null_mut());
    let started = std::time::Instant::now();
    let status = unsafe {
        clang_parseTranslationUnit2(
            index.0,
            sources[0].name.as_ptr(),
            pointers.as_ptr(),
            pointers.len().try_into().unwrap(),
            unsaved.as_mut_ptr(),
            unsaved.len().try_into().unwrap(),
            flags.try_into().unwrap(),
            &raw mut unit.0,
        )
    };
    profile::record("native-parse", started.elapsed());
    if unit.0.is_null() || status != CXError_Success {
        return Err(Error(format!("libclang parse failed: {status}")));
    }
    let mut diagnostics = Vec::new();
    let mut failed = false;
    let started = std::time::Instant::now();
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
        profile::count("diagnostic-count", diagnostics.len());
        profile::record("diagnostics", started.elapsed());
    }
    Ok(Parsed {
        unit,
        diagnostics,
        failed,
    })
}

fn parse(
    index: &Index,
    source: &Source,
    input: &Input,
    target: Target,
    compiler: &str,
    partial: bool,
) -> Result<(Snapshot, partial::Issues), Error> {
    let _profile = profile::begin(format!("{target:?} {}", input.name));
    let mut arguments = vec!["-x".into(), "c++".into(), "-std=c++20".into()];
    arguments.extend(input.arguments.iter().cloned());
    arguments.push(format!("--target={}", target.triple()));
    let context = format!(
        "{} [{}] {}",
        source.name.to_string_lossy(),
        target.triple(),
        arguments.join(" ")
    );
    let parse = || {
        let flags = CXTranslationUnit_IncludeAttributedTypes
            | CXTranslationUnit_VisitImplicitAttributes
            | if input.macros.is_empty() && input.macro_headers.is_empty() {
                0
            } else {
                CXTranslationUnit_DetailedPreprocessingRecord
            };
        let initial = profile::phase("initial");
        let mut parsed = parse_unit(index, &[source], &arguments, flags)?;
        parsed.check()?;
        drop(initial);
        let mut issues = partial::Issues::default();
        let mut functions = macros::function_probes(&parsed.unit, source)?;
        let mut names = input.macros.clone();
        if !input.macro_headers.is_empty() {
            let _phase = profile::phase("discovery");
            let (discovered, aliases, rejected) =
                macros::discover(&parsed.unit, source, input, &arguments, target)?;
            if !partial && !rejected.excluded.is_empty() {
                return Err(Error(rejected.finish().join("\n")));
            }
            issues.extend(rejected);
            names.extend(discovered);
            functions.extend(aliases);
        }
        let generated = if names.is_empty() {
            None
        } else {
            Some(macro_source(
                input,
                &names,
                "__windows_clang_macro",
                Some(&functions),
            )?)
        };
        if let Some(source) = &generated {
            let _phase = profile::phase("final");
            parsed = parse_unit(index, &[source], &arguments, flags)?;
            parsed.check()?;
        }
        let source = generated.as_ref().unwrap_or(source);
        let _phase = profile::phase("capture");
        let mut snapshot = Snapshot {
            input: source.name.to_string_lossy().into_owned(),
            target,
            compiler: compiler.to_string(),
            arguments: arguments.clone(),
            diagnostics: parsed.diagnostics,
            roots: Vec::new(),
            files: Vec::new(),
            records: Vec::new(),
            aliases: Vec::new(),
            enums: Vec::new(),
            values: Vec::new(),
            inline_functions: Vec::new(),
            observations: Vec::new(),
            types: Vec::new(),
        };
        snapshot.diagnostics.extend(issues.messages.iter().cloned());
        issues.extend(capture::read(
            parsed.unit.0,
            input,
            &source.probes,
            &mut snapshot,
            partial,
        )?);
        Ok((snapshot, issues))
    };
    parse().map_err(|error| Error(format!("{context}\n{error}")))
}
