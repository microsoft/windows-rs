use super::*;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

struct Candidate {
    name: String,
    origin: String,
}

fn selected_header(input: &Input, file: &str) -> bool {
    let header = Path::new(file).file_name().and_then(|name| name.to_str());
    input
        .macro_headers
        .iter()
        .any(|name| header.is_some_and(|header| header.eq_ignore_ascii_case(name)))
}

pub(super) fn discover(
    index: &Index,
    unit: &Unit,
    source: &Source,
    input: &Input,
    arguments: &[String],
    target: Target,
) -> Result<(Vec<String>, partial::Issues), Error> {
    let mut candidates = BTreeMap::new();
    for cursor in native::children(unsafe { clang_getTranslationUnitCursor(unit.0) }) {
        if cursor.kind != CXCursor_MacroDefinition
            || unsafe { clang_Cursor_isMacroBuiltin(cursor) } != 0
            || unsafe { clang_Cursor_isMacroFunctionLike(cursor) } != 0
        {
            continue;
        }
        let (file, line, column, _) = native::source(cursor);
        if !selected_header(input, &file) {
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
        } else if !selected_header(input, &file) {
            Some("active definition is outside the selected macro headers")
        } else {
            None
        };
        if let Some(reason) = reason {
            issues
                .messages
                .insert(format!("{target:?}: {}: {reason}", candidate.origin));
        } else {
            candidates.push(candidate);
        }
    }
    drop(bindings);
    let mut pending = Vec::new();
    if !candidates.is_empty() {
        pending.push(candidates.as_slice());
    }
    while let Some(batch) = pending.pop() {
        let names: Vec<_> = batch.iter().map(|value| value.name.clone()).collect();
        let probe_source = macro_source(&probe, &names, "__windows_clang_probe")?;
        let parsed = parse_unit(
            index,
            &[&probe_source, source],
            &arguments,
            CXTranslationUnit_DetailedPreprocessingRecord,
        )?;
        let checked = parsed.check().and_then(|()| {
            let file = unsafe { clang_getFile(parsed.unit.0, probe_source.name.as_ptr()) };
            probe_source.probes.iter().try_for_each(|probe| {
                capture::macro_definition(parsed.unit.0, file, probe).map(|_| ())
            })
        });
        match checked {
            Ok(()) => accepted.extend(names),
            Err(_) if batch.len() > 1 => {
                let (left, right) = batch.split_at(batch.len() / 2);
                pending.push(right);
                pending.push(left);
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
