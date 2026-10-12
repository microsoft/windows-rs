#![allow(non_upper_case_globals)]

use super::*;
use clang_sys::*;
use std::ffi::{CStr, CString};
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};
use std::sync::{RwLock, RwLockReadGuard};

// clang_createIndex registers LLVM targets, which must not overlap registry readers.
static TARGET_REGISTRY: RwLock<()> = RwLock::new(());

/// Reusable library ownership; each capture has a private index and translation unit.
pub struct Compiler {
    library: Arc<SharedLibrary>,
    layouts: Option<layout::Bridge>,
}

impl Compiler {
    pub fn new() -> Result<Self> {
        let library = load_manually().map_err(|error| Error::new(Stage::Capture, error))?;
        Ok(Self {
            library: Arc::new(library),
            layouts: None,
        })
    }

    pub fn with_layout_bridge(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let mut compiler = Self::new()?;
        compiler.layouts = Some(layout::Bridge::load(
            compiler.library.path(),
            path.as_ref(),
        )?);
        Ok(compiler)
    }

    pub fn capture(&self, input: &Input) -> Result<Snapshot> {
        self.capture_input(input).map_err(|mut error| {
            error.message = format!(
                "{} [{}]: {}",
                input.name,
                input.target.triple(),
                error.message
            );
            error
        })
    }

    fn capture_input(&self, input: &Input) -> Result<Snapshot> {
        if input.required.is_empty() {
            return Err(Error::new(
                Stage::Scope,
                "required declaration scope cannot be empty",
            ));
        }
        let session = Session::new(Arc::clone(&self.library), input)?;
        let mut capture = Capture {
            declarations: Vec::new(),
            cursors: Vec::new(),
            pending: BTreeSet::new(),
            layouts: self.layouts.as_ref(),
        };
        let mut roots = Vec::new();
        let mut selected: BTreeMap<String, Vec<CXCursor>> = BTreeMap::new();
        let mut pending = children(unsafe { clang_getTranslationUnitCursor(session.unit) });
        while let Some(cursor) = pending.pop() {
            let kind = unsafe { clang_getCursorKind(cursor) };
            if matches!(
                kind,
                CXCursor_Namespace | CXCursor_LinkageSpec | CXCursor_UnexposedDecl
            ) {
                pending.extend(children(cursor));
                continue;
            }
            if matches!(
                kind,
                CXCursor_StructDecl | CXCursor_ClassDecl | CXCursor_UnionDecl
            ) {
                pending.extend(children(cursor).into_iter().filter(|child| {
                    matches!(
                        unsafe { clang_getCursorKind(*child) },
                        CXCursor_StructDecl
                            | CXCursor_ClassDecl
                            | CXCursor_UnionDecl
                            | CXCursor_TypedefDecl
                            | CXCursor_TypeAliasDecl
                    )
                }));
            }
            if unsafe { clang_isDeclaration(kind) } == 0 && kind != CXCursor_MacroDefinition {
                continue;
            }
            let location = unsafe { clang_getCursorLocation(cursor) };
            let mut file = std::ptr::null_mut();
            unsafe {
                clang_getExpansionLocation(
                    location,
                    &raw mut file,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                );
            };
            if file.is_null()
                || !session
                    .roots
                    .iter()
                    .any(|root| unsafe { clang_File_isEqual(file, *root) } != 0)
            {
                continue;
            }
            let names = names(cursor)?;
            let qualified_name = names.join("::");
            let required = input.required.contains(&qualified_name);
            roots.push(Root {
                qualified_name: qualified_name.clone(),
                kind: text(unsafe { clang_getCursorKindSpelling(kind) }),
                required,
                location: locations(cursor).0,
            });
            if required {
                let values = selected.entry(qualified_name).or_default();
                if !values
                    .iter()
                    .any(|value| unsafe { clang_equalCursors(*value, cursor) } != 0)
                {
                    values.push(cursor);
                }
            }
        }
        for name in &input.required {
            let Some(values) = selected.get(name) else {
                return Err(Error::new(
                    Stage::Scope,
                    format!(
                        "{}: required declaration `{name}` is not in the compiler-identified root scope",
                        input.name
                    ),
                ));
            };
            for cursor in values {
                capture.declaration(*cursor)?;
            }
        }
        roots.sort_by(|a, b| {
            (&a.qualified_name, &a.kind, a.location.offset).cmp(&(
                &b.qualified_name,
                &b.kind,
                b.location.offset,
            ))
        });
        let snapshot = Snapshot {
            target: input.target,
            target_triple: session.target.clone(),
            compiler: text(unsafe { clang_getClangVersion() }),
            library: self.library.path().to_path_buf(),
            layout_bridge: self
                .layouts
                .as_ref()
                .map(|layouts| layouts.evidence.clone()),
            input_name: input.name.clone(),
            arguments: session.arguments.clone(),
            diagnostics: session.diagnostics.clone(),
            roots,
            declarations: capture.declarations,
        };
        snapshot.check()?;
        Ok(snapshot)
    }
}

struct Bindings {
    previous: Option<Arc<SharedLibrary>>,
}

impl Bindings {
    fn new(library: Arc<SharedLibrary>) -> Self {
        Self {
            previous: set_library(Some(library)),
        }
    }
}

