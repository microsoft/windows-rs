#![allow(non_upper_case_globals)]

use super::*;
use clang_sys::*;
use std::ffi::{CStr, CString};
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

pub struct Declaration {
    pub scope: &'static str,
    pub kind: String,
    pub name: String,
    pub qualified: String,
    pub usr: String,
    pub canonical_usr: String,
    pub parent_usr: String,
    pub alias_usr: String,
    pub header: PathBuf,
    pub line: u32,
    pub column: u32,
    pub offset: u32,
    pub spelling_header: String,
    pub spelling_offset: u32,
    pub written_type: String,
    pub canonical_type: String,
    pub definition: bool,
    pub inline: bool,
    pub function_macro: bool,
}

pub struct Capture {
    pub declarations: Vec<Declaration>,
    pub files: Vec<PathBuf>,
    pub diagnostics: Vec<(CXDiagnosticSeverity, String)>,
    pub complete: bool,
    pub target: String,
    pub pointer_bits: i32,
}

struct Unit {
    raw: CXTranslationUnit,
    index: CXIndex,
}

impl Drop for Unit {
    fn drop(&mut self) {
        unsafe {
            if !self.raw.is_null() {
                clang_disposeTranslationUnit(self.raw);
            }
            if !self.index.is_null() {
                clang_disposeIndex(self.index);
            }
        }
    }
}

