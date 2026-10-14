#![allow(non_upper_case_globals)]

use std::ffi::{CStr, CString};
use std::sync::Arc;
use windows_clang_sys::*;

#[test]
fn native_semantics() {
    helpers::ensure_libclang();
    let library = Arc::new(load_manually().unwrap());
    let previous = set_library(Some(Arc::clone(&library)));
    for target in [
        "i686-pc-windows-msvc",
        "x86_64-pc-windows-msvc",
        "aarch64-pc-windows-msvc",
    ] {
        let session = Session::new(target);
        let declarations = children(unsafe { clang_getTranslationUnitCursor(session.unit) });
        let find = |name: &str| {
            *declarations
                .iter()
                .find(|cursor| text(unsafe { clang_getCursorSpelling(**cursor) }) == name)
                .unwrap()
        };
        let mut layouts = Vec::new();
        for (name, expected) in [("PackOne", 8), ("PackTwo", 16)] {
            let record = find(name);
            let ty = unsafe { clang_getCursorType(record) };
            layouts.push(unsafe { (clang_Type_getSizeOf(ty), clang_Type_getAlignOf(ty)) });
            let attributes: Vec<_> = children(record)
                .into_iter()
                .filter(|cursor| unsafe { clang_isAttribute(cursor.kind) } != 0)
                .collect();
            assert_eq!(attributes.len(), 1);
            let attribute = attributes[0];
            assert_eq!(attribute.kind, CXCursor_UnexposedAttr);
            assert_eq!(
                native_name(unsafe { windows_clang_getAttributeKindName(attribute) }),
                "MaxFieldAlignment"
            );
            let mut bits = 0;
            assert_eq!(
                unsafe { windows_clang_getMaxFieldAlignment(attribute, &raw mut bits) },
                WindowsClangSuccess
            );
            assert_eq!(bits, expected);
            assert_eq!(
                unsafe { windows_clang_getMaxFieldAlignment(attribute, std::ptr::null_mut()) },
                WindowsClangInvalidArgument
            );
            assert_eq!(
                unsafe { windows_clang_getMaxFieldAlignment(record, &raw mut bits) },
                WindowsClangWrongKind
            );
            assert_eq!(bits, expected);
        }
        assert_eq!(layouts, [(1, 1), (1, 1)]);
        let function =
            |name| unsafe { clang_getPointeeType(clang_getTypedefDeclUnderlyingType(find(name))) };
        for (name, kind) in [("Cdecl", "CDecl"), ("Stdcall", "StdCall")] {
            let ty = function(name);
            assert_eq!(ty.kind, CXType_Attributed);
            assert_eq!(
                native_name(unsafe { windows_clang_getTypeAttributeKindName(ty) }),
                kind
            );
        }
        let plain = function("Plain");
        assert!(unsafe { windows_clang_getTypeAttributeKindName(plain) }.is_null());
        let plain_info = info(plain);
        assert_eq!(
            (
                plain_info.has_regparm,
                plain_info.regparm,
                plain_info.no_return,
                plain_info.additional_properties
            ),
            (0, 0, 0, 0)
        );
        for (name, count) in [("Registers", 2), ("ZeroRegisters", 0)] {
            let ty = function(name);
            assert_eq!(ty.kind, CXType_FunctionProto);
            assert_eq!(unsafe { clang_getFunctionTypeCallingConv(ty) }, unsafe {
                clang_getFunctionTypeCallingConv(plain)
            });
            let value = info(ty);
            if target.starts_with("i686") {
                assert_eq!((value.has_regparm, value.regparm), (1, count));
                assert_eq!(unsafe { clang_equalTypes(ty, plain) }, 0);
            } else {
                assert_eq!((value.has_regparm, value.regparm), (0, 0));
            }
            assert_eq!(value.additional_properties, 0);
        }
        assert_eq!(info(function("PlainVoid")).no_return, 0);
        assert_eq!(info(function("NoReturn")).no_return, 1);
        let qualified = unsafe { clang_getTypedefDeclUnderlyingType(find("QualifiedFunction")) };
        assert_ne!(
            info(qualified).additional_properties & WindowsClangMethodQualifiers as u32,
            0
        );
        let invalid = CXType::default();
        let mut value = WindowsClangFunctionInfo {
            regparm: 99,
            ..Default::default()
        };
        assert_eq!(
            unsafe { windows_clang_getFunctionInfo(invalid, &raw mut value) },
            WindowsClangWrongKind
        );
        assert_eq!(value.regparm, 99);
        for (name, expected) in [
            ("PlainInt", 0),
            ("CVInt", WindowsClangConst | WindowsClangVolatile),
            ("InheritedCVInt", WindowsClangConst | WindowsClangVolatile),
            ("RestrictedPointer", WindowsClangRestrict),
            ("UnalignedInt", WindowsClangUnaligned),
            ("InheritedUnalignedInt", WindowsClangUnaligned),
            ("AddressSpaceInt", WindowsClangOtherQualifiers),
        ] {
            let ty = unsafe { clang_getTypedefDeclUnderlyingType(find(name)) };
            let mut qualifiers = 99;
            assert_eq!(
                unsafe { windows_clang_getTypeQualifiers(ty, &raw mut qualifiers) },
                WindowsClangSuccess
            );
            assert_eq!(qualifiers, expected as u32, "{target}: {name}");
        }
        let mut qualifiers = 99;
        assert_eq!(
            unsafe { windows_clang_getTypeQualifiers(invalid, &raw mut qualifiers) },
            WindowsClangWrongKind
        );
        assert_eq!(qualifiers, 99);
        assert_eq!(
            unsafe { windows_clang_getTypeQualifiers(plain, std::ptr::null_mut()) },
            WindowsClangInvalidArgument
        );
        assert_eq!(
            unsafe { windows_clang_getFunctionInfo(plain, std::ptr::null_mut()) },
            WindowsClangInvalidArgument
        );
    }
    std::thread::scope(|scope| {
        for _ in 0..3 {
            let library = Arc::clone(&library);
            scope.spawn(move || {
                let previous = set_library(Some(library));
                assert_eq!(unsafe { windows_clang_getABIVersion() }, 2);
                assert_eq!(
                    native_name(unsafe { windows_clang_getLLVMVersion() }),
                    LIBCLANG_VERSION
                );
                set_library(previous);
            });
        }
    });
    set_library(previous);
    assert!(load_from(std::path::Path::new("missing-native-clang-directory")).is_err());
}