impl Drop for Bindings {
    fn drop(&mut self) {
        set_library(self.previous.take());
    }
}

struct Session {
    index: CXIndex,
    unit: CXTranslationUnit,
    roots: Vec<CXFile>,
    arguments: Vec<String>,
    diagnostics: Vec<String>,
    target: String,
    _registry: Option<RwLockReadGuard<'static, ()>>,
    _bindings: Bindings,
}

impl Drop for Session {
    fn drop(&mut self) {
        unsafe {
            if !self.unit.is_null() {
                clang_disposeTranslationUnit(self.unit);
            }
            if !self.index.is_null() {
                clang_disposeIndex(self.index);
            }
        }
    }
}

impl Session {
    fn new(library: Arc<SharedLibrary>, input: &Input) -> Result<Self> {
        let mut result = Self {
            index: std::ptr::null_mut(),
            unit: std::ptr::null_mut(),
            roots: Vec::new(),
            arguments: vec![
                "-x".into(),
                "c++".into(),
                "-std=c++20".into(),
                format!("--target={}", input.target.triple()),
                "-fms-extensions".into(),
                "-fms-compatibility".into(),
            ],
            diagnostics: Vec::new(),
            target: String::new(),
            _registry: None,
            _bindings: Bindings::new(library),
        };
        result.arguments.extend(input.arguments.iter().cloned());
        let string = |value: &str| {
            CString::new(value)
                .map_err(|_| Error::new(Stage::Source, "compiler input contains NUL"))
        };
        let name = string(&input.name)?;
        let source = string(&input.source)?;
        let arguments = result
            .arguments
            .iter()
            .map(|arg| string(arg))
            .collect::<Result<Vec<_>>>()?;
        let pointers = arguments.iter().map(|arg| arg.as_ptr()).collect::<Vec<_>>();
        let mut unsaved = CXUnsavedFile {
            Filename: name.as_ptr(),
            Contents: source.as_ptr(),
            Length: input.source.len().try_into().map_err(|_| {
                Error::new(Stage::Source, "source length exceeds compiler input range")
            })?,
        };
        {
            let _registry = TARGET_REGISTRY.write().map_err(|_| {
                Error::new(Stage::Capture, "compiler target registry lock is poisoned")
            })?;
            result.index = unsafe { clang_createIndex(0, 0) };
        }
        if result.index.is_null() {
            return Err(Error::new(
                Stage::Capture,
                "compiler could not create an index",
            ));
        }
        result._registry = Some(TARGET_REGISTRY.read().map_err(|_| {
            Error::new(Stage::Capture, "compiler target registry lock is poisoned")
        })?);
        let status = unsafe {
            clang_parseTranslationUnit2(
                result.index,
                name.as_ptr(),
                pointers.as_ptr(),
                pointers
                    .len()
                    .try_into()
                    .map_err(|_| Error::new(Stage::Source, "too many compiler arguments"))?,
                &raw mut unsaved,
                1,
                CXTranslationUnit_DetailedPreprocessingRecord,
                &raw mut result.unit,
            )
        };
        if status != CXError_Success || result.unit.is_null() {
            return Err(Error::new(
                Stage::Source,
                format!("{}: parse failed with {status}", input.name),
            ));
        }
        let mut failed = false;
        for index in 0..unsafe { clang_getNumDiagnostics(result.unit) } {
            let diagnostic = unsafe { clang_getDiagnostic(result.unit, index) };
            failed |= unsafe { clang_getDiagnosticSeverity(diagnostic) } >= CXDiagnostic_Error;
            result.diagnostics.push(text(unsafe {
                clang_formatDiagnostic(diagnostic, clang_defaultDiagnosticDisplayOptions())
            }));
            unsafe { clang_disposeDiagnostic(diagnostic) };
        }
        if failed {
            return Err(Error::new(Stage::Source, result.diagnostics.join("\n")));
        }
        let info = unsafe { clang_getTranslationUnitTargetInfo(result.unit) };
        if info.is_null() {
            return Err(Error::new(
                Stage::Capture,
                "compiler did not report target information",
            ));
        }
        let triple = text(unsafe { clang_TargetInfo_getTriple(info) });
        let width = unsafe { clang_TargetInfo_getPointerWidth(info) };
        unsafe { clang_TargetInfo_dispose(info) };
        let matches = triple
            .strip_prefix(input.target.triple())
            .is_some_and(|version| {
                version.is_empty()
                    || version.split('.').all(|part| {
                        !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit())
                    })
            });
        if !matches || width != input.target.pointer_bits() as i32 {
            return Err(Error::new(
                Stage::Capture,
                format!("unexpected compiler target {triple}/{width}"),
            ));
        }
        result.target = triple;
        if input.roots.is_empty() {
            result
                .roots
                .push(unsafe { clang_getFile(result.unit, name.as_ptr()) });
        } else {
            for path in &input.roots {
                let path = string(
                    path.to_str()
                        .ok_or_else(|| Error::new(Stage::Scope, "non-UTF-8 root path"))?,
                )?;
                result
                    .roots
                    .push(unsafe { clang_getFile(result.unit, path.as_ptr()) });
            }
        }
        if result.roots.iter().any(|root| root.is_null()) {
            return Err(Error::new(
                Stage::Scope,
                "requested root file was not observed by the compiler",
            ));
        }
        Ok(result)
    }
}