pub fn capture(profile: &Profile, input: &Path) -> Result<Capture> {
    if !is_loaded() {
        load().map_err(|error| format!("failed to load libclang: {error}"))?;
    }
    let input = CString::new(input.to_str().ok_or("non-UTF-8 translation unit path")?)?;
    let source = CString::new(profile.source.as_str())?;
    let arguments = profile
        .arguments
        .iter()
        .map(|value| CString::new(value.as_str()))
        .collect::<std::result::Result<Vec<_>, _>>()?;
    let arguments = arguments
        .iter()
        .map(|value| value.as_ptr())
        .collect::<Vec<_>>();
    let count = arguments.len().try_into()?;
    let mut unsaved = CXUnsavedFile {
        Filename: input.as_ptr(),
        Contents: source.as_ptr(),
        Length: profile.source.len().try_into()?,
    };
    let mut unit = Unit {
        raw: std::ptr::null_mut(),
        index: unsafe { clang_createIndex(0, 0) },
    };
    if unit.index.is_null() {
        return Err("libclang could not create an index".into());
    }
    let status = unsafe {
        clang_parseTranslationUnit2(
            unit.index,
            input.as_ptr(),
            arguments.as_ptr(),
            count,
            &raw mut unsaved,
            1,
            CXTranslationUnit_DetailedPreprocessingRecord | CXTranslationUnit_SkipFunctionBodies,
            &raw mut unit.raw,
        )
    };
    if status != CXError_Success || unit.raw.is_null() {
        return Err(format!("libclang parse failed with CXErrorCode {status}").into());
    }
    let mut diagnostics = Vec::new();
    for index in 0..unsafe { clang_getNumDiagnostics(unit.raw) } {
        let diagnostic = unsafe { clang_getDiagnostic(unit.raw, index) };
        let severity = unsafe { clang_getDiagnosticSeverity(diagnostic) };
        let text = text(unsafe {
            clang_formatDiagnostic(diagnostic, clang_defaultDiagnosticDisplayOptions())
        });
        unsafe { clang_disposeDiagnostic(diagnostic) };
        diagnostics.push((severity, text));
    }
    let mut files = Collector::default();
    extern "C" fn include(file: CXFile, _: *mut CXSourceLocation, _: u32, data: CXClientData) {
        unsafe { &mut *data.cast::<Collector<CXFile>>() }.push(file);
    }
    unsafe { clang_getInclusions(unit.raw, include, (&raw mut files).cast()) };
    let files = files.finish();
    let paths = files
        .iter()
        .map(|file| PathBuf::from(text(unsafe { clang_getFileName(*file) })))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let target = unsafe { clang_getTranslationUnitTargetInfo(unit.raw) };
    if target.is_null() {
        return Err("compiler did not report a translation-unit target".into());
    }
    let triple = text(unsafe { clang_TargetInfo_getTriple(target) });
    let pointer_bits = unsafe { clang_TargetInfo_getPointerWidth(target) };
    unsafe { clang_TargetInfo_dispose(target) };
    let (expected, width) = match profile.architecture {
        1 => ("i686-pc-windows-msvc", 32),
        2 => ("x86_64-pc-windows-msvc", 64),
        4 => ("aarch64-pc-windows-msvc", 64),
        _ => return Err("source profile must select one target architecture".into()),
    };
    if !triple.starts_with(expected) || pointer_bits != width {
        return Err(format!(
            "compiler target `{triple}`/{pointer_bits} does not match `{expected}`/{width}"
        )
        .into());
    }
    let mut result = Capture {
        declarations: Vec::new(),
        files: paths,
        complete: !diagnostics
            .iter()
            .any(|(severity, _)| *severity >= CXDiagnostic_Error),
        diagnostics,
        target: triple,
        pointer_bits,
    };
    if !result.complete {
        return Ok(result);
    }
    let file = |path: &Path| -> Result<CXFile> {
        let path = CString::new(path.to_str().ok_or("non-UTF-8 scope path")?)?;
        Ok(unsafe { clang_getFile(unit.raw, path.as_ptr()) })
    };
    let roots = profile
        .roots
        .iter()
        .map(|path| {
            let root = file(path)?;
            if root.is_null()
                || !files
                    .iter()
                    .any(|included| unsafe { clang_File_isEqual(root, *included) } != 0)
            {
                return Err(format!("requested root `{}` was not included", path.display()).into());
            }
            Ok(root)
        })
        .collect::<Result<Vec<_>>>()?;
    let excluded = profile
        .excluded
        .iter()
        .map(|path| file(path))
        .collect::<Result<Vec<_>>>()?;
    let dirs = profile
        .root_dirs
        .iter()
        .map(std::fs::canonicalize)
        .collect::<std::io::Result<Vec<_>>>()?;
    let main = unsafe { clang_getFile(unit.raw, input.as_ptr()) };
    let mut scopes = Vec::new();
    for included in &files {
        if !main.is_null() && unsafe { clang_File_isEqual(main, *included) } != 0 {
            scopes.push((*included, "translation-unit"));
            continue;
        }
        let path = PathBuf::from(text(unsafe { clang_getFileName(*included) }));
        let path = std::fs::canonicalize(path)?;
        let equal = |candidate: &CXFile| {
            !candidate.is_null() && unsafe { clang_File_isEqual(*included, *candidate) } != 0
        };
        let scope = if excluded.iter().any(equal) {
            "excluded"
        } else if roots.iter().any(equal) || dirs.iter().any(|dir| path.starts_with(dir)) {
            "root"
        } else {
            "dependency"
        };
        scopes.push((*included, scope));
    }
    let mut pending = children(unsafe { clang_getTranslationUnitCursor(unit.raw) });
    while let Some(cursor) = pending.pop() {
        let kind = unsafe { clang_getCursorKind(cursor) };
        if matches!(
            kind,
            CXCursor_Namespace
                | CXCursor_LinkageSpec
                | CXCursor_UnexposedDecl
                | CXCursor_StructDecl
                | CXCursor_UnionDecl
                | CXCursor_ClassDecl
                | CXCursor_EnumDecl
                | CXCursor_FunctionDecl
                | CXCursor_CXXMethod
                | CXCursor_Constructor
                | CXCursor_Destructor
                | CXCursor_FunctionTemplate
                | CXCursor_ClassTemplate
        ) {
            pending.extend(children(cursor));
        }
        if unsafe { clang_isDeclaration(kind) } == 0 && kind != CXCursor_MacroDefinition {
            continue;
        }
        let location = unsafe { clang_getCursorLocation(cursor) };
        let mut file = std::ptr::null_mut();
        let (mut line, mut column, mut offset) = (0, 0, 0);
        unsafe {
            clang_getExpansionLocation(
                location,
                &raw mut file,
                &raw mut line,
                &raw mut column,
                &raw mut offset,
            );
        }
        if file.is_null() {
            continue;
        }
        let scope = scopes
            .iter()
            .find(|(included, _)| unsafe { clang_File_isEqual(file, *included) } != 0)
            .map(|(_, scope)| *scope)
            .ok_or("declaration file is absent from the compiler inclusion graph")?;
        let header = PathBuf::from(text(unsafe { clang_getFileName(file) }));
        let mut spelling_file = std::ptr::null_mut();
        let mut spelling_offset = 0;
        unsafe {
            clang_getSpellingLocation(
                location,
                &raw mut spelling_file,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &raw mut spelling_offset,
            );
        }
        let native_type = unsafe { clang_getCursorType(cursor) };
        let canonical_type = unsafe { clang_getCanonicalType(native_type) };
        result.declarations.push(Declaration {
            scope,
            kind: if kind == CXCursor_MacroDefinition {
                "macro-definition".into()
            } else {
                text(unsafe { clang_getCursorKindSpelling(kind) })
            },
            name: text(unsafe { clang_getCursorSpelling(cursor) }),
            qualified: qualified_name(cursor),
            usr: text(unsafe { clang_getCursorUSR(cursor) }),
            canonical_usr: text(unsafe { clang_getCursorUSR(clang_getCanonicalCursor(cursor)) }),
            parent_usr: text(unsafe { clang_getCursorUSR(clang_getCursorSemanticParent(cursor)) }),
            alias_usr: if matches!(kind, CXCursor_TypedefDecl | CXCursor_TypeAliasDecl) {
                text(unsafe {
                    clang_getCursorUSR(clang_getTypeDeclaration(clang_getCanonicalType(
                        clang_getTypedefDeclUnderlyingType(cursor),
                    )))
                })
            } else {
                String::new()
            },
            header,
            line,
            column,
            offset,
            spelling_header: if spelling_file.is_null() {
                String::new()
            } else {
                text(unsafe { clang_getFileName(spelling_file) })
            },
            spelling_offset,
            written_type: text(unsafe { clang_getTypeSpelling(native_type) }),
            canonical_type: text(unsafe { clang_getTypeSpelling(canonical_type) }),
            definition: unsafe { clang_isCursorDefinition(cursor) } != 0,
            inline: matches!(kind, CXCursor_FunctionDecl | CXCursor_CXXMethod)
                && unsafe { clang_Cursor_isFunctionInlined(cursor) } != 0,
            function_macro: kind == CXCursor_MacroDefinition
                && unsafe { clang_Cursor_isMacroFunctionLike(cursor) } != 0,
        });
    }
    result.declarations.sort_by(|left, right| {
        (&left.header, left.offset, &left.kind, &left.usr, &left.name).cmp(&(
            &right.header,
            right.offset,
            &right.kind,
            &right.usr,
            &right.name,
        ))
    });
    Ok(result)
}