fn info(ty: CXType) -> WindowsClangFunctionInfo {
    let mut result = WindowsClangFunctionInfo::default();
    assert_eq!(
        unsafe { windows_clang_getFunctionInfo(ty, &raw mut result) },
        WindowsClangSuccess
    );
    result
}

fn native_name(pointer: *const i8) -> String {
    assert!(!pointer.is_null());
    unsafe { CStr::from_ptr(pointer) }.to_str().unwrap().into()
}

fn text(value: CXString) -> String {
    let result = native_name(unsafe { clang_getCString(value) });
    unsafe { clang_disposeString(value) };
    result
}

fn children(cursor: CXCursor) -> Vec<CXCursor> {
    #[derive(Default)]
    struct State {
        values: Vec<CXCursor>,
        panic: Option<Box<dyn std::any::Any + Send>>,
    }
    extern "C" fn visit(cursor: CXCursor, _: CXCursor, data: CXClientData) -> CXChildVisitResult {
        let state = unsafe { &mut *data.cast::<State>() };
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| state.values.push(cursor))) {
            Ok(()) => CXChildVisit_Continue,
            Err(panic) => {
                state.panic = Some(panic);
                CXChildVisit_Break
            }
        }
    }
    let mut state = State::default();
    unsafe { clang_visitChildren(cursor, Some(visit), (&raw mut state).cast()) };
    if let Some(panic) = state.panic {
        std::panic::resume_unwind(panic);
    }
    state.values
}

struct Session {
    index: CXIndex,
    unit: CXTranslationUnit,
}

impl Session {
    fn new(target: &str) -> Self {
        let mut session = Self {
            index: unsafe { clang_createIndex(0, 0) },
            unit: std::ptr::null_mut(),
        };
        assert!(!session.index.is_null());
        let source = include_bytes!("..\\input\\semantics.h");
        let mut file = CXUnsavedFile {
            Filename: c"semantics.h".as_ptr(),
            Contents: source.as_ptr().cast(),
            Length: source.len().try_into().unwrap(),
        };
        let target = CString::new(format!("--target={target}")).unwrap();
        let args = [
            c"-x".as_ptr(),
            c"c++".as_ptr(),
            c"-std=c++20".as_ptr(),
            target.as_ptr(),
        ];
        let status = unsafe {
            clang_parseTranslationUnit2(
                session.index,
                file.Filename,
                args.as_ptr(),
                args.len().try_into().unwrap(),
                &raw mut file,
                1,
                (CXTranslationUnit_IncludeAttributedTypes
                    | CXTranslationUnit_VisitImplicitAttributes)
                    .try_into()
                    .unwrap(),
                &raw mut session.unit,
            )
        };
        assert_eq!(status, CXError_Success);
        assert!(!session.unit.is_null());
        for index in 0..unsafe { clang_getNumDiagnostics(session.unit) } {
            let diagnostic = unsafe { clang_getDiagnostic(session.unit, index) };
            let severity = unsafe { clang_getDiagnosticSeverity(diagnostic) };
            let message = text(unsafe { clang_getDiagnosticSpelling(diagnostic) });
            unsafe { clang_disposeDiagnostic(diagnostic) };
            assert!(severity < CXDiagnostic_Error, "{message}");
        }
        session
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        if !self.unit.is_null() {
            unsafe { clang_disposeTranslationUnit(self.unit) };
        }
        if !self.index.is_null() {
            unsafe { clang_disposeIndex(self.index) };
        }
    }
}