struct Capture<'a> {
    declarations: Vec<Declaration>,
    cursors: Vec<CXCursor>,
    pending: BTreeSet<Id>,
    layouts: Option<&'a layout::Bridge>,
}

impl Capture<'_> {
    fn declaration(&mut self, cursor: CXCursor) -> Result<Id> {
        let observed = cursor;
        let mut parent = unsafe { clang_getCursorSemanticParent(cursor) };
        while unsafe { clang_getCursorKind(parent) } == CXCursor_LinkageSpec {
            parent = unsafe { clang_getCursorSemanticParent(parent) };
        }
        if !matches!(
            unsafe { clang_getCursorKind(parent) },
            CXCursor_TranslationUnit
                | CXCursor_Namespace
                | CXCursor_StructDecl
                | CXCursor_ClassDecl
                | CXCursor_UnionDecl
        ) {
            return Err(error(cursor, "unsupported native declaration owner"));
        }
        let nested = matches!(
            unsafe { clang_getCursorKind(parent) },
            CXCursor_StructDecl | CXCursor_ClassDecl | CXCursor_UnionDecl
        );
        if nested && unsafe { clang_getCXXAccessSpecifier(cursor) } != CX_CXXPublic {
            return Err(error(
                cursor,
                "nonpublic nested declarations are outside C3-C",
            ));
        }
        self.check_attributes(cursor)?;
        let cursor = unsafe { clang_getCanonicalCursor(cursor) };
        if let Some(id) = self
            .cursors
            .iter()
            .position(|value| unsafe { clang_equalCursors(*value, cursor) } != 0)
        {
            if unsafe { clang_getCursorKind(observed) } == CXCursor_FunctionDecl {
                let function = self.function(observed)?;
                let DeclarationKind::Function(observations) = &mut self.declarations[id].kind
                else {
                    return Err(error(observed, "function identity targets a non-function"));
                };
                let first = &observations[0];
                if first.evidence.linker_name != function.evidence.linker_name
                    || first.evidence.convention != function.evidence.convention
                    || first.evidence.variadic != function.evidence.variadic
                    || first.evidence.dll_import != function.evidence.dll_import
                    || first.result != function.result
                    || first.canonical_result != function.canonical_result
                    || first.params.len() != function.params.len()
                    || first
                        .params
                        .iter()
                        .zip(&function.params)
                        .any(|(a, b)| a.written != b.written || a.canonical != b.canonical)
                {
                    return Err(error(observed, "native function redeclarations disagree"));
                }
                observations.push(function);
            }
            return Ok(id);
        }
        if self.pending.len() >= 64 {
            return Err(error(cursor, "native dependency nesting exceeds 64"));
        }
        let kind = unsafe { clang_getCursorKind(cursor) };
        if !matches!(
            kind,
            CXCursor_TypedefDecl
                | CXCursor_TypeAliasDecl
                | CXCursor_StructDecl
                | CXCursor_ClassDecl
                | CXCursor_UnionDecl
                | CXCursor_EnumDecl
                | CXCursor_FunctionDecl
        ) {
            return Err(error(
                cursor,
                "declaration family is outside C2 (C3-C9 required)",
            ));
        }
        self.check_attributes(cursor)?;
        let anonymous = unsafe { clang_Cursor_isAnonymous(cursor) } != 0;
        let mut names = names(cursor)?;
        if anonymous {
            *names.last_mut().unwrap() = String::new();
        }
        if anonymous && !nested && kind != CXCursor_EnumDecl {
            return Err(error(
                cursor,
                "unnamed records require a direct enclosing record field (C3-C)",
            ));
        }
        let usr = text(unsafe { clang_getCursorUSR(cursor) });
        if usr.is_empty() {
            return Err(error(
                cursor,
                "compiler did not supply declaration identity",
            ));
        }
        let (expansion, spelling) = locations(cursor);
        let id = self.declarations.len();
        self.cursors.push(cursor);
        self.pending.insert(id);
        self.declarations.push(Declaration {
            evidence: Evidence {
                id,
                qualified_name: names.join("::"),
                kind: text(unsafe { clang_getCursorKindSpelling(kind) }),
                usr,
                expansion,
                spelling,
                written_type: String::new(),
                canonical_type: String::new(),
                layout: None,
            },
            names,
            owner: None,
            anonymous,
            kind: DeclarationKind::Opaque { union: false },
        });
        if nested {
            let owner = self.declaration(parent)?;
            self.declarations[id].owner = Some(owner);
        }
        let declaration = if kind == CXCursor_FunctionDecl {
            self.declarations[id].evidence.written_type =
                text(unsafe { clang_getTypeSpelling(clang_getCursorType(observed)) });
            self.declarations[id].evidence.canonical_type = text(unsafe {
                clang_getTypeSpelling(clang_getCanonicalType(clang_getCursorType(observed)))
            });
            DeclarationKind::Function(vec![self.function(observed)?])
        } else if kind == CXCursor_EnumDecl {
            let definition = unsafe { clang_getCursorDefinition(cursor) };
            if unsafe { clang_Cursor_isNull(definition) } != 0 {
                return Err(error(
                    cursor,
                    "forward enums require a separate completeness contract",
                ));
            }
            let ty = unsafe { clang_getCursorType(definition) };
            let underlying = self.ty(unsafe { clang_getEnumDeclIntegerType(definition) }, 0)?;
            let mut members = Vec::new();
            for member in children(definition) {
                if unsafe { clang_getCursorKind(member) } != CXCursor_EnumConstantDecl {
                    return Err(error(
                        member,
                        "enum attributes or members require a captured contract",
                    ));
                }
                self.check_attributes(member)?;
                let (expansion, spelling) = locations(member);
                members.push(EnumMemberEvidence {
                    name: text(unsafe { clang_getCursorSpelling(member) }),
                    value: enum_value(member, &underlying)?,
                    expansion,
                    spelling,
                });
            }
            self.declarations[id].evidence.written_type =
                text(unsafe { clang_getTypeSpelling(ty) });
            self.declarations[id].evidence.canonical_type =
                text(unsafe { clang_getTypeSpelling(clang_getCanonicalType(ty)) });
            self.declarations[id].evidence.layout = layout(ty)?;
            let (expansion, spelling) = locations(definition);
            self.declarations[id].evidence.expansion = expansion;
            self.declarations[id].evidence.spelling = spelling;
            DeclarationKind::Enum {
                underlying,
                scoped: unsafe { clang_EnumDecl_isScoped(definition) } != 0,
                members,
            }
        } else if matches!(kind, CXCursor_TypedefDecl | CXCursor_TypeAliasDecl) {
            let ty = unsafe { clang_getTypedefDeclUnderlyingType(cursor) };
            let canonical = unsafe { clang_getCanonicalType(ty) };
            self.declarations[id].evidence.written_type =
                text(unsafe { clang_getTypeSpelling(ty) });
            self.declarations[id].evidence.canonical_type =
                text(unsafe { clang_getTypeSpelling(canonical) });
            self.declarations[id].evidence.layout = layout(ty)?;
            DeclarationKind::Alias {
                written: self.ty(ty, 0)?,
                canonical: self.ty(canonical, 0)?,
            }
        } else {
            let ty = unsafe { clang_getCursorType(cursor) };
            self.declarations[id].evidence.written_type =
                text(unsafe { clang_getTypeSpelling(ty) });
            self.declarations[id].evidence.canonical_type =
                text(unsafe { clang_getTypeSpelling(clang_getCanonicalType(ty)) });
            self.declarations[id].evidence.layout = layout(ty)?;
            let definition = unsafe { clang_getCursorDefinition(cursor) };
            if unsafe { clang_Cursor_isNull(definition) } != 0 {
                DeclarationKind::Opaque {
                    union: kind == CXCursor_UnionDecl,
                }
            } else {
                let contract = self
                    .layouts
                    .map(|layouts| layouts.record(definition))
                    .transpose()
                    .map_err(|mut failure| {
                        failure.message = error(definition, &failure.message).message;
                        failure
                    })?;
                let fields = self.record(definition)?;
                let (expansion, spelling) = locations(definition);
                self.declarations[id].evidence.expansion = expansion;
                self.declarations[id].evidence.spelling = spelling;
                DeclarationKind::Record {
                    union: kind == CXCursor_UnionDecl,
                    fields,
                    contract,
                }
            }
        };
        self.declarations[id].kind = declaration;
        self.pending.remove(&id);
        Ok(id)
    }

    fn check_attributes(&self, cursor: CXCursor) -> Result<()> {
        let aligned_record = self.layouts.is_some()
            && matches!(
                unsafe { clang_getCursorKind(cursor) },
                CXCursor_StructDecl | CXCursor_ClassDecl | CXCursor_UnionDecl
            );
        for child in children(cursor) {
            let kind = unsafe { clang_getCursorKind(child) };
            if unsafe { clang_isAttribute(kind) } != 0
                && !(aligned_record && kind == CXCursor_AlignedAttr)
                && !(unsafe { clang_getCursorKind(cursor) } == CXCursor_FunctionDecl
                    && kind == CXCursor_DLLImport)
            {
                return Err(error(
                    cursor,
                    "declaration attributes require a captured contract (C7)",
                ));
            }
        }
        Ok(())
    }

    fn function(&mut self, cursor: CXCursor) -> Result<NativeFunction> {
        if unsafe { clang_getCursorLinkage(cursor) } != CXLinkage_External
            || unsafe { clang_Cursor_isFunctionInlined(cursor) } != 0
        {
            return Err(error(
                cursor,
                "inline or nonexternal functions are not native imports",
            ));
        }
        let ty = unsafe { clang_getCursorType(cursor) };
        if ty.kind != CXType_FunctionProto {
            return Err(error(cursor, "function has no native prototype"));
        }
        if unsafe { clang_getExceptionSpecificationType(ty) }
            != CXCursor_ExceptionSpecificationKind_None
        {
            return Err(error(
                cursor,
                "function exception specification requires a captured contract",
            ));
        }
        let convention = match unsafe { clang_getFunctionTypeCallingConv(ty) } {
            CXCallingConv_C => CallingConvention::C,
            CXCallingConv_X86StdCall => CallingConvention::Stdcall,
            CXCallingConv_X86FastCall => CallingConvention::Fastcall,
            CXCallingConv_Win64 => CallingConvention::Win64,
            _ => {
                return Err(error(
                    cursor,
                    "function calling convention is not representable",
                ));
            }
        };
        let linker_name = text(unsafe { clang_Cursor_getMangling(cursor) });
        if linker_name.is_empty() {
            return Err(error(
                cursor,
                "compiler did not supply a native linker symbol",
            ));
        }
        let result = unsafe { clang_getResultType(ty) };
        let canonical_result = unsafe { clang_getCanonicalType(result) };
        let count = unsafe { clang_Cursor_getNumArguments(cursor) };
        if count < 0 {
            return Err(error(cursor, "compiler did not supply function parameters"));
        }
        let mut params = Vec::new();
        for ordinal in 0..count as u32 {
            let param = unsafe { clang_Cursor_getArgument(cursor, ordinal) };
            self.check_attributes(param)?;
            if unsafe { clang_Cursor_isNull(clang_Cursor_getVarDeclInitializer(param)) } == 0 {
                return Err(error(
                    param,
                    "default arguments require a captured contract",
                ));
            }
            let ty = unsafe { clang_getCursorType(param) };
            let canonical = unsafe { clang_getCanonicalType(ty) };
            let (expansion, spelling) = locations(param);
            params.push(NativeParameter {
                evidence: ParameterEvidence {
                    name: text(unsafe { clang_getCursorSpelling(param) }),
                    ordinal: ordinal as usize,
                    expansion,
                    spelling,
                    written_type: text(unsafe { clang_getTypeSpelling(ty) }),
                    canonical_type: text(unsafe { clang_getTypeSpelling(canonical) }),
                },
                written: self.ty(ty, 0)?,
                canonical: self.ty(canonical, 0)?,
            });
        }
        Ok(NativeFunction {
            evidence: FunctionEvidence {
                linker_name,
                convention,
                variadic: unsafe { clang_Cursor_isVariadic(cursor) } != 0,
                dll_import: children(cursor)
                    .iter()
                    .any(|child| unsafe { clang_getCursorKind(*child) } == CXCursor_DLLImport),
                written_result: text(unsafe { clang_getTypeSpelling(result) }),
                canonical_result: text(unsafe { clang_getTypeSpelling(canonical_result) }),
            },
            result: self.ty(result, 0)?,
            canonical_result: self.ty(canonical_result, 0)?,
            params,
        })
    }

    fn record(&mut self, cursor: CXCursor) -> Result<Vec<NativeField>> {
        if unsafe { clang_isPODType(clang_getCursorType(cursor)) } == 0 {
            return Err(error(cursor, "non-POD record behavior is outside C3-A"));
        }
        let members = children(cursor);
        for child in &members {
            let child = *child;
            match unsafe { clang_getCursorKind(child) } {
                CXCursor_AlignedAttr if self.layouts.is_some() => {}
                CXCursor_CXXAccessSpecifier => {
                    if unsafe { clang_getCXXAccessSpecifier(child) } != CX_CXXPublic {
                        return Err(error(child, "nonpublic record members are outside C3-A"));
                    }
                }
                CXCursor_FieldDecl
                | CXCursor_StructDecl
                | CXCursor_ClassDecl
                | CXCursor_UnionDecl
                | CXCursor_TypedefDecl
                | CXCursor_TypeAliasDecl => {}
                _ => {
                    return Err(error(
                        child,
                        "record member behavior or attributes are outside C3 storage",
                    ));
                }
            }
        }
        let mut fields = Vec::new();
        for child in record_fields(cursor) {
            if unsafe { clang_Cursor_isBitField(child) } != 0 {
                return Err(error(child, "bit-field storage is outside C3-A"));
            }
            if unsafe { clang_getCXXAccessSpecifier(child) } != CX_CXXPublic {
                return Err(error(child, "nonpublic record members are outside C3-A"));
            }
            if children(child)
                .iter()
                .any(|attr| unsafe { clang_isAttribute(clang_getCursorKind(*attr)) } != 0)
            {
                return Err(error(child, "field attributes require C7 contracts"));
            }
            let anonymous = if let Some(layouts) = self.layouts {
                layouts.anonymous_field(child).map_err(|mut failure| {
                    failure.message = error(child, &failure.message).message;
                    failure
                })?
            } else if members
                .iter()
                .any(|member| unsafe { clang_equalCursors(*member, child) } != 0)
            {
                false
            } else {
                return Err(error(
                    child,
                    "implicit native fields require the matched identity bridge",
                ));
            };
            let name = if anonymous {
                String::new()
            } else {
                text(unsafe { clang_getCursorSpelling(child) })
            };
            let bits = unsafe { clang_Cursor_getOffsetOfField(child) };
            if bits < 0 || bits % 8 != 0 {
                return Err(error(child, "native field has no byte-aligned offset"));
            }
            let ty = unsafe { clang_getCursorType(child) };
            let canonical = unsafe { clang_getCanonicalType(ty) };
            if anonymous
                && unsafe {
                    clang_Cursor_isAnonymousRecordDecl(clang_getTypeDeclaration(canonical))
                } == 0
            {
                return Err(error(
                    child,
                    "unnamed field is not an anonymous record member",
                ));
            }
            let (expansion, spelling) = locations(child);
            let written = self.ty(ty, 0)?;
            let canonical_type = self.ty(canonical, 0)?;
            let record_id = match canonical_type.kind {
                NativeKind::Record(id) => Some(id),
                _ => None,
            };
            fields.push(NativeField {
                evidence: FieldEvidence {
                    name,
                    ordinal: fields.len(),
                    anonymous,
                    record_id,
                    expansion,
                    spelling,
                    written_type: text(unsafe { clang_getTypeSpelling(ty) }),
                    canonical_type: text(unsafe { clang_getTypeSpelling(canonical) }),
                    layout: layout(ty)?,
                    offset: bits as u64 / 8,
                },
                written,
                canonical: canonical_type,
            });
        }
        if fields.is_empty() {
            return Err(error(cursor, "empty record storage is outside C3-A"));
        }
        Ok(fields)
    }

    fn ty(&mut self, ty: CXType, depth: usize) -> Result<NativeType> {
        if depth > 64 {
            return Err(Error::new(Stage::Capture, "native type nesting exceeds 64"));
        }
        let qualifiers = Qualifiers {
            constant: unsafe { clang_isConstQualifiedType(ty) } != 0,
            volatile: unsafe { clang_isVolatileQualifiedType(ty) } != 0,
            restrict: unsafe { clang_isRestrictQualifiedType(ty) } != 0,
        };
        let kind = match ty.kind {
            CXType_Void => NativeKind::Void,
            CXType_Bool | CXType_Char_U | CXType_Char_S | CXType_UChar | CXType_SChar
            | CXType_UShort | CXType_Short | CXType_UInt | CXType_Int | CXType_ULong
            | CXType_Long | CXType_ULongLong | CXType_LongLong | CXType_Float | CXType_Double
            | CXType_LongDouble | CXType_WChar | CXType_Char16 | CXType_Char32 => {
                NativeKind::Scalar(ty.kind)
            }
            CXType_Typedef => {
                NativeKind::Alias(self.declaration(unsafe { clang_getTypeDeclaration(ty) })?)
            }
            CXType_Elaborated => {
                let mut named = self.ty(unsafe { clang_Type_getNamedType(ty) }, depth + 1)?;
                named.qualify(&qualifiers);
                return Ok(named);
            }
            CXType_Record => {
                NativeKind::Record(self.declaration(unsafe { clang_getTypeDeclaration(ty) })?)
            }
            CXType_Enum => {
                NativeKind::Enum(self.declaration(unsafe { clang_getTypeDeclaration(ty) })?)
            }
            CXType_Pointer => NativeKind::Pointer(Box::new(
                self.ty(unsafe { clang_getPointeeType(ty) }, depth + 1)?,
            )),
            CXType_LValueReference => NativeKind::Reference(Box::new(
                self.ty(unsafe { clang_getPointeeType(ty) }, depth + 1)?,
            )),
            CXType_ConstantArray => {
                let count = unsafe { clang_getArraySize(ty) };
                let count = usize::try_from(count)
                    .map_err(|_| Error::new(Stage::Capture, "invalid native array bound"))?;
                NativeKind::Array(
                    Box::new(self.ty(unsafe { clang_getArrayElementType(ty) }, depth + 1)?),
                    count,
                )
            }
            _ => {
                return Err(Error::new(
                    Stage::Capture,
                    format!(
                        "unsupported C2 native type {} ({})",
                        text(unsafe { clang_getTypeSpelling(ty) }),
                        ty.kind
                    ),
                ));
            }
        };
        Ok(NativeType {
            kind,
            qualifiers,
            layout: layout(ty)?,
        })
    }
}