fn qualified_name(cursor: CXCursor) -> String {
    let mut names = vec![text(unsafe { clang_getCursorSpelling(cursor) })];
    let mut parent = unsafe { clang_getCursorSemanticParent(cursor) };
    while unsafe { clang_Cursor_isNull(parent) } == 0
        && unsafe { clang_getCursorKind(parent) } != CXCursor_TranslationUnit
    {
        let name = text(unsafe { clang_getCursorSpelling(parent) });
        if !name.is_empty() {
            names.push(name);
        }
        parent = unsafe { clang_getCursorSemanticParent(parent) };
    }
    names.reverse();
    names.join("::")
}

pub fn text(value: CXString) -> String {
    unsafe {
        let pointer = clang_getCString(value);
        let result = if pointer.is_null() {
            String::new()
        } else {
            CStr::from_ptr(pointer).to_string_lossy().into_owned()
        };
        clang_disposeString(value);
        result
    }
}

struct Collector<T> {
    values: Vec<T>,
    panic: Option<Box<dyn std::any::Any + Send>>,
}

impl<T> Default for Collector<T> {
    fn default() -> Self {
        Self {
            values: Vec::new(),
            panic: None,
        }
    }
}

impl<T> Collector<T> {
    fn push(&mut self, value: T) -> bool {
        if self.panic.is_some() {
            return false;
        }
        match catch_unwind(AssertUnwindSafe(|| self.values.push(value))) {
            Ok(()) => true,
            Err(panic) => {
                self.panic = Some(panic);
                false
            }
        }
    }

    fn finish(self) -> Vec<T> {
        if let Some(panic) = self.panic {
            resume_unwind(panic);
        }
        self.values
    }
}

