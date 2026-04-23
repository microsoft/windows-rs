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

    /// Returns the libclang version string (e.g. `"clang version 18.1.0 ..."`).
    pub fn version(&self) -> String {
        to_string(unsafe { clang_getClangVersion() })
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
    /// The `flags` parameter is forwarded directly to
    /// `clang_parseTranslationUnit` as the `options` argument.  Callers should
    /// combine the `CXTranslationUnit_*` constants appropriate for their use
    /// case (e.g. `CXTranslationUnit_KeepGoing` for evaluation passes,
    /// `CXTranslationUnit_DetailedPreprocessingRecord` to expose macros).
    pub fn parse_unsaved(
        &self,
        filename: &str,
        content: &str,
        args: &[&str],
        flags: i32,
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
                flags,
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

    /// Convert a source range to one whose start and end are the **expansion**
    /// locations of the originals.
    ///
    /// When a cursor originates from a macro expansion, `clang_getCursorExtent`
    /// returns a range whose start location is in the macro body (the spelling
    /// location) and whose end location is the end of the expansion in the
    /// source file.  Passing this mixed range to `clang_tokenize` produces
    /// tokens from the macro body all the way to the expansion end, which can
    /// span multiple unrelated macro invocations.
    ///
    /// By converting both endpoints to their expansion locations we obtain a
    /// range that lives entirely in the source file at the specific call site,
    /// so `clang_tokenize` returns only the tokens for that invocation.
    pub fn to_expansion_range(&self, range: CXSourceRange) -> CXSourceRange {
        unsafe {
            let start = clang_getRangeStart(range);
            let end = clang_getRangeEnd(range);

            let mut start_file: CXFile = std::ptr::null_mut();
            let mut start_line: u32 = 0;
            let mut start_col: u32 = 0;
            let mut start_offset: u32 = 0;
            clang_getExpansionLocation(
                start,
                &mut start_file,
                &mut start_line,
                &mut start_col,
                &mut start_offset,
            );

            let mut end_file: CXFile = std::ptr::null_mut();
            let mut end_line: u32 = 0;
            let mut end_col: u32 = 0;
            let mut end_offset: u32 = 0;
            clang_getExpansionLocation(
                end,
                &mut end_file,
                &mut end_line,
                &mut end_col,
                &mut end_offset,
            );

            let new_start = clang_getLocation(self.0, start_file, start_line, start_col);
            let new_end = clang_getLocation(self.0, end_file, end_line, end_col);
            clang_getRange(new_start, new_end)
        }
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

    /// Returns the source file name for this cursor's spelling location, or an
    /// empty string if no file information is available (e.g. for built-in
    /// declarations).
    ///
    /// This is the physical file in which the declaration is written, and is
    /// used to match against the caller-supplied filter suffixes when deciding
    /// whether to include a declaration from a transitively included header.
    pub fn file_name(&self) -> String {
        unsafe {
            let loc = clang_getCursorLocation(self.0);
            let mut source_file: CXFile = std::ptr::null_mut();
            clang_getSpellingLocation(
                loc,
                &mut source_file,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            if source_file.is_null() {
                String::new()
            } else {
                to_string(clang_getFileName(source_file))
            }
        }
    }

    /// Returns `true` when the **expansion** location of this cursor is in the
    /// main input file.
    ///
    /// This differs from [`is_from_main_file()`] (which checks the spelling
    /// location) when the cursor originates from a macro whose definition is in
    /// an included header.  For example, if `extern "C"` is spelled inside
    /// `#define EXTERN_C extern "C"` in an included file but the macro is
    /// invoked in the main file, the spelling location is in the included
    /// header while the expansion location is in the main file.
    pub fn is_expansion_from_main_file(&self, tu: &TranslationUnit) -> bool {
        unsafe {
            let loc = clang_getCursorLocation(self.0);
            let mut file: CXFile = std::ptr::null_mut();
            let mut line: u32 = 0;
            let mut col: u32 = 0;
            clang_getExpansionLocation(loc, &mut file, &mut line, &mut col, std::ptr::null_mut());
            if file.is_null() {
                return false;
            }
            let expansion_loc = clang_getLocation(tu.0, file, line, col);
            clang_Location_isFromMainFile(expansion_loc) != 0
        }
    }

    pub fn is_macro_builtin(&self) -> bool {
        unsafe { clang_Cursor_isMacroBuiltin(self.0) != 0 }
    }

    pub fn is_macro_function_like(&self) -> bool {
        unsafe { clang_Cursor_isMacroFunctionLike(self.0) != 0 }
    }

    pub fn is_pure_virtual(&self) -> bool {
        unsafe { clang_CXXMethod_isPureVirtual(self.0) != 0 }
    }

    /// Returns true if this struct/class cursor has at least one pure-virtual method, indicating
    /// that it is a COM-style abstract interface rather than a plain data struct.
    ///
    /// When the cursor is a forward declaration (e.g. produced by `typedef struct IC IC;`
    /// before the definition has been seen), `clang_getCursorDefinition` is used to follow
    /// to the actual definition cursor whose children include the method declarations.
    pub fn has_pure_virtual_methods(&self) -> bool {
        // clang_getCursorDefinition returns the definition cursor if one exists, or a
        // null cursor if no definition is available.  For a forward declaration that was
        // created before the struct body was parsed, the definition is a *different*
        // cursor node and only that node has CXCursor_CXXMethod children.
        let defn = unsafe { clang_getCursorDefinition(self.0) };
        let cursor = if unsafe { clang_Cursor_isNull(defn) } == 0 {
            Cursor(defn)
        } else {
            Cursor(self.0)
        };
        cursor
            .children()
            .iter()
            .any(|c| c.kind() == CXCursor_CXXMethod && c.is_pure_virtual())
    }

    /// Searches the cursor's attribute children for a UUID string.
    ///
    /// Handles `CXCursor_UnexposedAttr`, produced by `__declspec(uuid("..."))` when compiled with
    /// `-fms-extensions`.  The UUID is extracted by tokenizing the attribute extent and finding a
    /// string-literal token that matches the UUID format.
    ///
    /// Returns the first UUID found (without quotes), or `None`.
    pub fn extract_uuid(&self, tu: &TranslationUnit) -> Option<String> {
        for child in self.children() {
            // __declspec(uuid("...")) with -fms-extensions.
            // When the attribute originates from a macro body (e.g. `MIDL_INTERFACE(x)`),
            // `child.extent()` spans from the macro body's spelling location all the way
            // to the end of the expansion.  For the second and later macro invocations
            // this covers the entire earlier content too, so `tokenize(child.extent())`
            // finds the first interface's UUID for every subsequent interface.
            //
            // Fix: convert both endpoints of the child's extent to their *expansion*
            // locations before tokenizing.  This produces a range that lives entirely at
            // the specific call site in the source file (e.g. `MIDL_INTERFACE("…")`),
            // so only the tokens for *this* invocation are returned.
            if child.kind() == CXCursor_UnexposedAttr {
                let expansion_range = tu.to_expansion_range(child.extent());
                for (kind, spelling) in tu.tokenize(expansion_range) {
                    if kind == CXToken_Literal && spelling.starts_with('"') {
                        let inner = spelling.trim_matches('"');
                        if is_uuid_format(inner) {
                            return Some(inner.to_string());
                        }
                    }
                }
            }
        }
        None
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

    pub fn language(&self) -> CXLanguageKind {
        unsafe { clang_getCursorLanguage(self.0) }
    }

    /// Returns a stable string key `"filename:line:col"` that uniquely
    /// identifies this cursor's source location within a translation unit.
    ///
    /// This is used to key anonymous struct/union declarations in the
    /// `tag_rename` map because `clang_getCursorSpelling` returns an empty
    /// string for every anonymous type, making the spelling unsuitable as a
    /// unique identifier.
    pub fn location_id(&self) -> String {
        unsafe {
            let loc = clang_getCursorLocation(self.0);
            let mut source_file: CXFile = std::ptr::null_mut();
            let mut line: u32 = 0;
            let mut col: u32 = 0;
            clang_getExpansionLocation(
                loc,
                &mut source_file,
                &mut line,
                &mut col,
                std::ptr::null_mut(),
            );
            let filename = if source_file.is_null() {
                String::new()
            } else {
                to_string(clang_getFileName(source_file))
            };
            format!("{filename}:{line}:{col}")
        }
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

    pub fn array_element_type(&self) -> Self {
        Self(unsafe { clang_getArrayElementType(self.0) })
    }

    pub fn array_size(&self) -> usize {
        unsafe { clang_getArraySize(self.0) as usize }
    }

    /// Returns `true` if this is a variadic function type (has a trailing `...`
    /// parameter).  For a `CXCursor_FunctionDecl` cursor call `cursor.ty().is_variadic()`.
    pub fn is_variadic(&self) -> bool {
        unsafe { clang_isFunctionTypeVariadic(self.0) != 0 }
    }

    /// Returns `true` if this type is a COM-style abstract interface (a record
    /// type whose declaration has at least one pure-virtual method).
    ///
    /// `CXType_Elaborated` is unwrapped transparently because clang frequently
    /// wraps record types in an elaborated-type node in C++ mode.
    ///
    /// `CXType_Typedef` is also unwrapped so that forward-declared interfaces
    /// (`typedef struct IC IC;`) are detected correctly when the pointee type
    /// is represented as the typedef rather than the underlying record.
    ///
    /// `has_pure_virtual_methods()` additionally follows `clang_getCursorDefinition`
    /// so that a forward-declaration cursor (which has no children) never
    /// falsely suppresses interface detection.
    pub fn is_interface(&self) -> bool {
        match self.kind() {
            CXType_Record => self.ty().has_pure_virtual_methods(),
            CXType_Elaborated => self.underlying_type().is_interface(),
            CXType_Typedef => self.ty().typedef_underlying_type().is_interface(),
            _ => false,
        }
    }

    /// Convert this clang type to a metadata `Type`.
    ///
    /// `parser` carries the shared parse context: the current namespace,
    /// reference map, tag-rename map, and the `pending_typedefs` accumulator
    /// for typedef declarations from included headers.
    pub fn to_type(&self, parser: &mut Parser<'_>) -> metadata::Type {
        match self.kind() {
            CXType_Void => metadata::Type::Void,
            CXType_Bool => metadata::Type::Bool,
            CXType_Char_U | CXType_UChar => metadata::Type::U8,
            CXType_UShort => metadata::Type::U16,
            CXType_UInt => metadata::Type::U32,
            CXType_ULong => metadata::Type::U32,
            CXType_ULongLong => metadata::Type::U64,
            CXType_Char_S | CXType_SChar => metadata::Type::I8,
            CXType_Short => metadata::Type::I16,
            CXType_Int => metadata::Type::I32,
            CXType_Long => metadata::Type::I32,
            CXType_LongLong => metadata::Type::I64,
            CXType_Float => metadata::Type::F32,
            CXType_Double => metadata::Type::F64,
            CXType_WChar => metadata::Type::U16,
            CXType_Enum | CXType_Record => {
                let decl = self.ty();
                let tag_name = decl.name();
                // For anonymous types (empty or "(anonymous …)") the cursor
                // spelling is not a usable key; fall back to the source
                // location which is unique per declaration site.
                let name = if is_anonymous_name(&tag_name) {
                    parser
                        .tag_rename
                        .get(&decl.location_id())
                        .cloned()
                        .unwrap_or(tag_name)
                } else {
                    // Apply the tag→typedef rename so that the internal tag (e.g. `_TEST`)
                    // is always replaced by its public typedef alias (e.g. `TEST`).
                    parser
                        .tag_rename
                        .get(&tag_name)
                        .cloned()
                        .unwrap_or(tag_name)
                };
                let ns = parser
                    .ref_map
                    .get(&name)
                    .map(|s| s.as_str())
                    .unwrap_or(parser.namespace);
                metadata::Type::value_named(ns, &name)
            }
            CXType_Elaborated => self.underlying_type().to_type(parser),
            CXType_Typedef => {
                let decl = self.ty();
                let name = decl.name();
                if let Some(ns) = parser.ref_map.get(&name) {
                    // Type is known in the reference metadata — use the qualified name.
                    metadata::Type::value_named(ns, &name)
                } else if decl.is_from_main_file() {
                    // Local typedef — it will be emitted separately as a `type` item.
                    metadata::Type::value_named(parser.namespace, &name)
                } else {
                    // Typedef from an included/system header that is not present in
                    // the reference metadata.  Preserve the typedef name so the
                    // semantics (e.g. BYTE, DWORD) are visible in the generated RDL,
                    // and schedule its definition for emission in a follow-up pass.
                    parser.pending_typedefs.push(decl);
                    metadata::Type::value_named(parser.namespace, &name)
                }
            }
            CXType_Pointer => {
                let pointee = self.pointee_type();
                // Function pointers map to opaque *mut u8; they are emitted
                // separately as callbacks via `Callback::parse`.
                if pointee.kind() == CXType_FunctionProto
                    || pointee.kind() == CXType_FunctionNoProto
                {
                    return metadata::Type::PtrMut(Box::new(metadata::Type::U8), 1);
                }
                // Interface types are implied pointers in Windows metadata and RDL:
                // `IA*` is written as `IA` and `IA**` is written as `*mut IA`.
                // Strip one level of pointer indirection when the pointee is an
                // interface so the generated RDL matches the Windows convention.
                if pointee.is_interface() {
                    return pointee.to_type(parser);
                }
                let inner = pointee.to_type(parser);
                if pointee.is_const() {
                    // Flatten consecutive const-pointer levels: PtrConst(PtrConst(T, n), 1) → PtrConst(T, n+1)
                    match inner {
                        metadata::Type::PtrConst(t, n) => metadata::Type::PtrConst(t, n + 1),
                        inner => metadata::Type::PtrConst(Box::new(inner), 1),
                    }
                } else {
                    // Flatten consecutive mut-pointer levels: PtrMut(PtrMut(T, n), 1) → PtrMut(T, n+1)
                    match inner {
                        metadata::Type::PtrMut(t, n) => metadata::Type::PtrMut(t, n + 1),
                        inner => metadata::Type::PtrMut(Box::new(inner), 1),
                    }
                }
            }
            CXType_LValueReference => {
                let pointee = self.pointee_type();
                let inner = pointee.to_type(parser);
                if pointee.is_const() {
                    metadata::Type::RefConst(Box::new(inner))
                } else {
                    metadata::Type::RefMut(Box::new(inner))
                }
            }
            CXType_ConstantArray => {
                let element = self.array_element_type().to_type(parser);
                let size = self.array_size();
                metadata::Type::ArrayFixed(Box::new(element), size)
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

/// Returns `true` if `s` matches the standard UUID format `xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx`
/// (8-4-4-4-12 lowercase or uppercase hex digits separated by hyphens).
pub fn is_uuid_format(s: &str) -> bool {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 5 {
        return false;
    }
    let lengths = [8usize, 4, 4, 4, 12];
    parts
        .iter()
        .zip(lengths.iter())
        .all(|(p, &l)| p.len() == l && p.chars().all(|c| c.is_ascii_hexdigit()))
}

/// Returns `true` if `name` is the spelling of an anonymous struct/union.
///
/// `clang_getCursorSpelling` returns `""` for anonymous types on most libclang
/// versions; some versions return a synthesised string beginning with `"("`.
/// Both forms are treated as anonymous.
pub fn is_anonymous_name(name: &str) -> bool {
    name.is_empty() || name.starts_with('(')
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