fn enum_value(cursor: CXCursor, underlying: &NativeType) -> Result<metadata::Value> {
    let NativeKind::Scalar(kind) = underlying.kind else {
        return Err(error(cursor, "enum has a nonscalar underlying type"));
    };
    let signed = unsafe { clang_getEnumConstantDeclValue(cursor) };
    let unsigned = unsafe { clang_getEnumConstantDeclUnsignedValue(cursor) };
    let failure = |_| error(cursor, "enum value exceeds its declared underlying type");
    Ok(match kind {
        CXType_Char_S | CXType_SChar => metadata::Value::I8(i8::try_from(signed).map_err(failure)?),
        CXType_Char_U | CXType_UChar => {
            metadata::Value::U8(u8::try_from(unsigned).map_err(failure)?)
        }
        CXType_Short => metadata::Value::I16(i16::try_from(signed).map_err(failure)?),
        CXType_UShort => metadata::Value::U16(u16::try_from(unsigned).map_err(failure)?),
        CXType_Int | CXType_Long => metadata::Value::I32(i32::try_from(signed).map_err(failure)?),
        CXType_UInt | CXType_ULong => {
            metadata::Value::U32(u32::try_from(unsigned).map_err(failure)?)
        }
        CXType_LongLong => metadata::Value::I64(signed),
        CXType_ULongLong => metadata::Value::U64(unsigned),
        _ => return Err(error(cursor, "enum underlying type is not representable")),
    })
}

