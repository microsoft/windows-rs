use super::*;
use std::path::Path;

pub(super) fn function_probes(unit: &Unit, source: &Source) -> Result<HashSet<String>, Error> {
    let mut functions = HashSet::new();
    if source.probes.is_empty() {
        return Ok(functions);
    }
    let file = unsafe { clang_getFile(unit.0, source.name.as_ptr()) };
    for probe in &source.probes {
        let declaration = unsafe {
            clang_getCursor(
                unit.0,
                clang_getLocationForOffset(unit.0, file, probe.declaration),
            )
        };
        let mut expression = unsafe { clang_getNullCursor() };
        let status = unsafe { windows_clang_getTypeOfOperand(declaration, &raw mut expression) };
        if status != WindowsClangSuccess {
            return Err(Error(format!(
                "macro `{}` expression type unavailable: {status}",
                probe.name
            )));
        }
        let kind = unsafe { clang_getCanonicalType(clang_getCursorType(expression)) }.kind;
        if matches!(kind, CXType_FunctionProto | CXType_FunctionNoProto) {
            functions.insert(probe.name.clone());
        }
    }
    Ok(functions)
}

fn selected_header(headers: &HashSet<String>, file: &str) -> bool {
    Path::new(file)
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|header| headers.contains(&header.to_ascii_lowercase()))
}

pub(super) fn discover(
    unit: &Unit,
    source: &Source,
    input: &Input,
    arguments: &[String],
    target: Target,
) -> Result<(Vec<String>, HashSet<String>, partial::Issues), Error> {
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
        if !input.macros.contains(&name) {
            let origin = format!("{file}:{line}:{column}: macro `{name}`");
            candidates.insert(name, origin);
        }
    }
    let mut issues = partial::Issues::default();
    let mut accepted = Vec::new();
    let mut functions = HashSet::new();
    if candidates.is_empty() {
        return Ok((accepted, functions, issues));
    }
    profile::count("candidates", candidates.len());
    let names: Vec<_> = candidates.keys().map(String::as_str).collect();
    let probes = probe(source, arguments, &names)?;
    for ((name, mut origin), probe) in candidates.into_iter().zip(probes) {
        let Probe {
            kind,
            file,
            line,
            column,
            diagnostics,
            ..
        } = probe;
        if kind != WindowsClangMacroUndefined {
            origin = format!("{file}:{line}:{column}: macro `{name}`");
        }
        let reason = match kind {
            WindowsClangMacroUndefined => Some("macro is not defined at the end of the input"),
            WindowsClangMacroFunctionLike => Some("active definition is function-like"),
            _ if !selected_header(&headers, &file) => {
                Some("active definition is outside the selected macro headers")
            }
            _ => None,
        };
        if let Some(reason) = reason {
            issues
                .messages
                .insert(format!("{target:?}: {origin}: {reason}"));
            continue;
        }
        match kind {
            WindowsClangMacroEmpty => {
                profile::count("empty", 1);
                issues.reject(
                    partial::Key::Macro(name),
                    format!("{target:?}: {origin}: empty macro replacement is not an expression"),
                );
            }
            WindowsClangMacroRejected => {
                profile::count("rejected", 1);
                issues.reject(
                    partial::Key::Macro(name),
                    format!("{target:?}: {origin}: expression probe failed:\n{diagnostics}"),
                );
            }
            WindowsClangMacroValue | WindowsClangMacroFunction => {
                if kind == WindowsClangMacroFunction {
                    functions.insert(name.clone());
                }
                accepted.push(name);
            }
            _ => {
                return Err(Error(format!(
                    "macro `{name}` has unknown probe result {kind}"
                )));
            }
        }
    }
    Ok((accepted, functions, issues))
}

/// Declarations that function-macro aliases name under each of the input's alias variants.
pub(super) fn alias_variants(
    source: &Source,
    input: &Input,
    arguments: &[String],
    names: &[&str],
) -> Result<Vec<Vec<Option<String>>>, Error> {
    let mut targets = vec![Vec::with_capacity(input.alias_variants.len()); names.len()];
    for variant in &input.alias_variants {
        let mut arguments = arguments.to_vec();
        arguments.extend(variant.iter().cloned());
        for (targets, probe) in targets.iter_mut().zip(probe(source, &arguments, names)?) {
            targets.push((probe.kind == WindowsClangMacroFunction).then_some(probe.target));
        }
    }
    Ok(targets)
}

struct Probe {
    kind: WindowsClangMacroKind,
    file: String,
    line: u32,
    column: u32,
    diagnostics: String,
    target: String,
}

fn probe(source: &Source, arguments: &[String], names: &[&str]) -> Result<Vec<Probe>, Error> {
    let names = names
        .iter()
        .map(|name| CString::new(*name).map_err(|error| Error(error.to_string())))
        .collect::<Result<Vec<_>, _>>()?;
    let names: Vec<_> = names.iter().map(|name| name.as_ptr()).collect();
    let args = arguments
        .iter()
        .map(|arg| CString::new(arg.as_str()).map_err(|error| Error(error.to_string())))
        .collect::<Result<Vec<_>, _>>()?;
    let args: Vec<_> = args.iter().map(|arg| arg.as_ptr()).collect();
    let mut unsaved = CXUnsavedFile {
        Filename: source.name.as_ptr(),
        Contents: source.text.as_ptr(),
        Length: source
            .text
            .as_bytes()
            .len()
            .try_into()
            .map_err(|_| Error("source too large".into()))?,
    };
    let mut probes = vec![WindowsClangMacroProbe::default(); names.len()];
    let mut error = CXString::default();
    // One incremental parse checks every candidate's active definition in its own probe input.
    let status = unsafe {
        windows_clang_probeMacros(
            source.name.as_ptr(),
            args.as_ptr(),
            args.len().try_into().unwrap(),
            &raw mut unsaved,
            1,
            names.as_ptr(),
            names.len().try_into().unwrap(),
            probes.as_mut_ptr(),
            &raw mut error,
        )
    };
    // Zeroed strings are unmanaged, so every string can be taken before an early return.
    let error = text(error);
    let probes: Vec<_> = probes
        .into_iter()
        .map(|probe| Probe {
            kind: probe.kind,
            file: text(probe.file),
            line: probe.line,
            column: probe.column,
            diagnostics: text(probe.diagnostics),
            target: text(probe.target),
        })
        .collect();
    if status != WindowsClangSuccess {
        return Err(Error(format!(
            "macro discovery parse failed: {status}\n{error}"
        )));
    }
    Ok(probes)
}
