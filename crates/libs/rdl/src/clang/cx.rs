use super::*;
pub use clang_sys::*;
use std::ffi::{CStr, CString};

pub struct Library;

impl Library {
    pub fn new() -> Result<Self, Error> {
        clang_sys::load()
            .map_err(|e| Error::new(&format!("failed to load libclang: {e}"), "", 0, 0))?;
        Ok(Self)
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        _ = clang_sys::unload();
    }
}

pub struct Index(CXIndex);

impl Index {
    pub fn new() -> Result<Self, Error> {
        let index = unsafe { clang_createIndex(0, 0) };

        if index.is_null() {
            return Err(Error::new("failed to create libclang index", "", 0, 0));
        }

        Ok(Self(index))
    }

    pub fn parse(&self, input: &str, args: &[&str]) -> Result<TranslationUnit, Error> {
        let cinput = CString::new(input).map_err(|_| Error::new("invalid input", input, 0, 0))?;
        let mut cargs = vec![];

        for arg in args {
            cargs.push(
                CString::new(*arg)
                    .map_err(|_| Error::new(&format!("invalid argument: {arg}"), "", 0, 0))?,
            );
        }

        let cargs: Vec<_> = cargs.iter().map(|arg| arg.as_ptr()).collect();

        let tu = unsafe {
            clang_parseTranslationUnit(
                self.0,
                cinput.as_ptr(),
                cargs.as_ptr(),
                cargs.len().try_into().unwrap(),
                std::ptr::null_mut(),
                0,
                CXTranslationUnit_DetailedPreprocessingRecord,
            )
        };

        if tu.is_null() {
            return Err(Error::new("failed to parse", input, 0, 0));
        }

        Ok(TranslationUnit(tu))
    }

    /// Parse a synthetic in-memory source file without requiring it to exist on
    /// disk.
    ///
    /// The file's content is provided via the `content` string, which is
    /// mapped to `filename` using a `CXUnsavedFile`.  This is the standard
    /// libclang mechanism for injecting synthetic source; the file name is used
    /// only as the virtual path from which relative `#include` directives
    /// resolve.
    ///
    /// Parsing uses `CXTranslationUnit_KeepGoing` so that isolated errors
    /// (e.g. from macros that are not valid integer constant expressions) do
    /// not abort the entire translation unit.
    pub fn parse_unsaved(
        &self,
        filename: &str,
        content: &str,
        args: &[&str],
    ) -> Result<TranslationUnit, Error> {
        let c_filename =
            CString::new(filename).map_err(|_| Error::new("invalid filename", filename, 0, 0))?;
        let c_content =
            CString::new(content).map_err(|_| Error::new("invalid content", filename, 0, 0))?;

        let mut cargs = vec![];
        for arg in args {
            cargs.push(
                CString::new(*arg)
                    .map_err(|_| Error::new(&format!("invalid argument: {arg}"), "", 0, 0))?,
            );
        }
        let cargs: Vec<_> = cargs.iter().map(|a| a.as_ptr()).collect();

        let mut unsaved = CXUnsavedFile {
            Filename: c_filename.as_ptr(),
            Contents: c_content.as_ptr(),
            Length: content.len() as _,
        };

        let tu = unsafe {
            clang_parseTranslationUnit(
                self.0,
                c_filename.as_ptr(),
                cargs.as_ptr(),
                cargs.len().try_into().unwrap(),
                &mut unsaved,
                1,
                CXTranslationUnit_KeepGoing,
            )
        };

        if tu.is_null() {
            return Err(Error::new("failed to parse", filename, 0, 0));
        }

        Ok(TranslationUnit(tu))
    }
}

impl Drop for Index {
    fn drop(&mut self) {
        unsafe { clang_disposeIndex(self.0) }
    }
}

pub struct TranslationUnit(CXTranslationUnit);