fn layout(ty: CXType) -> Result<Option<Layout>> {
    let size = unsafe { clang_Type_getSizeOf(ty) };
    let alignment = unsafe { clang_Type_getAlignOf(ty) };
    if size < 0 || alignment < 0 {
        if ty.kind == CXType_Void
            || size == i64::from(CXTypeLayoutError_Incomplete)
            || alignment == i64::from(CXTypeLayoutError_Incomplete)
        {
            return Ok(None);
        }
        return Err(Error::new(
            Stage::Capture,
            format!("unavailable native layout {size}/{alignment}"),
        ));
    }
    Ok(Some(Layout {
        size: size as u64,
        alignment: alignment as u64,
    }))
}

fn names(cursor: CXCursor) -> Result<Vec<String>> {
    let mut names = vec![text(unsafe { clang_getCursorSpelling(cursor) })];
    if unsafe { clang_getCursorKind(cursor) } == CXCursor_MacroDefinition {
        return Ok(names);
    }
    let mut parent = unsafe { clang_getCursorSemanticParent(cursor) };
    for _ in 0..64 {
        let kind = unsafe { clang_getCursorKind(parent) };
        if kind == CXCursor_TranslationUnit
            || unsafe { clang_isInvalid(kind) } != 0
            || unsafe { clang_Cursor_isNull(parent) } != 0
        {
            names.reverse();
            return Ok(names);
        }
        if kind != CXCursor_LinkageSpec {
            names.push(text(unsafe { clang_getCursorSpelling(parent) }));
        }
        parent = unsafe { clang_getCursorSemanticParent(parent) };
    }
    Err(error(
        cursor,
        "compiler semantic ownership exceeds 64 levels",
    ))
}

