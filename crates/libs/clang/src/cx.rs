use super::*;
pub use clang_sys::*;
use std::ffi::{CStr, CString};

pub struct Library;

impl Library {
    pub fn new() -> Result<Self, Error> {
        load().map_err(|e| Error::new(&format!("failed to load libclang: {e}"), "", 0, 0))?;
        Ok(Self)
    }

    /// Returns the libclang version string (e.g. `"clang version 18.1.0 ..."`).
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

    /// Returns `true` when a `CXCursor_MacroDefinition` is function-like by source
    /// adjacency — its name is immediately followed by `(` with no intervening
    /// whitespace (`#define FOO(x) ...`). This is a robust backstop for
    /// `clang_Cursor_isMacroFunctionLike`, which can misreport some macros (e.g.
    /// `#define ASSERT(exp) ((VOID)0)` in `mswsockdef.h`) as object-like, causing
    /// the parameter name to leak into the constant scraper as a bogus type.
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

    /// The semantic parent of this cursor — for a record defined inside another
    /// record, the enclosing record. Used to detect the "named instance" idiom
    /// (`struct { … } field;`) so it can be emitted as an inline nested type.
    pub fn semantic_parent(&self) -> Self {
        Self(unsafe { clang_getCursorSemanticParent(self.0) })
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

    /// Returns the source file name for this cursor's **expansion** location, or
    /// an empty string if none is available.
    ///
    /// Unlike [`file_name()`](Self::file_name) (the spelling location), this is
    /// where a macro is *invoked* rather than where its tokens are spelled. For a
    /// macro-wrapped declaration such as `STDAPI Foo(...)` (i.e.
    /// `extern "C" HRESULT Foo(...)`), the wrapping `CXCursor_LinkageSpec` is
    /// spelled at the `EXTERN_C` macro definition but expanded at the call site
    /// (the API header), so the expansion file is the one that determines which
    /// namespace partition owns the declaration.
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

    /// Returns `true` if this method overrides a virtual method from a base class.
    ///
    /// Old-style COM headers (`DECLARE_INTERFACE_`) redeclare every inherited method
    /// (`QueryInterface`/`AddRef`/`Release` and the whole ancestor chain) in each derived
    /// interface. Those redeclarations override the base virtuals — they occupy existing
    /// vtable slots rather than adding new ones — so they must not be re-emitted on top of
    /// the inherited base vtable.
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

    /// Returns `true` if a definition for this cursor's class/struct exists in the
    /// translation unit.
    ///
    /// For a forward-declaration cursor (e.g. `class DiaSource;`) this is `true` when the
    /// class body appears elsewhere in the same TU, and `false` when the class is never
    /// defined (only declared).
    pub fn has_definition(&self) -> bool {
        let defn = unsafe { clang_getCursorDefinition(self.0) };
        unsafe { clang_Cursor_isNull(defn) == 0 }
    }

    /// Returns the *canonical* (first) declaration cursor for a redeclarable entity.
    ///
    /// A typedef such as `ULONG` or `PROPVARIANT` may be declared identically in
    /// several headers (e.g. `winnt.h` and `winsmcrd.h` both `typedef`-declare
    /// `ULONG`). `clang_getTypeDeclaration` returns whichever redeclaration clang
    /// happens to resolve at a given reference site, so per-header routing must
    /// canonicalize to the first declaration in translation order — a stable,
    /// most-upstream choice — to attribute every reference to the same defining
    /// header.
    pub fn canonical(&self) -> Self {
        Self(unsafe { clang_getCanonicalCursor(self.0) })
    }

    /// Returns clang's Unified Symbol Resolution string for this cursor: a stable
    /// identity that is identical across every redeclaration of the same entity
    /// (same name + linkage) regardless of which header it is written in. Used to
    /// canonically deduplicate declarations that the SDK repeats across headers.
    /// May be empty for entities clang does not assign a USR (e.g. anonymous types).
    pub fn usr(&self) -> String {
        to_string(unsafe { clang_getCursorUSR(self.0) })
    }

    /// Returns the cursor's defining declaration, following a forward declaration to
    /// its definition when one exists in the translation unit.
    ///
    /// A header may forward-declare a struct (e.g. `excpt.h`'s `struct _EXCEPTION_RECORD;`)
    /// and reference it before the real definition is seen (in `winnt.h`). Per-header
    /// routing must attribute such a reference to the *definition's* header, not the
    /// forward declaration's, so the cross-header reference resolves to where the type is
    /// actually emitted.
    pub fn definition(&self) -> Self {
        let defn = unsafe { clang_getCursorDefinition(self.0) };
        if unsafe { clang_Cursor_isNull(defn) } == 0 {
            Self(defn)
        } else {
            Self(self.0)
        }
    }

    /// Returns the cursor referenced by this cursor (e.g. the declaration a
    /// `CXCursor_TypeRef` points at), or a null cursor when there is none.
    pub fn referenced(&self) -> Self {
        Self(unsafe { clang_getCursorReferenced(self.0) })
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
            Self(defn)
        } else {
            Self(self.0)
        };
        cursor
            .children()
            .iter()
            .any(|c| c.kind() == CXCursor_CXXMethod && c.is_pure_virtual())
    }

