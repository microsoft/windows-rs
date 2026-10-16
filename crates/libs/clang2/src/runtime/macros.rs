use super::*;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

struct Candidate {
    name: String,
    origin: String,
}

fn selected_header(headers: &HashSet<String>, file: &str) -> bool {
    Path::new(file)
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|header| headers.contains(&header.to_ascii_lowercase()))
}

fn empty_definition(unit: &Unit, definition: CXCursor, name: &str) -> bool {
    let mut tokens = std::ptr::null_mut();
    let mut count = 0;
    unsafe {
        clang_tokenize(
            unit.0,
            clang_getCursorExtent(definition),
            &raw mut tokens,
            &raw mut count,
        );
        let empty = count == 1
            && clang_getTokenKind(*tokens) == CXToken_Identifier
            && text(clang_getTokenSpelling(unit.0, *tokens)) == name;
        clang_disposeTokens(unit.0, tokens, count);
        empty
    }
}

pub(super) fn discover(
    index: &Index,
    unit: &Unit,
    source: &Source,
    input: &Input,
    arguments: &[String],
    target: Target,
) -> Result<(Vec<String>, partial::Issues), Error> {
    let headers = input
        .macro_headers
        .iter()
        .map(|header| header.to_ascii_lowercase())
        .collect();
    let mut candidates = BTreeMap::new();
    for cursor in native::children(unsafe { clang_getTranslationUnitCursor(unit.0) }) {
        if cursor.kind != CXCursor_MacroDefinition
            || unsafe { clang_Cursor_isMacroBuiltin(cursor) } != 0
            || unsafe { clang_Cursor_isMacroFunctionLike(cursor) } != 0
        {
            continue;
        }
        let (file, line, column, _) = native::source(cursor);
        if !selected_header(&headers, &file) {
            continue;
        }
        let name = text(unsafe { clang_getCursorSpelling(cursor) });
        if input.macros.contains(&name) {
            continue;
        }
        candidates.insert(
            name.clone(),
            Candidate {
                origin: format!("{file}:{line}:{column}: macro `{name}`"),
                name,
            },
        );
    }
    let mut issues = partial::Issues::default();
    let mut accepted = Vec::new();
    if candidates.is_empty() {
        return Ok((accepted, issues));
    }
    let mut pch = Precompiled::new(unit)?;
    let mut inherited = arguments.iter();
    let mut arguments = Vec::new();
    while let Some(argument) = inherited.next() {
        // The saved context already contains forced includes and their macro effects.
        if matches!(argument.as_str(), "-include" | "-imacros" | "-include-pch") {
            inherited
                .next()
                .ok_or_else(|| Error(format!("{argument} requires a filename")))?;
        } else if !argument.starts_with("-include") && !argument.starts_with("-imacros") {
            arguments.push(argument.clone());
        }
    }
    arguments.extend([
        "-ferror-limit=0".into(),
        "-include-pch".into(),
        pch.path.as_ref().unwrap().display().to_string(),
    ]);
    let mut probe = Input {
        name: format!("{}.macro-probe.cpp", input.name),
        ..Default::default()
    };
    // Conditional references query active bindings without expanding replacement tokens.
    // This parse must succeed before expression errors can be attributed to candidates.
    let mut positions = Vec::new();
    for candidate in candidates.into_values() {
        probe.source.push_str("#ifdef ");
        let offset: u32 = probe
            .source
            .len()
            .try_into()
            .map_err(|_| Error("source too large".into()))?;
        probe.source.push_str(&candidate.name);
        probe.source.push_str("\n#endif\n");
        positions.push((candidate, offset));
    }
    let control = macro_source(&probe, &[], "__windows_clang_probe")?;
    probe.source.clear();
    let bindings = parse_unit(
        index,
        &[&control, source],
        &arguments,
        CXTranslationUnit_DetailedPreprocessingRecord,
    )?;
    bindings.check()?;
    let file = unsafe { clang_getFile(bindings.unit.0, control.name.as_ptr()) };
    let mut candidates = Vec::new();
    for (mut candidate, offset) in positions {
        let cursor = unsafe {
            clang_getCursor(
                bindings.unit.0,
                clang_getLocationForOffset(bindings.unit.0, file, offset),
            )
        };
        if cursor.kind == CXCursor_NoDeclFound {
            issues.messages.insert(format!(
                "{target:?}: {}: macro is not defined at the end of the input",
                candidate.origin
            ));
            continue;
        }
        if cursor.kind != CXCursor_MacroExpansion
            || text(unsafe { clang_getCursorSpelling(cursor) }) != candidate.name
        {
            return Err(Error(format!(
                "cannot identify active macro `{}`",
                candidate.name
            )));
        }
        let definition = unsafe { clang_getCursorReferenced(cursor) };
        if definition.kind != CXCursor_MacroDefinition {
            return Err(Error(format!(
                "active macro `{}` has no definition",
                candidate.name
            )));
        }
        let (file, line, column, _) = native::source(definition);
        candidate.origin = format!("{file}:{line}:{column}: macro `{}`", candidate.name);
        let reason = if unsafe { clang_Cursor_isMacroFunctionLike(definition) } != 0 {
            Some("active definition is function-like")
        } else if !selected_header(&headers, &file) {
            Some("active definition is outside the selected macro headers")
        } else {
            None
        };
        if let Some(reason) = reason {
            issues
                .messages
                .insert(format!("{target:?}: {}: {reason}", candidate.origin));
        } else if empty_definition(&bindings.unit, definition, &candidate.name) {
            issues.reject(
                partial::Key::Macro(candidate.name),
                format!(
                    "{target:?}: {}: empty macro replacement is not an expression",
                    candidate.origin
                ),
            );
        } else {
            candidates.push(candidate);
        }
    }
    drop(bindings);
    let mut pending = Vec::new();
    if !candidates.is_empty() {
        pending.push(candidates);
    }
    while let Some(mut batch) = pending.pop() {
        let names: Vec<_> = batch.iter().map(|value| value.name.clone()).collect();
        let probe_source = macro_source(&probe, &names, "__windows_clang_probe")?;
        let parsed = parse_unit(
            index,
            &[&probe_source, source],
            &arguments,
            CXTranslationUnit_DetailedPreprocessingRecord,
        )?;
        if parsed.failed && batch.len() > 1 {
            let suspects = suspect_probes(&parsed, &probe_source);
            drop(parsed);
            let mut remaining = Vec::new();
            let count = batch.len();
            for (position, candidate) in batch.into_iter().enumerate() {
                if suspects.contains(&position) {
                    // Recovery can blame valid siblings. Confirm failure in isolation.
                    let isolated = macro_source(
                        &probe,
                        std::slice::from_ref(&candidate.name),
                        "__windows_clang_probe",
                    )?;
                    let checked = parse_unit(
                        index,
                        &[&isolated, source],
                        &arguments,
                        CXTranslationUnit_DetailedPreprocessingRecord,
                    )?;
                    if let Err(error) = checked.check() {
                        issues.reject(
                            partial::Key::Macro(candidate.name.clone()),
                            format!(
                                "{target:?}: {}: expression probe failed:\n{error}",
                                candidate.origin
                            ),
                        );
                        continue;
                    }
                }
                remaining.push(candidate);
            }
            if remaining.len() == count {
                let right = remaining.split_off(remaining.len() / 2);
                pending.push(right);
            }
            if !remaining.is_empty() {
                pending.push(remaining);
            }
            continue;
        }
        let checked = parsed.check().and_then(|()| {
            let file = unsafe { clang_getFile(parsed.unit.0, probe_source.name.as_ptr()) };
            probe_source.probes.iter().try_for_each(|probe| {
                capture::macro_definition(parsed.unit.0, file, probe).map(|_| ())
            })
        });
        match checked {
            Ok(()) => accepted.extend(names),
            Err(_) if batch.len() > 1 => {
                let right = batch.split_off(batch.len() / 2);
                pending.push(right);
                pending.push(batch);
            }
            Err(error) => {
                let candidate = &batch[0];
                issues.reject(
                    partial::Key::Macro(candidate.name.clone()),
                    format!(
                        "{target:?}: {}: expression probe failed:\n{error}",
                        candidate.origin
                    ),
                );
            }
        }
    }
    pch.remove()?;
    Ok((accepted, issues))
}