fn locations(cursor: CXCursor) -> (Location, Location) {
    let location = unsafe { clang_getCursorLocation(cursor) };
    let read = |expansion| {
        let mut file = std::ptr::null_mut();
        let (mut line, mut column, mut offset) = (0, 0, 0);
        unsafe {
            if expansion {
                clang_getExpansionLocation(
                    location,
                    &raw mut file,
                    &raw mut line,
                    &raw mut column,
                    &raw mut offset,
                );
            } else {
                clang_getSpellingLocation(
                    location,
                    &raw mut file,
                    &raw mut line,
                    &raw mut column,
                    &raw mut offset,
                );
            }
        }
        Location {
            file: if file.is_null() {
                String::new()
            } else {
                text(unsafe { clang_getFileName(file) })
            },
            line,
            column,
            offset,
        }
    };
    (read(true), read(false))
}

fn error(cursor: CXCursor, message: &str) -> Error {
    let location = locations(cursor).0;
    Error::new(
        Stage::Capture,
        format!(
            "{}:{}:{}: {}: {message}",
            location.file,
            location.line,
            location.column,
            text(unsafe { clang_getCursorSpelling(cursor) })
        ),
    )
}

fn text(value: CXString) -> String {
    let pointer = unsafe { clang_getCString(value) };
    let result = if pointer.is_null() {
        String::new()
    } else {
        unsafe { CStr::from_ptr(pointer) }
            .to_string_lossy()
            .into_owned()
    };
    unsafe { clang_disposeString(value) };
    result
}