    /// Whether this record derives from a base that is itself a COM interface.
    ///
    /// A *marker* interface such as `ID2D1Image` (`interface ID2D1Image :
    /// ID2D1Resource {}`) declares no methods of its own, so
    /// [`has_pure_virtual_methods`](Self::has_pure_virtual_methods) alone
    /// misclassifies it as a plain record — and its `IFoo*` parameters then fail
    /// to collapse to the bare interface. Walking the single-inheritance base
    /// chain fixes this; it terminates at `IUnknown`, whose
    /// `QueryInterface`/`AddRef`/`Release` are pure virtual.
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

    /// True when this record declares at least one non-static data member.
    ///
    /// Distinguishes a genuine data struct that happens to inherit an interface base
    /// (e.g. `MFASYNCRESULT`, which derives from `IMFAsyncResult` but carries
    /// `overlapped`/`pCallback`/... fields and must stay a struct) from a pure
    /// *marker* interface (`IVssWriterComponentsExt`, `DebugBaseEventCallbacks`) that
    /// derives from an interface base and declares no fields of its own.
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
            //
            // When the attribute originates from a macro body (e.g. `MIDL_INTERFACE(x)`),
            // `child.extent()` spans from the macro body's spelling location all the way
            // to the end of the expansion, which would cover earlier macro invocations
            // too. Converting both endpoints to expansion locations restricts the range
            // to the specific call site.
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

    /// True for a C++ scoped enumeration (`enum class` / `enum struct`).
    pub fn is_scoped_enum(&self) -> bool {
        unsafe { clang_EnumDecl_isScoped(self.0) != 0 }
    }

    pub fn name(&self) -> String {
        to_string(unsafe { clang_getCursorSpelling(self.0) })
    }

    pub fn enum_value(&self) -> i64 {
        unsafe { clang_getEnumConstantDeclValue(self.0) }
    }

    /// Evaluate this cursor as a compile-time constant expression and return
    /// the unsigned integer result, or `None` if evaluation fails or the result
    /// is not an integer.
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