impl TranslationUnit {
    pub fn diagnostics(&self) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];

        unsafe {
            let num_diags = clang_getNumDiagnostics(self.0);

            for i in 0..num_diags {
                let diag = clang_getDiagnostic(self.0, i);

                if diag.is_null() {
                    continue;
                }

                let severity = clang_getDiagnosticSeverity(diag);
                let message = to_string(clang_getDiagnosticSpelling(diag));

                let loc = clang_getDiagnosticLocation(diag);
                let mut file = std::ptr::null_mut();
                let mut line: u32 = 0;
                let mut column: u32 = 0;
                let mut offset: u32 = 0;

                clang_getSpellingLocation(loc, &mut file, &mut line, &mut column, &mut offset);

                let file_name = if file.is_null() {
                    String::new()
                } else {
                    let fname = clang_getFileName(file);
                    to_string(fname)
                };

                diagnostics.push(Diagnostic {
                    severity,
                    message,
                    file_name,
                    line,
                    column,
                });

                clang_disposeDiagnostic(diag);
            }
        }

        diagnostics
    }

    pub fn cursor(&self) -> Cursor {
        Cursor(unsafe { clang_getTranslationUnitCursor(self.0) })
    }

    /// Tokenize the given source range and return the spellings of all tokens.
    pub fn tokenize(&self, range: CXSourceRange) -> Vec<(CXTokenKind, String)> {
        unsafe {
            let mut tokens: *mut CXToken = std::ptr::null_mut();
            let mut n_tokens: u32 = 0;
            clang_tokenize(self.0, range, &mut tokens, &mut n_tokens);

            if n_tokens == 0 {
                return vec![];
            }

            let result = (0..n_tokens as usize)
                .map(|i| {
                    let token = *tokens.add(i);
                    let kind = clang_getTokenKind(token);
                    let spelling = to_string(clang_getTokenSpelling(self.0, token));
                    (kind, spelling)
                })
                .collect();

            clang_disposeTokens(self.0, tokens, n_tokens);
            result
        }
    }
}

impl Drop for TranslationUnit {
    fn drop(&mut self) {
        unsafe { clang_disposeTranslationUnit(self.0) };
    }
}

#[derive(Copy, Clone)]
pub struct Cursor(CXCursor);

impl Cursor {
    pub fn children(&self) -> Vec<Self> {
        extern "C" fn callback(
            cursor: CXCursor,
            _parent: CXCursor,
            data: CXClientData,
        ) -> CXChildVisitResult {
            let children = unsafe { &mut *(data as *mut Vec<Cursor>) };
            children.push(Cursor(cursor));
            CXChildVisit_Continue
        }

        let mut children = vec![];

        unsafe {
            clang_visitChildren(self.0, callback, &mut children as *mut _ as CXClientData);
        }

        children
    }

    pub fn kind(&self) -> CXCursorKind {
        unsafe { clang_getCursorKind(self.0) }
    }

    pub fn is_definition(&self) -> bool {
        unsafe { clang_isCursorDefinition(self.0) != 0 }
    }

    pub fn is_from_main_file(&self) -> bool {
        unsafe {
            let loc = clang_getCursorLocation(self.0);
            clang_Location_isFromMainFile(loc) != 0
        }
    }

    pub fn is_macro_builtin(&self) -> bool {
        unsafe { clang_Cursor_isMacroBuiltin(self.0) != 0 }
    }

    pub fn is_macro_function_like(&self) -> bool {
        unsafe { clang_Cursor_isMacroFunctionLike(self.0) != 0 }
    }

    pub fn extent(&self) -> CXSourceRange {
        unsafe { clang_getCursorExtent(self.0) }
    }

    pub fn enum_repr(&self) -> Type {
        Type(unsafe { clang_getEnumDeclIntegerType(self.0) })
    }

    pub fn name(&self) -> String {
        to_string(unsafe { clang_getCursorSpelling(self.0) })
    }

    pub fn enum_value(&self) -> i64 {
        unsafe { clang_getEnumConstantDeclValue(self.0) }
    }

    pub fn ty(&self) -> Type {
        Type(unsafe { clang_getCursorType(self.0) })
    }

