use super::*;
use clang_sys::*;
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
                CXTranslationUnit_None,
            )
        };

        if tu.is_null() {
            return Err(Error::new("failed to parse", input, 0, 0));
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
                let message = cxstring_to_string(clang_getDiagnosticSpelling(diag));

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
                    cxstring_to_string(fname)
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
}

impl Drop for TranslationUnit {
    fn drop(&mut self) {
        unsafe { clang_disposeTranslationUnit(self.0) };
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

fn cxstring_to_string(cxstr: CXString) -> String {
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