fn suspect_probes(parsed: &Parsed, source: &Source) -> HashSet<usize> {
    let file = unsafe { clang_getFile(parsed.unit.0, source.name.as_ptr()) };
    let locate = |location| {
        let mut found = std::ptr::null_mut();
        let mut offset = 0;
        unsafe {
            clang_getExpansionLocation(
                location,
                &raw mut found,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &raw mut offset,
            );
        }
        if found != file {
            return None;
        }
        source
            .probes
            .partition_point(|probe| probe.start <= offset)
            .checked_sub(1)
    };
    let mut rejected = HashSet::new();
    for id in 0..unsafe { clang_getNumDiagnostics(parsed.unit.0) } {
        let diagnostic = unsafe { clang_getDiagnostic(parsed.unit.0, id) };
        if unsafe { clang_getDiagnosticSeverity(diagnostic) } < CXDiagnostic_Error {
            unsafe { clang_disposeDiagnostic(diagnostic) };
            continue;
        }
        if let Some(attribution) = locate(unsafe { clang_getDiagnosticLocation(diagnostic) }) {
            rejected.insert(attribution);
        }
        unsafe { clang_disposeDiagnostic(diagnostic) };
    }
    rejected
}

struct Precompiled {
    path: Option<std::path::PathBuf>,
}

impl Precompiled {
    fn new(unit: &Unit) -> Result<Self, Error> {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|error| Error(error.to_string()))?
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "windows-clang2-{}-{stamp}-{}.pch",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::File::create_new(&path)
            .map_err(|error| Error(format!("{}: {error}", path.display())))?;
        let owned = Self { path: Some(path) };
        let name = CString::new(
            owned
                .path
                .as_ref()
                .unwrap()
                .to_str()
                .ok_or_else(|| Error("PCH path is not UTF-8".into()))?,
        )
        .map_err(|error| Error(error.to_string()))?;
        let status = unsafe {
            clang_saveTranslationUnit(unit.0, name.as_ptr(), clang_defaultSaveOptions(unit.0))
        };
        if status != CXSaveError_None {
            return Err(Error(format!(
                "cannot save macro discovery context: {status}"
            )));
        }
        Ok(owned)
    }

    fn remove(&mut self) -> Result<(), Error> {
        let path = self.path.take().unwrap();
        std::fs::remove_file(&path).map_err(|error| Error(format!("{}: {error}", path.display())))
    }
}

impl Drop for Precompiled {
    fn drop(&mut self) {
        if self.path.is_some()
            && let Err(error) = self.remove()
        {
            eprintln!("macro context cleanup failed: {error}");
        }
    }
}