    pub fn typedef_underlying_type(&self) -> Type {
        Type(unsafe { clang_getTypedefDeclUnderlyingType(self.0) })
    }

    pub fn result_type(&self) -> Type {
        Type(unsafe { clang_getCursorResultType(self.0) })
    }
}

pub struct Type(CXType);

impl Type {
    pub fn kind(&self) -> CXTypeKind {
        self.0.kind
    }

    pub fn ty(&self) -> Cursor {
        Cursor(unsafe { clang_getTypeDeclaration(self.0) })
    }

    pub fn pointee_type(&self) -> Self {
        Self(unsafe { clang_getPointeeType(self.0) })
    }

    pub fn is_const(&self) -> bool {
        unsafe { clang_isConstQualifiedType(self.0) != 0 }
    }

    pub fn underlying_type(&self) -> Self {
        Self(unsafe { clang_Type_getNamedType(self.0) })
    }

    pub fn is_function_pointer(&self) -> bool {
        self.function_pointee().is_some()
    }

    pub fn function_pointee(&self) -> Option<Self> {
        if self.kind() == CXType_Pointer {
            let pointee = self.pointee_type();
            if pointee.kind() == CXType_FunctionProto || pointee.kind() == CXType_FunctionNoProto {
                return Some(pointee);
            }
        }
        None
    }

    pub fn num_arg_types(&self) -> i32 {
        unsafe { clang_getNumArgTypes(self.0) }
    }

    pub fn arg_type(&self, index: u32) -> Self {
        Self(unsafe { clang_getArgType(self.0, index) })
    }

    pub fn fn_result_type(&self) -> Self {
        Self(unsafe { clang_getResultType(self.0) })
    }

    pub fn to_type(&self, namespace: &str) -> metadata::Type {
        match self.kind() {
            CXType_Void => metadata::Type::Void,
            CXType_Char_U | CXType_UChar => metadata::Type::U8,
            CXType_UShort => metadata::Type::U16,
            CXType_UInt => metadata::Type::U32,
            CXType_ULong => metadata::Type::USize,
            CXType_ULongLong => metadata::Type::U64,
            CXType_Char_S | CXType_SChar => metadata::Type::I8,
            CXType_Short => metadata::Type::I16,
            CXType_Int => metadata::Type::I32,
            CXType_Long => metadata::Type::ISize,
            CXType_LongLong => metadata::Type::I64,
            CXType_Float => metadata::Type::F32,
            CXType_Double => metadata::Type::F64,
            CXType_Record => metadata::Type::value_named(namespace, &self.ty().name()),
            CXType_Elaborated => self.underlying_type().to_type(namespace),
            CXType_Typedef => metadata::Type::value_named(namespace, &self.ty().name()),
            CXType_Pointer => {
                let pointee = self.pointee_type();
                // Function pointers map to opaque *mut u8; they are emitted
                // separately as callbacks via `Callback::parse`.
                if pointee.kind() == CXType_FunctionProto
                    || pointee.kind() == CXType_FunctionNoProto
                {
                    return metadata::Type::PtrMut(Box::new(metadata::Type::U8), 1);
                }
                let inner = pointee.to_type(namespace);
                if pointee.is_const() {
                    metadata::Type::PtrConst(Box::new(inner), 1)
                } else {
                    metadata::Type::PtrMut(Box::new(inner), 1)
                }
            }
            rest => panic!("{rest:?}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: CXDiagnosticSeverity,
    pub message: String,
    pub file_name: String,
    pub line: u32,
    pub column: u32,
}

impl Diagnostic {
    pub fn is_err(&self) -> bool {
        self.severity >= CXDiagnostic_Error
    }
}

fn to_string(cxstr: CXString) -> String {
    unsafe {
        let cstr_ptr = clang_getCString(cxstr);

        let result = if cstr_ptr.is_null() {
            String::new()
        } else {
            CStr::from_ptr(cstr_ptr).to_string_lossy().into_owned()
        };

        clang_disposeString(cxstr);
        result
    }
}