fn children(cursor: CXCursor) -> Vec<CXCursor> {
    extern "C" fn visit(cursor: CXCursor, _: CXCursor, data: CXClientData) -> CXChildVisitResult {
        if unsafe { &mut *data.cast::<Collector<CXCursor>>() }.push(cursor) {
            CXChildVisit_Continue
        } else {
            CXChildVisit_Break
        }
    }
    let mut state = Collector::default();
    unsafe { clang_visitChildren(cursor, visit, (&raw mut state).cast()) };
    state.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiler_source_identity_and_clean_evidence() {
        let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
        let root = fixtures.join("source.h");
        let mut profile = Profile {
            name: "fixture".into(),
            family: "sdk",
            architecture: 2,
            source: "#include \"source.h\"\n".into(),
            arguments: vec![
                "-x".into(),
                "c++".into(),
                "-std=c++20".into(),
                "--target=x86_64-pc-windows-msvc".into(),
                "-I".into(),
                fixtures.to_string_lossy().into_owned(),
            ],
            roots: vec![root.clone()],
            root_dirs: Vec::new(),
            excluded: Vec::new(),
        };
        let input = fixtures.join("unsaved.hpp");
        let captured = capture(&profile, &input).unwrap();
        assert!(captured.complete, "{:?}", captured.diagnostics);
        assert!(captured.target.starts_with("x86_64-pc-windows-msvc"));
        assert_eq!(captured.pointer_bits, 64);
        let named = |name: &str| {
            captured
                .declarations
                .iter()
                .filter(|item| item.name == name)
                .collect::<Vec<_>>()
        };
        assert_eq!(named("DependencyRecord")[0].scope, "dependency");
        assert_eq!(named("OtherRecord")[0].scope, "dependency");
        assert_eq!(named("RecordAlias")[0].scope, "root");
        let definition = named("NativeRecord")
            .into_iter()
            .find(|item| item.definition)
            .unwrap();
        assert_eq!(named("RecordAlias")[0].alias_usr, definition.canonical_usr);
        assert_eq!(
            named("value")
                .into_iter()
                .filter(|item| item.kind == "FieldDecl" && item.parent_usr == definition.usr)
                .count(),
            1
        );
        let echo = named("Echo");
        let identities = echo
            .iter()
            .filter(|item| item.kind == "FunctionDecl")
            .map(|item| &item.usr)
            .collect::<BTreeSet<_>>();
        assert_eq!(identities.len(), 2);
        let repeated = named("Repeated");
        assert_eq!(repeated.len(), 2);
        assert_eq!(repeated[0].canonical_usr, repeated[1].canonical_usr);
        assert_ne!(repeated[0].offset, repeated[1].offset);
        assert_eq!(named("VALUE").len(), 2);
        assert!(named("HELPER")[0].function_macro);
        assert!(!named("VALUE")[0].function_macro);
        assert_eq!(named("FromMacro")[0].scope, "root");
        assert!(named("FromMacro")[0].line > 0);
        assert!(named("FromMacro")[0].spelling_offset > 0);
        let candidates = candidate_index(&captured, "source.h");
        assert!(!candidates.contains_key("OtherRecord"));
        assert_eq!(candidates["Echo"].len(), 2);
        let alias = named("RecordAlias")[0];
        assert_eq!(
            classify::group(
                "struct",
                "source.h",
                "ambiguous-candidate",
                &[(0, definition), (1, alias)],
                &[]
            ),
            "tag-and-typedef"
        );
        assert_eq!(
            classify::group(
                "function",
                "source.h",
                "ambiguous-candidate",
                &candidates["Echo"],
                &[]
            ),
            "distinct-callables"
        );

        profile.root_dirs.push(fixtures.clone());
        profile.excluded.push(root);
        let captured = capture(&profile, &input).unwrap();
        assert_eq!(
            captured
                .declarations
                .iter()
                .find(|item| item.name == "RecordAlias")
                .unwrap()
                .scope,
            "excluded"
        );
        assert_eq!(
            captured
                .declarations
                .iter()
                .find(|item| item.name == "OtherRecord")
                .unwrap()
                .scope,
            "root"
        );

        profile.excluded.clear();
        profile.roots.push(fixtures.join("invalid.h"));
        assert!(
            capture(&profile, &input)
                .err()
                .unwrap()
                .to_string()
                .contains("was not included")
        );
        profile.source = "#include \"invalid.h\"\n".into();
        let captured = capture(&profile, &input).unwrap();
        assert!(!captured.complete);
        assert!(captured.declarations.is_empty());
        assert!(captured.diagnostics.iter().any(|(severity, text)| {
            *severity >= CXDiagnostic_Error && text.contains("rejected profile")
        }));
    }
}