fn children(cursor: CXCursor) -> Vec<CXCursor> {
    struct State {
        cursors: Vec<CXCursor>,
        panic: Option<Box<dyn std::any::Any + Send>>,
    }

    extern "C" fn visit(cursor: CXCursor, _: CXCursor, data: CXClientData) -> CXChildVisitResult {
        let state = unsafe { &mut *data.cast::<State>() };
        match catch_unwind(AssertUnwindSafe(|| state.cursors.push(cursor))) {
            Ok(()) => CXChildVisit_Continue,
            Err(panic) => {
                state.panic = Some(panic);
                CXChildVisit_Break
            }
        }
    }
    let mut state = State {
        cursors: Vec::new(),
        panic: None,
    };
    unsafe { clang_visitChildren(cursor, visit, (&raw mut state).cast()) };
    if let Some(panic) = state.panic {
        resume_unwind(panic);
    }
    state.cursors
}

fn record_fields(cursor: CXCursor) -> Vec<CXCursor> {
    struct State {
        cursors: Vec<CXCursor>,
        panic: Option<Box<dyn std::any::Any + Send>>,
    }
    extern "C" fn visit(cursor: CXCursor, data: CXClientData) -> CXVisitorResult {
        let state = unsafe { &mut *data.cast::<State>() };
        match catch_unwind(AssertUnwindSafe(|| state.cursors.push(cursor))) {
            Ok(()) => CXVisit_Continue,
            Err(panic) => {
                state.panic = Some(panic);
                CXVisit_Break
            }
        }
    }
    let mut state = State {
        cursors: Vec::new(),
        panic: None,
    };
    unsafe {
        clang_Type_visitFields(clang_getCursorType(cursor), visit, (&raw mut state).cast());
    }
    if let Some(panic) = state.panic {
        resume_unwind(panic);
    }
    state.cursors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiler_field_list_exposes_implicit_storage_and_nominal_owners() {
        let compiler = Compiler::new().unwrap();
        for target in [Target::X86, Target::X64, Target::Arm64] {
            let source = include_str!("..\\..\\..\\tests\\libs\\clang2\\input\\nested_records.h");
            let session = Session::new(
                Arc::clone(&compiler.library),
                &Input::new("nested.cpp", source, target),
            )
            .unwrap();
            let namespace = children(unsafe { clang_getTranslationUnitCursor(session.unit) })
                .into_iter()
                .find(|cursor| text(unsafe { clang_getCursorSpelling(*cursor) }) == "Shapes")
                .unwrap();
            let owner = children(namespace)
                .into_iter()
                .find(|cursor| text(unsafe { clang_getCursorSpelling(*cursor) }) == "Owner")
                .unwrap();
            let fields = record_fields(owner);
            assert_eq!(fields.len(), 9);
            assert_eq!(
                text(unsafe { clang_getCursorSpelling(fields[5]) }),
                "detail"
            );
            for slot in [6, 7] {
                assert!(!text(unsafe { clang_getCursorSpelling(fields[slot]) }).is_empty());
                assert!(
                    !children(owner)
                        .iter()
                        .any(|child| unsafe { clang_equalCursors(*child, fields[slot]) } != 0)
                );
            }
            for slot in [5, 6, 7] {
                let field = fields[slot];
                assert_eq!(unsafe { clang_getCursorKind(field) }, CXCursor_FieldDecl);
                assert_ne!(
                    unsafe { clang_equalCursors(clang_getCursorSemanticParent(field), owner) },
                    0
                );
                let record = unsafe { clang_getTypeDeclaration(clang_getCursorType(field)) };
                assert_ne!(unsafe { clang_Cursor_isAnonymous(record) }, 0);
                assert_ne!(
                    unsafe { clang_equalCursors(clang_getCursorSemanticParent(record), owner) },
                    0
                );
                assert_eq!(
                    unsafe { clang_Cursor_isAnonymousRecordDecl(record) } != 0,
                    slot != 5
                );
                assert!(unsafe { clang_Cursor_getOffsetOfField(field) } >= 0);
                assert!(!text(unsafe { clang_getCursorUSR(record) }).is_empty());
            }
        }
    }

    #[test]
    fn layout_queries_do_not_supply_explicit_layout_contracts() {
        let compiler = Compiler::new().unwrap();
        let source = include_str!("..\\..\\..\\tests\\libs\\clang2\\input\\layout_contracts.h");
        for target in [Target::X86, Target::X64, Target::Arm64] {
            let input = Input::new("layouts.cpp", source, target);
            let session = Session::new(Arc::clone(&compiler.library), &input).unwrap();
            let records = children(unsafe { clang_getTranslationUnitCursor(session.unit) })
                .into_iter()
                .filter(|cursor| unsafe { clang_getCursorKind(*cursor) } == CXCursor_StructDecl)
                .map(|cursor| (text(unsafe { clang_getCursorSpelling(cursor) }), cursor))
                .collect::<BTreeMap<_, _>>();
            let storage = |name: &str| {
                layout(unsafe { clang_getCursorType(records[name]) })
                    .unwrap()
                    .unwrap()
            };
            assert_eq!(
                storage("PackOne"),
                Layout {
                    size: 5,
                    alignment: 1
                }
            );
            assert_eq!(
                storage("PackTwo"),
                Layout {
                    size: 4,
                    alignment: 2
                }
            );
            assert_eq!(storage("PackTwo"), storage("PackFour"));
            for name in ["PackOne", "PackTwo", "PackFour"] {
                assert!(
                    children(records[name])
                        .iter()
                        .all(|child| unsafe { clang_getCursorKind(*child) == CXCursor_FieldDecl })
                );
            }
            assert_eq!(
                storage("Aligned"),
                Layout {
                    size: 16,
                    alignment: 16
                }
            );
            assert_eq!(
                storage("AlignOne"),
                Layout {
                    size: 4,
                    alignment: 4
                }
            );
            assert_eq!(storage("AlignOne"), storage("AlignFour"));
            for name in ["Aligned", "AlignOne", "AlignFour"] {
                let attributes = children(records[name])
                    .into_iter()
                    .filter(|child| unsafe { clang_getCursorKind(*child) } == CXCursor_AlignedAttr)
                    .collect::<Vec<_>>();
                assert_eq!(attributes.len(), 1);
                assert!(children(attributes[0]).is_empty());
                let value = unsafe { clang_Cursor_Evaluate(attributes[0]) };
                if !value.is_null() {
                    unsafe { clang_EvalResult_dispose(value) };
                }
                assert!(value.is_null());
            }
            assert_eq!(storage("PackOne"), storage("PackedAttr"));
            assert!(
                children(records["PackedAttr"])
                    .iter()
                    .any(|child| unsafe { clang_getCursorKind(*child) == CXCursor_PackedAttr })
            );
            assert_eq!(
                storage("FieldAligned"),
                Layout {
                    size: 32,
                    alignment: 16
                }
            );
            let field = children(records["FieldAligned"])
                .into_iter()
                .find(|child| text(unsafe { clang_getCursorSpelling(*child) }) == "value")
                .unwrap();
            assert_eq!(
                layout(unsafe { clang_getCursorType(field) }).unwrap(),
                Some(Layout {
                    size: 4,
                    alignment: 4
                })
            );
            assert_eq!(unsafe { clang_Cursor_getOffsetOfField(field) }, 128);
        }
    }
}
