use super::*;
pub use clang_sys::*;
use std::ffi::{CStr, CString};

pub struct Library;

impl Library {
    pub fn new() -> Result<Self, Error> {
        load().map_err(|e| Error::new(&format!("failed to load libclang: {e}"), "", 0, 0))?;
        Ok(Self)
    }

    pub fn version(&self) -> String {
        to_string(unsafe { clang_getClangVersion() })
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        _ = unload();
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

    /// Parses a virtual source file with caller-supplied libclang flags.
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

    /// Moves both range endpoints to expansion locations so macro tokenization stays at
    /// the call site instead of spanning the macro body and unrelated expansions.
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

    /// Source-adjacency check for function-like macros; libclang can misreport some
    /// SDK macros as object-like, leaking parameter names into constant scraping.
    pub fn macro_is_function_like_by_source(&self, range: CXSourceRange) -> bool {
        unsafe {
            let mut tokens: *mut CXToken = std::ptr::null_mut();
            let mut n_tokens: u32 = 0;
            clang_tokenize(self.0, range, &mut tokens, &mut n_tokens);
            if n_tokens < 2 {
                if !tokens.is_null() {
                    clang_disposeTokens(self.0, tokens, n_tokens);
                }
                return false;
            }

            let offset = |token: CXToken, end: bool| -> u32 {
                let ext = clang_getTokenExtent(self.0, token);
                let loc = if end {
                    clang_getRangeEnd(ext)
                } else {
                    clang_getRangeStart(ext)
                };
                let mut file: CXFile = std::ptr::null_mut();
                let mut line: u32 = 0;
                let mut col: u32 = 0;
                let mut off: u32 = 0;
                clang_getSpellingLocation(loc, &mut file, &mut line, &mut col, &mut off);
                off
            };

            let name_end = offset(*tokens.add(0), true);
            let second = *tokens.add(1);
            let second_spelling = to_string(clang_getTokenSpelling(self.0, second));
            let second_start = offset(second, false);
            let function_like = second_spelling == "(" && second_start == name_end;

            clang_disposeTokens(self.0, tokens, n_tokens);
            function_like
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

    /// Used to detect `struct { ... } field;` as an inline nested type.
    pub fn semantic_parent(&self) -> Self {
        Self(unsafe { clang_getCursorSemanticParent(self.0) })
    }

    pub fn is_from_main_file(&self) -> bool {
        unsafe {
            let loc = clang_getCursorLocation(self.0);
            clang_Location_isFromMainFile(loc) != 0
        }
    }

    /// File containing the declaration spelling; used for included-header filtering.
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

    /// Checks the expansion location, not the macro definition spelling location.
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

    /// File containing the macro expansion site; routes declarations such as
    /// `STDAPI Foo(...)` to the API header instead of the macro definition.
    pub fn expansion_file_name(&self) -> String {
        unsafe {
            let loc = clang_getCursorLocation(self.0);
            let mut file: CXFile = std::ptr::null_mut();
            clang_getExpansionLocation(
                loc,
                &mut file,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            if file.is_null() {
                String::new()
            } else {
                to_string(clang_getFileName(file))
            }
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

    /// Old-style COM headers redeclare inherited methods; those occupy existing vtable
    /// slots and must not be emitted as new methods.
    pub fn overrides_base_method(&self) -> bool {
        let mut cursors = std::ptr::null_mut();
        let mut count = 0;
        unsafe {
            clang_getOverriddenCursors(self.0, &mut cursors, &mut count);
            if !cursors.is_null() {
                clang_disposeOverriddenCursors(cursors);
            }
        }
        count > 0
    }

    pub fn has_definition(&self) -> bool {
        let defn = unsafe { clang_getCursorDefinition(self.0) };
        unsafe { clang_Cursor_isNull(defn) == 0 }
    }

    /// Stable first declaration for SDK typedefs repeated across headers.
    pub fn canonical(&self) -> Self {
        Self(unsafe { clang_getCanonicalCursor(self.0) })
    }

    /// Stable redeclaration identity used to deduplicate SDK declarations.
    pub fn usr(&self) -> String {
        to_string(unsafe { clang_getCursorUSR(self.0) })
    }

    /// Follows forward declarations so per-header routing points at the emitted type.
    pub fn definition(&self) -> Self {
        let defn = unsafe { clang_getCursorDefinition(self.0) };
        if unsafe { clang_Cursor_isNull(defn) } == 0 {
            Self(defn)
        } else {
            Self(self.0)
        }
    }

    pub fn referenced(&self) -> Self {
        Self(unsafe { clang_getCursorReferenced(self.0) })
    }

    /// Checks the definition cursor so forward-declared COM interfaces are classified by
    /// their method declarations.
    pub fn has_pure_virtual_methods(&self) -> bool {
        let defn = unsafe { clang_getCursorDefinition(self.0) };
        let cursor = if unsafe { clang_Cursor_isNull(defn) } == 0 {
            Self(defn)
        } else {
            Self(self.0)
        };
        cursor
            .children()
            .iter()
            .any(|c| c.kind() == CXCursor_CXXMethod && c.is_pure_virtual())
    }

    /// Detects marker interfaces with no methods of their own, such as `ID2D1Image`.
    pub fn has_interface_base(&self) -> bool {
        let defn = unsafe { clang_getCursorDefinition(self.0) };
        let cursor = if unsafe { clang_Cursor_isNull(defn) } == 0 {
            Self(defn)
        } else {
            Self(self.0)
        };
        cursor
            .children()
            .iter()
            .any(|c| c.kind() == CXCursor_CXXBaseSpecifier && c.ty().is_interface())
    }

    /// Keeps structs with an interface base and real fields from being treated as marker
    /// interfaces.
    pub fn has_data_fields(&self) -> bool {
        let defn = unsafe { clang_getCursorDefinition(self.0) };
        let cursor = if unsafe { clang_Cursor_isNull(defn) } == 0 {
            Self(defn)
        } else {
            Self(self.0)
        };
        cursor
            .children()
            .iter()
            .any(|c| c.kind() == CXCursor_FieldDecl)
    }

    /// Extracts the first `__declspec(uuid("..."))` string from attribute tokens.
    pub fn extract_uuid(&self, tu: &TranslationUnit) -> Option<String> {
        for child in self.children() {
            // Macro-origin attributes need expansion ranges to avoid earlier invocations.
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

    pub fn is_scoped_enum(&self) -> bool {
        unsafe { clang_EnumDecl_isScoped(self.0) != 0 }
    }

    pub fn name(&self) -> String {
        to_string(unsafe { clang_getCursorSpelling(self.0) })
    }

    pub fn enum_value(&self) -> i64 {
        unsafe { clang_getEnumConstantDeclValue(self.0) }
    }

    pub fn evaluate_unsigned(&self) -> Option<u64> {
        unsafe {
            let result = clang_Cursor_Evaluate(self.0);
            if result.is_null() {
                return None;
            }
            let kind = clang_EvalResult_getKind(result);
            let value = if kind == CXEval_Int {
                Some(clang_EvalResult_getAsUnsigned(result))
            } else {
                None
            };
            clang_EvalResult_dispose(result);
            value
        }
    }

    pub fn evaluate_integer(&self) -> Option<(u64, i64)> {
        unsafe {
            let result = clang_Cursor_Evaluate(self.0);
            if result.is_null() {
                return None;
            }
            let kind = clang_EvalResult_getKind(result);
            let value = if kind == CXEval_Int {
                Some((
                    clang_EvalResult_getAsUnsigned(result),
                    clang_EvalResult_getAsLongLong(result),
                ))
            } else {
                None
            };
            clang_EvalResult_dispose(result);
            value
        }
    }

    /// Allows integer initializers for floating constants, matching libclang evaluation.
    pub fn evaluate_double(&self) -> Option<f64> {
        unsafe {
            let result = clang_Cursor_Evaluate(self.0);
            if result.is_null() {
                return None;
            }
            let kind = clang_EvalResult_getKind(result);
            let value = match kind {
                CXEval_Float => Some(clang_EvalResult_getAsDouble(result)),
                // Libclang may evaluate a floating constant integer initializer as `CXEval_Int`.
                CXEval_Int => Some(clang_EvalResult_getAsLongLong(result) as f64),
                _ => None,
            };
            clang_EvalResult_dispose(result);
            value
        }
    }

    pub fn ty(&self) -> Type {
        Type(unsafe { clang_getCursorType(self.0) })
    }

    pub fn is_bit_field(&self) -> bool {
        unsafe { clang_Cursor_isBitField(self.0) != 0 }
    }

    /// Anonymous record members produce no `FieldDecl`; rebuild their promoted fields.
    pub fn is_anonymous_record(&self) -> bool {
        unsafe { clang_Cursor_isAnonymousRecordDecl(self.0) != 0 }
    }

    /// Returns `0` for zero-width bit-fields, which force a fresh storage unit.
    pub fn bit_field_width(&self) -> i32 {
        unsafe { clang_getFieldDeclBitWidth(self.0) }
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

    /// Stable key for anonymous declarations whose spelling is empty or synthetic.
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

    pub fn spelling(&self) -> String {
        to_string(unsafe { clang_getTypeSpelling(self.0) })
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

    pub fn canonical_type(&self) -> Self {
        Self(unsafe { clang_getCanonicalType(self.0) })
    }

    /// Recovers closed WinRT generic arguments from canonical C++ specializations.
    pub fn num_template_args(&self) -> i32 {
        unsafe { clang_Type_getNumTemplateArguments(self.0) }
    }

    pub fn template_arg_type(&self, i: u32) -> Self {
        Self(unsafe { clang_Type_getTemplateArgumentAsType(self.0, i) })
    }

    /// Maps C++/WinRT ABI generic arguments like `HSTRING` and `IInspectable *` back to
    /// WinRT metadata types.
    fn winrt_generic_arg(&self, parser: &mut Parser<'_>) -> metadata::Type {
        let mut peeled = self.canonical_type();
        while peeled.kind() == CXType_Pointer {
            peeled = peeled.pointee_type();
        }
        match peeled.ty().name().as_str() {
            "HSTRING__" => metadata::Type::String,
            "IInspectable" => metadata::Type::Object,
            _ => self.to_type(parser),
        }
    }

    pub fn is_function_pointer(&self) -> bool {
        self.function_pointee().is_some()
    }

    pub fn function_pointee(&self) -> Option<Self> {
        match self.kind() {
            // Function-type typedefs emit as callbacks like pointer typedefs.
            CXType_FunctionProto | CXType_FunctionNoProto => Some(Self(self.0)),
            CXType_Pointer => {
                let pointee = self.pointee_type();
                if pointee.kind() == CXType_FunctionProto
                    || pointee.kind() == CXType_FunctionNoProto
                {
                    return Some(pointee);
                }
                None
            }
            _ => None,
        }
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

    pub fn align_of(&self) -> i64 {
        unsafe { clang_Type_getAlignOf(self.0) }
    }

    pub fn size_of(&self) -> i64 {
        unsafe { clang_Type_getSizeOf(self.0) }
    }

    pub fn is_variadic(&self) -> bool {
        unsafe { clang_isFunctionTypeVariadic(self.0) != 0 }
    }

    /// Detects COM interfaces through records, elaborated types, typedefs, and forward
    /// declarations.
    pub fn is_interface(&self) -> bool {
        match self.kind() {
            CXType_Record => {
                let decl = self.ty();
                decl.has_pure_virtual_methods() || decl.has_interface_base()
            }
            CXType_Elaborated => self.underlying_type().is_interface(),
            CXType_Typedef => {
                // ABI projection typedefs may have no instantiated methods; classify the
                // canonical ABI form so pointer collapse still applies.
                let canonical = self.canonical_type();
                if canonical.kind() != CXType_Pointer {
                    let spelling = canonical.spelling();
                    if spelling.starts_with("ABI::") {
                        if spelling.contains('<') {
                            return true;
                        }
                        // Defined ABI records are checked structurally so value structs stay
                        // values; incomplete ABI records are projection-interface references.
                        if canonical.kind() == CXType_Record {
                            let decl = canonical.ty();
                            return decl.has_pure_virtual_methods()
                                || decl.has_interface_base()
                                || !decl.has_definition();
                        }
                    }
                }
                self.ty().typedef_underlying_type().is_interface()
            }
            _ => false,
        }
    }

    /// Detects opaque handle typedefs backed by `struct NAME__ *` or MIDL placeholder
    /// tags, including SAL-wrapped forms.
    pub fn is_handle_tag(&self, name: &str) -> bool {
        let canonical = self.canonical_type();
        if canonical.kind() != CXType_Pointer {
            return false;
        }
        let pointee = canonical.pointee_type();
        if pointee.kind() != CXType_Record {
            return false;
        }
        let decl = pointee.ty();
        decl.name() == format!("{name}__")
            || (is_midl_placeholder_tag(&decl.name()) && is_handle_shape(&decl))
    }

    /// Resolves `ABI::Windows::*` C++/WinRT projection spellings, keeping WinRT
    /// references cross-winmd and capturing absent interop types into the flat root.
    fn abi_projection(&self, parser: &mut Parser<'_>) -> Option<metadata::Type> {
        let canonical = self.canonical_type().spelling();
        let projected = canonical.strip_prefix("ABI::")?;
        // The stem drops generic arguments; the closed list is rebuilt below.
        let stem = projected.split('<').next().unwrap_or(projected);
        let (namespace, name) = stem.rsplit_once("::")?;
        let ns = namespace.replace("::", ".");
        // Names absent from the resolution winmd are captured as Win32 interop types.
        if let Some(set) = parser.winrt_types
            && !set.contains(&format!("{ns}.{name}"))
        {
            return Some(metadata::Type::value_named(parser.namespace, name));
        }
        let record = self.canonical_type();
        let num = record.num_template_args();
        let generics = if num > 0 {
            (0..num as u32)
                .map(|i| record.template_arg_type(i).winrt_generic_arg(parser))
                .collect()
        } else {
            vec![]
        };
        Some(metadata::Type::ClassName(metadata::TypeName {
            namespace: ns,
            name: name.to_string(),
            generics,
        }))
    }

    pub fn to_type(&self, parser: &mut Parser<'_>) -> metadata::Type {
        if is_fundamental_scalar_kind(self.kind()) {
            return scalar_kind_to_type(self.kind());
        }
        match self.kind() {
            CXType_Void => metadata::Type::Void,
            CXType_LongDouble => metadata::Type::F64,
            CXType_Enum | CXType_Record => {
                if parser.winrt_types.is_some()
                    && let Some(projected) = self.abi_projection(parser)
                {
                    return projected;
                }
                let decl = self.ty();
                let tag_name = decl.name();
                // Anonymous spellings need the declaration-location rename key.
                let name = if is_anonymous_name(&tag_name) {
                    parser
                        .tag_rename
                        .get(&decl.location_id())
                        .cloned()
                        .unwrap_or(tag_name)
                } else {
                    // Prefer the public typedef alias over the internal tag.
                    parser
                        .tag_rename
                        .get(&tag_name)
                        .cloned()
                        .unwrap_or(tag_name)
                };
                // Inline anonymous enums have no referenceable type; their constants are
                // emitted separately, so references use the underlying integer type.
                if self.kind() == CXType_Enum && is_anonymous_name(&name) {
                    return decl.enum_repr().to_type(parser);
                }
                // Flat scrape Numerics aliases collapse to the shared Numerics projection.
                if parser.header_root.is_some()
                    && let Some(num) = numerics_alias(&name)
                {
                    return metadata::Type::value_named(NUMERICS_NAMESPACE, num);
                }
                let ns = if parser.header_root.is_some() {
                    // In flat mode, headers select files but all records share one namespace.
                    parser.namespace.to_string()
                } else {
                    parser
                        .ref_map
                        .get(&name)
                        .map_or(parser.namespace, |s| s.as_str())
                        .to_string()
                };
                // Pointer-only incomplete records need an opaque forward declaration target.
                if parser.header_root.is_some()
                    && !is_anonymous_name(&name)
                    && !name.ends_with("__")
                    && !decl.has_definition()
                    && let Some(stem) = header_stem_of(&decl.definition())
                {
                    parser.pending_opaque.push((stem, name.clone()));
                }
                metadata::Type::value_named(&ns, &name)
            }
            CXType_Elaborated => self.underlying_type().to_type(parser),
            CXType_Typedef => {
                // Inline MIDL ABI generic aliases and ABI record typedefs to the projection
                // type; ABI enum typedefs keep their own names.
                let canonical_type = self.canonical_type();
                let canonical = canonical_type.spelling();
                if parser.winrt_types.is_some()
                    && canonical.starts_with("ABI::")
                    && (canonical.contains('<') || canonical_type.kind() == CXType_Record)
                    && let Some(projected) = self.abi_projection(parser)
                {
                    return projected;
                }
                resolve_typedef(self, parser)
            }
            CXType_Pointer => {
                let pointee = self.pointee_type();
                // Function pointers emit separately as callbacks.
                if pointee.kind() == CXType_FunctionProto
                    || pointee.kind() == CXType_FunctionNoProto
                {
                    // Recover `NAME *` for pointers to function-type typedefs so delegate
                    // aliases use the named callback instead of an opaque pointer.
                    if let Some(name) = named_function_typedef_pointer(self) {
                        let ns = parser
                            .ref_map
                            .get(&name)
                            .map_or(parser.namespace, String::as_str);
                        return metadata::Type::value_named(ns, &name);
                    }
                    return metadata::Type::PtrMut(Box::new(metadata::Type::U8), 1);
                }
                // Interface pointers have one implied level in metadata and RDL.
                if pointee.is_interface() {
                    return pointee.to_type(parser);
                }
                let inner = pointee.to_type(parser);
                if pointee.is_const() {
                    match inner {
                        metadata::Type::PtrConst(t, n) => metadata::Type::PtrConst(t, n + 1),
                        inner => metadata::Type::PtrConst(Box::new(inner), 1),
                    }
                } else {
                    match inner {
                        metadata::Type::PtrMut(t, n) => metadata::Type::PtrMut(t, n + 1),
                        inner => metadata::Type::PtrMut(Box::new(inner), 1),
                    }
                }
            }
            CXType_LValueReference => {
                // C++ references are passed like pointers; `RefConst`/`RefMut` are only
                // WinRT metadata concepts.
                let pointee = self.pointee_type();
                // Interface references use the same implied-pointer rule as `IA*`.
                if pointee.is_interface() {
                    return pointee.to_type(parser);
                }
                let inner = pointee.to_type(parser);
                if pointee.is_const() {
                    match inner {
                        metadata::Type::PtrConst(t, n) => metadata::Type::PtrConst(t, n + 1),
                        inner => metadata::Type::PtrConst(Box::new(inner), 1),
                    }
                } else {
                    match inner {
                        metadata::Type::PtrMut(t, n) => metadata::Type::PtrMut(t, n + 1),
                        inner => metadata::Type::PtrMut(Box::new(inner), 1),
                    }
                }
            }
            CXType_ConstantArray => {
                let element = self.array_element_type().to_type(parser);
                let size = self.array_size();
                metadata::Type::ArrayFixed(Box::new(element), size)
            }
            CXType_IncompleteArray => {
                let element = self.array_element_type().to_type(parser);
                metadata::Type::ArrayFixed(Box::new(element), 0)
            }
            // Bare function types in value positions decay to opaque function pointers.
            CXType_FunctionProto | CXType_FunctionNoProto => {
                metadata::Type::PtrMut(Box::new(metadata::Type::Void), 1)
            }
            rest => {
                // Late ABI projection fallback for out-of-scope WinRT references and kept
                // interop entities.
                if let Some(projected) = self.abi_projection(parser) {
                    return projected;
                }
                let spelling = self.spelling();
                let canonical = self.canonical_type().spelling();
                panic!(
                    "unhandled type kind {rest:?}: spelling={:?} canonical={:?} decl_at={}",
                    spelling,
                    canonical,
                    self.ty().location_id()
                )
            }
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

/// Handles the empty and synthesized anonymous spellings libclang returns.
pub fn is_anonymous_name(name: &str) -> bool {
    name.is_empty()
        || name.starts_with('(')
        || name.contains("(unnamed ")
        || name.contains("(anonymous ")
}

/// MIDL anonymous enum tags carry no type identity; emit their variants as constants.
pub fn is_midl_anonymous_enum_name(name: &str) -> bool {
    name.starts_with("__MIDL_")
}

/// MIDL file-scope placeholder tags back opaque handles and should not surface.
pub fn is_midl_placeholder_tag(name: &str) -> bool {
    name.starts_with("__MIDL___MIDL_itf_")
}

/// MIDL names unnamed parameters this way; replace them with positional names.
pub fn is_midl_synthetic_param_name(name: &str) -> bool {
    name.starts_with("__MIDL__")
}

/// Opaque handle tags are empty or contain only the dummy `int unused` field.
pub fn is_handle_shape(decl: &Cursor) -> bool {
    let field_kinds: Vec<CXTypeKind> = decl
        .children()
        .into_iter()
        .filter(|c| c.kind() == CXCursor_FieldDecl)
        .map(|c| c.ty().kind())
        .collect();
    field_kinds.is_empty() || (field_kinds.len() == 1 && field_kinds[0] == CXType_Int)
}

/// Recovers `NAME` from `NAME *` when `clang_getPointeeType` drops function-type
/// typedef sugar.
fn named_function_typedef_pointer(ty: &Type) -> Option<String> {
    let spelling = ty.spelling();
    let inner = spelling.strip_prefix("const ").unwrap_or(&spelling);
    let name = inner.strip_suffix('*')?.trim_end();
    let mut chars = name.chars();
    let first = chars.next()?;
    if !(first.is_ascii_alphabetic() || first == '_') {
        return None;
    }
    if chars.all(|c| c.is_ascii_alphanumeric() || c == '_') {
        Some(name.to_string())
    } else {
        None
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