    /// Evaluate this cursor as a compile-time constant expression and return the
    /// floating-point result, or `None` if evaluation fails or the result is not a
    /// float. Integer results are *not* coerced — callers that want an integer use
    /// [`evaluate_unsigned`](Self::evaluate_unsigned).
    pub fn evaluate_double(&self) -> Option<f64> {
        unsafe {
            let result = clang_Cursor_Evaluate(self.0);
            if result.is_null() {
                return None;
            }
            let kind = clang_EvalResult_getKind(result);
            let value = match kind {
                CXEval_Float => Some(clang_EvalResult_getAsDouble(result)),
                // A floating-point constant may have an integer initializer
                // (e.g. `const double UIA_ScrollPatternNoScroll = -1;`), which
                // libclang evaluates as an integer; coerce it to the float type.
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

    /// True when this field declaration is a bit-field (e.g. `int x : 4`).
    pub fn is_bit_field(&self) -> bool {
        unsafe { clang_Cursor_isBitField(self.0) != 0 }
    }

    /// True when this struct/union declaration is an *anonymous record member*:
    /// an aggregate declared inside another aggregate with no member declarator,
    /// whose fields are promoted into the parent (the C11 / MSVC anonymous
    /// struct/union idiom, e.g. the nested `union { ... };` inside `PROPVARIANT`).
    /// Such members produce no `FieldDecl`, so they must be reconstructed
    /// explicitly when rebuilding the parent's field list. This holds even when
    /// the aggregate carries a tag name but no instance name (e.g.
    /// `struct tag_inner_PROPVARIANT { ... };` used as a union arm).
    pub fn is_anonymous_record(&self) -> bool {
        unsafe { clang_Cursor_isAnonymousRecordDecl(self.0) != 0 }
    }

    /// The declared width in bits of a bit-field field declaration, or `-1`
    /// when the cursor is not a bit-field.  A zero-width bit-field (`int : 0`)
    /// reports `0` and is used purely to force the next field onto a fresh
    /// storage unit.
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

    /// The type's spelling, e.g. `void (int) __attribute__((noreturn))` for a
    /// `__declspec(noreturn)` function type.
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

    /// The fully-resolved canonical type (all typedefs and elaborations stripped).
    pub fn canonical_type(&self) -> Self {
        Self(unsafe { clang_getCanonicalType(self.0) })
    }

    pub fn is_function_pointer(&self) -> bool {
        self.function_pointee().is_some()
    }

    pub fn function_pointee(&self) -> Option<Self> {
        match self.kind() {
            // A bare function-type typedef (`typedef RET NAME(ARGS)`) carries the same
            // signature as the pointer form and is emitted as the same callback.
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

    /// Returns the alignment of this type in **bytes**, as reported by
    /// `clang_Type_getAlignOf`.  Returns a negative value (one of the
    /// `CXTypeLayoutError_*` constants) when the alignment cannot be
    /// determined (e.g. the type is incomplete or invalid).
    pub fn align_of(&self) -> i64 {
        unsafe { clang_Type_getAlignOf(self.0) }
    }

    /// Returns the size of this type in **bytes**, as reported by
    /// `clang_Type_getSizeOf`.  Returns a negative value (one of the
    /// `CXTypeLayoutError_*` constants) when the size cannot be determined.
    pub fn size_of(&self) -> i64 {
        unsafe { clang_Type_getSizeOf(self.0) }
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
            CXType_Record => {
                let decl = self.ty();
                decl.has_pure_virtual_methods() || decl.has_interface_base()
            }
            CXType_Elaborated => self.underlying_type().is_interface(),
            CXType_Typedef => self.ty().typedef_underlying_type().is_interface(),
            _ => false,
        }
    }

    /// Detects the `DECLARE_HANDLE(X)` idiom: a typedef whose underlying type
    /// is `struct X__ *` (a pointer to a record tagged `<name>__`).
    ///
    /// `DECLARE_HANDLE(HWND)` expands to `typedef struct HWND__ *HWND`, so the
    /// only structural signal is the conventional `<typedef>__` tag name. Such
    /// typedefs carry no payload beyond identity and are emitted as opaque
    /// `*mut void` handles (matching the hand-authored `Foundation::HANDLE`),
    /// rather than as a pointer to a one-off empty tag struct.
    ///
    /// The underlying type is canonicalised first so a SAL-annotated handle
    /// (`typedef __RPC_unique_pointer struct HSTRING__ *HSTRING`, where the SAL
    /// macro expands to an `__attribute__((annotate(…)))` wrapper) is recognised
    /// the same as the bare `DECLARE_HANDLE` form; canonicalisation also strips
    /// the elaboration/typedef sugar on the pointee tag.
    ///
    /// A MIDL file-scope handle placeholder (`typedef struct __MIDL___MIDL_itf_*
    /// { int unused; } *NAME`, [`is_midl_placeholder_tag`]) is recognised the same
    /// way: its opaque tag is dropped and the typedef emits `*mut void`.
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
            // MSVC `long double` is 64-bit (identical to `double`).
            CXType_LongDouble => metadata::Type::F64,
            CXType_WChar => metadata::Type::U16,
            CXType_Char16 => metadata::Type::U16,
            CXType_Char32 => metadata::Type::U32,
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
                // An inline anonymous enum used directly as a field/parameter type
                // (e.g. `enum { ElementType, TextType } Type;` inside `_WSDXML_NODE`)
                // has no name to reference — only a typedef or MIDL tag would have
                // supplied one via the rename map above. Its constants are emitted
                // separately as free constants, so the value itself is faithfully the
                // enum's underlying integer type.
                if self.kind() == CXType_Enum && is_anonymous_name(&name) {
                    return decl.enum_repr().to_type(parser);
                }
                let ns = if parser.header_root.is_some() {
                    // Flat namespace: every in-closure record resolves to the single
                    // root namespace; the defining header only selects the output file.
                    parser.namespace.to_string()
                } else {
                    parser
                        .ref_map
                        .get(&name)
                        .map_or(parser.namespace, |s| s.as_str())
                        .to_string()
                };
                // An incomplete record that is *never* defined anywhere in the
                // translation unit and is only referenced through a pointer (e.g.
                // the CRT's `struct __crt_locale_data`, mentioned solely as
                // `struct __crt_locale_data *locinfo;` with no top-level forward
                // declaration) would otherwise leave a dangling reference. Schedule
                // an opaque struct in the forward declaration's defining-header file
                // so it resolves.
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
            CXType_Typedef => resolve_typedef(self, parser),
            CXType_Pointer => {
                let pointee = self.pointee_type();
                // Function pointers map to opaque *mut u8; they are emitted
                // separately as callbacks via `Callback::parse`.
                if pointee.kind() == CXType_FunctionProto
                    || pointee.kind() == CXType_FunctionNoProto
                {
                    // `clang_getPointeeType` discards the typedef sugar of a pointer to
                    // a function-*type* typedef (`typedef R NAME(args); NAME *field;`),
                    // collapsing the pointee to a bare function proto. The pointer's own
                    // spelling still preserves the name (`"NAME *"`), so recover it and
                    // emit the delegate by name — the same implied-pointer convention as
                    // a function-*pointer* typedef field (`PLSA_INIT p;` -> `p: PLSA_INIT`).
                    if let Some(name) = named_function_typedef_pointer(self) {
                        let ns = parser
                            .ref_map
                            .get(&name)
                            .map_or(parser.namespace, String::as_str);
                        return metadata::Type::value_named(ns, &name);
                    }
                    // Anonymous function pointers map to opaque *mut u8; they are emitted
                    // separately as callbacks via `Callback::parse`.
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
                // A C++ reference (`T&` / `const T&`) has no ABI representation of
                // its own; it is passed exactly like a pointer. `RefConst`/`RefMut`
                // model WinRT logical metadata (`ELEMENT_TYPE_CMOD_REQD IsConst` /
                // `BYREF`), not the C/Rust ABI, so map references to pointers to
                // match the true calling convention (as win32metadata does).
                let pointee = self.pointee_type();
                // Interface types are implied pointers in Windows metadata: `IA&`
                // is written as `IA`, mirroring the `IA*` handling above.
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
                // A flexible array member (`Type field[];`) is an unsized
                // trailing array.  Represent it faithfully as a zero-length
                // fixed array rather than guessing a placeholder length.
                let element = self.array_element_type().to_type(parser);
                metadata::Type::ArrayFixed(Box::new(element), 0)
            }
            // A bare function type reaching a value position (e.g. a typedef alias of
            // another function-type typedef) decays to a function pointer; represent
            // it as an opaque pointer rather than panicking. Function-pointer typedefs
            // proper are handled earlier as callbacks.
            CXType_FunctionProto | CXType_FunctionNoProto => {
                metadata::Type::PtrMut(Box::new(metadata::Type::Void), 1)
            }
            rest => {
                // The `ABI::Windows::*` C++/WinRT projection mirrors a type that already
                // exists in `Windows.winmd`, so map it to that reference rather than
                // failing. Such types only reach here through declarations pulled in by an
                // incidentally-included `winrt/` projection header (e.g. `asyncinfo.h`'s
                // out-of-scope `IAsyncInfo::get_Status(AsyncStatus*)`, dragged in by
                // `UserConsentVerifierInterop.h`); the header sweep drops the referencing
                // declaration afterwards, but emission still has to resolve the type first.
                // `ABI::Windows::Foundation::AsyncStatus` -> `Windows.Foundation.AsyncStatus`.
                let spelling = self.spelling();
                // Match on the canonical spelling: a `using`-aliased reference (as in
                // `asyncinfo.h`) reports the bare name as its spelling but the fully
                // qualified `ABI::Windows::…` path as its canonical type.
                let canonical = self.canonical_type().spelling();
                if let Some(projected) = canonical.strip_prefix("ABI::") {
                    // Drop any generic arguments (`IReference<T>` -> `IReference`); these only
                    // occur on the dropped projection declarations, never on a kept interop API.
                    let projected = projected.split('<').next().unwrap_or(projected);
                    if let Some((namespace, name)) = projected.rsplit_once("::") {
                        return metadata::Type::value_named(&namespace.replace("::", "."), name);
                    }
                }
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

/// Returns `true` if `name` is the spelling of an anonymous struct/union/enum.
///
/// `clang_getCursorSpelling` returns `""` for anonymous types on most libclang
/// versions; some versions return a synthesised string beginning with `"("`
/// (e.g. `"(unnamed struct at f.h:1:1)"`), and others prefix the tag keyword
/// (e.g. `"struct (unnamed at f.h:1:1)"`, as produced for anonymous structs
/// defined inside a macro expansion such as `TYPE_ALIGNMENT`). All such forms
/// are treated as anonymous.
pub fn is_anonymous_name(name: &str) -> bool {
    name.is_empty()
        || name.starts_with('(')
        || name.contains("(unnamed ")
        || name.contains("(anonymous ")
}

/// Returns `true` if `name` is a MIDL-synthesised name for an originally
/// anonymous enumeration.
///
/// When MIDL compiles IDL that contains an anonymous `enum { ... }`, it
/// generates a synthetic tag name of the form `__MIDL___MIDL_itf_<...>`.
/// Such enumerations carry no semantic identity of their own; their variants
/// should be unwrapped and emitted as top-level constants, just as truly
/// unnamed enums (detected by [`is_anonymous_name`]) are.
pub fn is_midl_anonymous_enum_name(name: &str) -> bool {
    name.starts_with("__MIDL_")
}

/// Returns `true` if `name` is a MIDL-synthesised interface-file-scope tag, of the
/// form `__MIDL___MIDL_itf_<...>`. MIDL emits this pseudo-scope for types declared at
/// IDL *file* scope (outside any interface) — including the opaque one-off struct that
/// backs an IDL handle typedef (`typedef [handle] void *NAME` → `struct
/// __MIDL___MIDL_itf_<file>_NNNN { int unused; }; typedef struct … *NAME`). These tags
/// are compiler plumbing and must never be surfaced as a type of their own.
pub fn is_midl_placeholder_tag(name: &str) -> bool {
    name.starts_with("__MIDL___MIDL_itf_")
}

/// Returns `true` if `name` is a MIDL-synthesised *parameter* name, of the form
/// `__MIDL__<Interface>NNNN` (e.g. `__MIDL__IHolder0000`). MIDL generates these for IDL
/// method parameters declared without a name; they carry no meaning and should be treated
/// as unnamed (so the scraper falls back to its positional `param{idx}` naming) rather than
/// surfacing the compiler-internal spelling.
pub fn is_midl_synthetic_param_name(name: &str) -> bool {
    name.starts_with("__MIDL__")
}

/// Returns `true` if the record `decl` carries no payload — it is empty or has a single
/// primitive-`int` field. This is the shape of the dummy tag that backs an opaque
/// handle: `DECLARE_HANDLE`'s `struct X__ { int unused; }` and MIDL's file-scope handle
/// placeholder both use it. A real value struct carries record/typedef/multiple fields.
pub fn is_handle_shape(decl: &Cursor) -> bool {
    let field_kinds: Vec<CXTypeKind> = decl
        .children()
        .into_iter()
        .filter(|c| c.kind() == CXCursor_FieldDecl)
        .map(|c| c.ty().kind())
        .collect();
    field_kinds.is_empty() || (field_kinds.len() == 1 && field_kinds[0] == CXType_Int)
}

/// Recover the typedef name from the spelling of a pointer to a function-*type*
/// typedef. `clang_getPointeeType` collapses such a pointee to a bare function
/// proto (dropping the sugar), but the pointer's spelling still reads `"NAME *"`.
/// Returns `Some("NAME")` only for that exact shape — a single identifier followed
/// by one `*` — so anonymous function pointers (`"int (*)(int)"`) and multi-level
/// pointers fall through to the opaque `*mut u8` mapping.
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
