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
        for (name, expected) in [
            ("NativePointerNull", 0),
            ("NativePointerSentinel", -1),
            (
                "NativePointer32",
                if target.starts_with("i686") {
                    -1
                } else {
                    0xffffffff
                },
            ),
        ] {
            let mut value = 99;
            assert_eq!(
                unsafe { windows_clang_getConstantPointerInitializer(find(name), &raw mut value) },
                WindowsClangSuccess,
                "{name}"
            );
            assert_eq!(value, expected, "{name}");
        }
        for (name, expected) in [
            ("NativePointerRelocation", WindowsClangWrongKind),
            ("NativePointerSideEffect", WindowsClangDependent),
            ("NativePointerUndefined", WindowsClangDependent),
            ("NativeCppValue", WindowsClangWrongKind),
        ] {
            let mut value = 99;
            assert_eq!(
                unsafe { windows_clang_getConstantPointerInitializer(find(name), &raw mut value) },
                expected,
                "{name}"
            );
            assert_eq!(value, 99);
        }
        assert_eq!(
            unsafe {
                windows_clang_getConstantPointerInitializer(
                    find("NativePointerNull"),
                    std::ptr::null_mut(),
                )
            },
            WindowsClangInvalidArgument
        );
        let mut constant = 1;
        assert_eq!(
            unsafe {
                windows_clang_getConstantInitializer(
                    find("NativePointerSentinel"),
                    &raw mut constant,
                )
            },
            WindowsClangSuccess
        );
        assert_eq!(constant, 1);
        let mut pointer = 99;
        assert_eq!(
            unsafe {
                windows_clang_getConstantPointerInitializer(clang_getNullCursor(), &raw mut pointer)
            },
            WindowsClangWrongKind
        );
        assert_eq!(pointer, 99);
        for (name, expected) in [
            ("NativeTypeOfWord", "NativeMacroWord"),
            ("NativeTypeOfText", "const char[7]"),
        ] {
            let mut expression = unsafe { clang_getNullCursor() };
            assert_eq!(
                unsafe { windows_clang_getTypeOfOperand(find(name), &raw mut expression) },
                WindowsClangSuccess
            );
            assert_eq!(
                text(unsafe { clang_getTypeSpelling(clang_getCursorType(expression)) }),
                expected
            );
        }
        let mut size = unsafe { clang_getNullCursor() };
        assert_eq!(
            unsafe { windows_clang_getTypeOfOperand(find("NativeTypeOfSize"), &raw mut size) },
            WindowsClangSuccess
        );
        let size = unsafe { clang_getCursorType(size) };
        let mut underlying = unsafe { clang_getCursorType(find("NativeCppValue")) };
        assert_eq!(
            unsafe { windows_clang_getPredefinedType(size, &raw mut underlying) },
            WindowsClangSuccess
        );
        assert_eq!(
            underlying.kind,
            if target.starts_with("i686") {
                CXType_UInt
            } else {
                CXType_ULongLong
            }
        );
        assert_eq!(
            unsafe { windows_clang_getPredefinedType(size, std::ptr::null_mut()) },
            WindowsClangInvalidArgument
        );
        let alias = unsafe { clang_getCursorType(find("NativeMacroWord")) };
        assert_eq!(alias.kind, CXType_Typedef);
        let sentinel = underlying;
        assert_eq!(
            unsafe { windows_clang_getPredefinedType(alias, &raw mut underlying) },
            WindowsClangWrongKind
        );
        assert_ne!(unsafe { clang_equalTypes(underlying, sentinel) }, 0);
        let mut unchanged = find("NativeCppValue");
        assert_eq!(
            unsafe { windows_clang_getTypeOfOperand(unchanged, &raw mut unchanged) },
            WindowsClangWrongKind
        );
        assert_ne!(
            unsafe { clang_equalCursors(unchanged, find("NativeCppValue")) },
            0
        );
        assert_eq!(
            unsafe {
                windows_clang_getTypeOfOperand(find("NativeTypeOfWord"), std::ptr::null_mut())
            },
            WindowsClangInvalidArgument
        );
        let methods: Vec<_> = children(find("NativeSlots"))
            .into_iter()
            .filter(|cursor| cursor.kind == CXCursor_CXXMethod)
            .collect();
        for (method, expected) in methods
            .iter()
            .zip([Some(3), Some(4), Some(2), Some(1), None])
        {
            let mut slot = 99;
            let status = unsafe { windows_clang_getVirtualMethodSlot(*method, &raw mut slot) };
            assert_eq!(
                status,
                if expected.is_some() {
                    WindowsClangSuccess
                } else {
                    WindowsClangWrongKind
                }
            );
            assert_eq!(slot, expected.unwrap_or(99));
        }
        assert_eq!(methods.len(), 5);
        let method = children(find("NativeSlotVirtual"))
            .into_iter()
            .find(|cursor| cursor.kind == CXCursor_CXXMethod)
            .unwrap();
        let mut slot = 99;
        assert_eq!(
            unsafe { windows_clang_getVirtualMethodSlot(method, &raw mut slot) },
            WindowsClangWrongKind
        );
        assert_eq!(slot, 99);
        assert_eq!(
            unsafe { windows_clang_getVirtualMethodSlot(find("NativeSlots"), &raw mut slot) },
            WindowsClangWrongKind
        );
        assert_eq!(slot, 99);
        assert_eq!(
            unsafe { windows_clang_getVirtualMethodSlot(method, std::ptr::null_mut()) },
            WindowsClangInvalidArgument
        );
        for (name, path, status, expected) in [
            ("NativeGuid", vec![0], WindowsClangSuccess, 0xffffffff),
            ("NativeGuid", vec![1], WindowsClangSuccess, 0x8000),
            ("NativeGuid", vec![2], WindowsClangSuccess, 5),
            ("NativeGuid", vec![3, 0], WindowsClangSuccess, 0xff),
            ("NativeGuid", vec![3, 7], WindowsClangSuccess, 0),
            ("NativeGuid", vec![3, 8], WindowsClangWrongKind, 99),
            ("NativeGuid", vec![4], WindowsClangWrongKind, 99),
            ("NativeGuid", vec![0, 0], WindowsClangWrongKind, 99),
            ("NativeGuid", vec![], WindowsClangWrongKind, 99),
            ("NativeGuidSideEffect", vec![0], WindowsClangDependent, 99),
            ("NativeUninitialized", vec![], WindowsClangWrongKind, 99),
            ("NativeCpp", vec![], WindowsClangWrongKind, 99),
            ("NativeWideString", vec![0], WindowsClangSuccess, 65),
            ("NativeWideString", vec![1], WindowsClangSuccess, 0xd83d),
            ("NativeWideString", vec![2], WindowsClangSuccess, 0xde00),
            ("NativeWideString", vec![3], WindowsClangSuccess, 0),
            ("NativeNegativeByte", vec![0], WindowsClangWrongKind, 99),
        ] {
            let mut value = 99;
            assert_eq!(
                unsafe {
                    windows_clang_getConstantIntegerElement(
                        find(name),
                        path.as_ptr(),
                        path.len() as u32,
                        &raw mut value,
                    )
                },
                status,
                "{name} {path:?}",
            );
            assert_eq!(value, expected, "{name} {path:?}");
        }
        let mut value = 99;
        assert_eq!(
            unsafe {
                windows_clang_getConstantIntegerElement(
                    find("NativeGuid"),
                    std::ptr::null(),
                    1,
                    &raw mut value,
                )
            },
            WindowsClangInvalidArgument,
        );
        assert_eq!(value, 99);
        assert_eq!(
            unsafe {
                windows_clang_getConstantIntegerElement(
                    find("NativeGuid"),
                    std::ptr::null(),
                    0,
                    std::ptr::null_mut(),
                )
            },
            WindowsClangInvalidArgument,
        );
        for (name, expected) in [
            ("NativeDeprecated", ("use \"new\"\nnext", "")),
            ("NativeReplacement", ("old", "NewFunction")),
            ("NativeEmptyDeprecation", ("", "")),
        ] {
            let declaration = find(name);
            let attribute = children(declaration)
                .into_iter()
                .find(|cursor| {
                    (unsafe { clang_isAttribute(cursor.kind) }) != 0
                        && native_name(unsafe { windows_clang_getAttributeKindName(*cursor) })
                            == "Deprecated"
                })
                .unwrap();
            let mut message = CXString::default();
            let mut replacement = CXString::default();
            assert_eq!(
                unsafe {
                    windows_clang_getDeprecation(attribute, &raw mut message, &raw mut replacement)
                },
                WindowsClangSuccess,
            );
            assert_eq!(
                (text(message), text(replacement)),
                (expected.0.into(), expected.1.into())
            );
            assert_eq!(
                unsafe {
                    windows_clang_getDeprecation(
                        declaration,
                        &raw mut message,
                        &raw mut replacement,
                    )
                },
                WindowsClangWrongKind,
            );
            assert_eq!(
                unsafe {
                    windows_clang_getDeprecation(
                        attribute,
                        std::ptr::null_mut(),
                        &raw mut replacement,
                    )
                },
                WindowsClangInvalidArgument,
            );
            assert_eq!(
                unsafe {
                    windows_clang_getDeprecation(attribute, &raw mut message, std::ptr::null_mut())
                },
                WindowsClangInvalidArgument,
            );
        }
        for (parameter, expected) in
            children(find("NativeDefaults"))
                .into_iter()
                .zip([None, Some("3"), Some("nullptr")])
        {
            let mut expression = CXCursor::default();
            let mut spelling = CXString::default();
            assert_eq!(
                unsafe {
                    windows_clang_getParameterDefaultArgument(
                        parameter,
                        &raw mut expression,
                        &raw mut spelling,
                    )
                },
                WindowsClangSuccess,
            );
            assert_eq!(text(spelling), expected.unwrap_or(""));
            assert_eq!(
                unsafe { clang_Cursor_isNull(expression) } != 0,
                expected.is_none()
            );
            if expected.is_some() {
                assert!(
                    children(parameter)
                        .iter()
                        .any(|child| unsafe { clang_equalCursors(*child, expression) } != 0)
                );
            }
            assert_eq!(
                unsafe {
                    windows_clang_getParameterDefaultArgument(
                        parameter,
                        std::ptr::null_mut(),
                        &raw mut spelling,
                    )
                },
                WindowsClangInvalidArgument,
            );
            assert_eq!(
                unsafe {
                    windows_clang_getParameterDefaultArgument(
                        parameter,
                        &raw mut expression,
                        std::ptr::null_mut(),
                    )
                },
                WindowsClangInvalidArgument,
            );
        }
        for (name, expected) in [
            ("QualifiedFunction", WindowsClangConst),
            ("VolatileFunction", WindowsClangVolatile),
            ("CVFunction", WindowsClangConst | WindowsClangVolatile),
            ("RestrictedFunction", WindowsClangRestrict),
        ] {
            let ty = unsafe { clang_getTypedefDeclUnderlyingType(find(name)) };
            let mut value = 99;
            assert_eq!(
                unsafe { windows_clang_getMethodQualifiers(ty, &raw mut value) },
                WindowsClangSuccess
            );
            assert_eq!(value, expected as u32);
            assert_eq!(
                unsafe { windows_clang_getMethodQualifiers(ty, std::ptr::null_mut()) },
                WindowsClangInvalidArgument
            );
        }
        for (cursor, expected) in std::iter::once((find("NativeComputed"), vec![1])).chain(
            children(find("NativeExpressions"))
                .into_iter()
                .map(|field| {
                    let expected = match text(unsafe { clang_getCursorSpelling(field) }).as_str() {
                        "bound" | "bits" | "initialized_bits" => vec![1],
                        "initialized" => vec![0],
                        "initialized_array" => vec![1, 0],
                        name => panic!("unexpected field {name}"),
                    };
                    (field, expected)
                }),
        ) {
            let mut initialized = 99;
            let status = unsafe { windows_clang_getFieldInitializer(cursor, &raw mut initialized) };
            if cursor.kind == CXCursor_FieldDecl {
                assert_eq!(status, WindowsClangSuccess);
                assert_eq!(
                    initialized,
                    u32::from(
                        text(unsafe { clang_getCursorSpelling(cursor) }).starts_with("initialized")
                    )
                );
                assert_eq!(
                    unsafe { windows_clang_getFieldInitializer(cursor, std::ptr::null_mut()) },
                    WindowsClangInvalidArgument
                );
            } else {
                assert_eq!(status, WindowsClangWrongKind);
                assert_eq!(initialized, 99);
            }
            let expressions: Vec<_> = children(cursor)
                .into_iter()
                .filter(|child| unsafe { clang_isExpression(child.kind) } != 0)
                .collect();
            let actual: Vec<_> = expressions
                .iter()
                .map(|&expression| {
                    let mut value = 99;
                    assert_eq!(
                        unsafe {
                            windows_clang_getLayoutExpression(cursor, expression, &raw mut value)
                        },
                        WindowsClangSuccess
                    );
                    assert_eq!(
                        unsafe {
                            windows_clang_getLayoutExpression(
                                cursor,
                                expression,
                                std::ptr::null_mut(),
                            )
                        },
                        WindowsClangInvalidArgument
                    );
                    value
                })
                .collect();
            assert_eq!(
                actual,
                expected,
                "{target}: {}",
                text(unsafe { clang_getCursorSpelling(cursor) })
            );
            let mut value = 99;
            assert_eq!(
                unsafe { windows_clang_getLayoutExpression(cursor, cursor, &raw mut value) },
                WindowsClangWrongKind
            );
            assert_eq!(value, 99);
        }
        let uuid = children(find("NativeUuid"))
            .into_iter()
            .find(|cursor| {
                (unsafe { clang_isAttribute(cursor.kind) }) != 0
                    && native_name(unsafe { windows_clang_getAttributeKindName(*cursor) }) == "Uuid"
            })
            .unwrap();
        let mut value = CXString::default();
        assert_eq!(
            unsafe { windows_clang_getUuid(uuid, &raw mut value) },
            WindowsClangSuccess
        );
        assert_eq!(
            text(value).to_ascii_lowercase(),
            "01234567-89ab-cdef-0123-456789abcdef"
        );
        assert_eq!(
            unsafe { windows_clang_getUuid(uuid, std::ptr::null_mut()) },
            WindowsClangInvalidArgument
        );
        let mut value = CXString::default();
        assert_eq!(
            unsafe { windows_clang_getUuid(find("NativeUuid"), &raw mut value) },
            WindowsClangWrongKind
        );
        let mut linkage_declarations = declarations.clone();
        for &cursor in &declarations {
            if cursor.kind == CXCursor_LinkageSpec {
                linkage_declarations.extend(children(cursor));
            }
        }
        for (name, expected) in [
            ("NativeC", WindowsClangCLanguageLinkage),
            ("NativeCpp", WindowsClangCXXLanguageLinkage),
            ("NativeCValue", WindowsClangCLanguageLinkage),
            ("NativeCppValue", WindowsClangNoLanguageLinkage),
        ] {
            let cursor = *linkage_declarations
                .iter()
                .find(|cursor| text(unsafe { clang_getCursorSpelling(**cursor) }) == name)
                .unwrap();
            let mut value = WindowsClangNoLanguageLinkage;
            assert_eq!(
                unsafe { windows_clang_getLanguageLinkage(cursor, &raw mut value) },
                WindowsClangSuccess
            );
            assert_eq!(value, expected);
            assert_eq!(
                unsafe { windows_clang_getLanguageLinkage(cursor, std::ptr::null_mut()) },
                WindowsClangInvalidArgument
            );
        }
        let mut value = 99;
        assert_eq!(
            unsafe { windows_clang_getLanguageLinkage(find("NativeUuid"), &raw mut value) },
            WindowsClangWrongKind
        );
        assert_eq!(value, 99);
        for (name, expected) in [("NativeCppValue", 1), ("NativeSideEffect", 0)] {
            let cursor = find(name);
            let mut value = 99;
            assert_eq!(
                unsafe { windows_clang_getIntegerConstantInitializer(cursor, &raw mut value) },
                WindowsClangSuccess
            );
            assert_eq!(value, expected);
            assert_eq!(
                unsafe {
                    windows_clang_getIntegerConstantInitializer(cursor, std::ptr::null_mut())
                },
                WindowsClangInvalidArgument
            );
        }
        let mut value = 99;
        assert_eq!(
            unsafe {
                windows_clang_getIntegerConstantInitializer(find("NativeCpp"), &raw mut value)
            },
            WindowsClangWrongKind
        );
        assert_eq!(value, 99);
        let folded = unsafe { clang_Cursor_Evaluate(find("NativeSideEffect")) };
        assert!(!folded.is_null());
        assert_eq!(unsafe { clang_EvalResult_getAsInt(folded) }, 3);
        unsafe { clang_EvalResult_dispose(folded) };
        for (name, expected) in [
            ("NativeCppValue", 1),
            ("NativeFloat", 1),
            ("NativeDouble", 1),
            ("NativeBool", 1),
            ("NativeSideEffect", 0),
            ("NativeFloatSideEffect", 0),
            ("NativeUninitialized", 0),
            ("NativeMemberEnd", 1),
            ("NativeMemberOffset", 1),
            ("NativeSignedOverflow", 0),
            ("NativeDivisionByZero", 0),
            ("NativeNullRead", 0),
        ] {
            let cursor = find(name);
            let mut value = 99;
            assert_eq!(
                unsafe { windows_clang_getConstantInitializer(cursor, &raw mut value) },
                WindowsClangSuccess
            );
            assert_eq!(value, expected);
            assert_eq!(
                unsafe { windows_clang_getConstantInitializer(cursor, std::ptr::null_mut()) },
                WindowsClangInvalidArgument
            );
        }
        let mut value = 99;
        assert_eq!(
            unsafe { windows_clang_getConstantInitializer(find("NativeCpp"), &raw mut value) },
            WindowsClangWrongKind
        );
        assert_eq!(value, 99);
        let mut layouts = Vec::new();
        for (name, expected) in [
            ("NativeOwned", 1),
            ("NativeBuiltinArguments", 0),
            ("NativeAlias", 0),
            ("NativeTagAlias", 0),
            ("NativePointer", 0),
            ("NativeUsing", 0),
            ("NativeForwardAlias", 0),
            ("NativeOne", 1),
            ("NativeTwo", 1),
            ("NativeMultiPointer", 0),
            ("NativeArray", 0),
            ("NativeEnumAlias", 1),
            ("NativeAnonymous", 1),
            ("MacroOwned", 1),
        ] {
            let cursor = *declarations
                .iter()
                .find(|cursor| {
                    matches!(cursor.kind, CXCursor_TypedefDecl | CXCursor_TypeAliasDecl)
                        && text(unsafe { clang_getCursorSpelling(**cursor) }) == name
                })
                .unwrap();
            let mut value = 99;
            assert_eq!(
                unsafe { windows_clang_getTypedefDefinition(cursor, &raw mut value) },
                WindowsClangSuccess,
                "{target}: {name}, kind {}",
                cursor.kind
            );
            assert_eq!(value, expected, "{target}: {name}");
            assert_eq!(
                unsafe { windows_clang_getTypedefImplicit(cursor, &raw mut value) },
                WindowsClangSuccess
            );
            assert_eq!(value, 0, "{target}: {name}");
            assert_eq!(
                unsafe { windows_clang_getTypedefDefinition(cursor, std::ptr::null_mut()) },
                WindowsClangInvalidArgument
            );
        }
        let mut value = 99;
        assert_eq!(
            unsafe { windows_clang_getTypedefDefinition(find("NativeTag"), &raw mut value) },
            WindowsClangWrongKind
        );
        assert_eq!(value, 99);
        let builtin = unsafe {
            clang_getTypeDeclaration(clang_getTypedefDeclUnderlyingType(find(
                "NativeBuiltinArguments",
            )))
        };
        assert_eq!(
            unsafe { windows_clang_getTypedefImplicit(builtin, &raw mut value) },
            WindowsClangSuccess
        );
        assert_eq!(value, 1);
        assert_eq!(
            unsafe { windows_clang_getTypedefImplicit(builtin, std::ptr::null_mut()) },
            WindowsClangInvalidArgument
        );
        value = 99;
        assert_eq!(
            unsafe { windows_clang_getTypedefImplicit(find("NativeTag"), &raw mut value) },
            WindowsClangWrongKind
        );
        assert_eq!(value, 99);
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
        for (name, expected) in [
            ("Visible", CXVisibility_Default),
            ("Hidden", CXVisibility_Hidden),
            ("Protected", CXVisibility_Protected),
            ("TypeVisible", CXVisibility_Default),
            ("_GUID", CXVisibility_Default),
        ] {
            let record = find(name);
            let attribute = children(record)
                .into_iter()
                .find(|cursor| {
                    (unsafe { clang_isAttribute(cursor.kind) }) != 0
                        && native_name(unsafe { windows_clang_getAttributeKindName(*cursor) })
                            == "TypeVisibility"
                })
                .unwrap();
            let mut value = CXVisibility_Invalid;
            assert_eq!(
                unsafe { windows_clang_getTypeVisibility(attribute, &raw mut value) },
                WindowsClangSuccess
            );
            assert_eq!(value, expected, "{target}: {name}");
            assert_eq!(
                unsafe { windows_clang_getTypeVisibility(record, &raw mut value) },
                WindowsClangWrongKind
            );
            assert_eq!(value, expected);
            assert_eq!(
                unsafe { windows_clang_getTypeVisibility(attribute, std::ptr::null_mut()) },
                WindowsClangInvalidArgument
            );
        }
        let symbol = children(find("SymbolHidden"))
            .into_iter()
            .find(|cursor| unsafe { clang_isAttribute(cursor.kind) } != 0)
            .unwrap();
        let mut visibility = CXVisibility_Invalid;
        assert_eq!(
            unsafe { windows_clang_getTypeVisibility(symbol, &raw mut visibility) },
            WindowsClangWrongKind
        );
        assert_eq!(visibility, CXVisibility_Invalid);
        for (name, bits, kind) in [
            ("AlignFour", 32, WindowsClangAlignmentDeclspec),
            ("AlignEight", 64, WindowsClangAlignmentDeclspec),
            ("AlignZero", 0, WindowsClangAlignmentStandard),
            ("AlignStandard", 128, WindowsClangAlignmentStandard),
            ("AlignType", 128, WindowsClangAlignmentStandard),
            ("AlignGNU", 256, WindowsClangAlignmentGNU),
            ("AlignDefault", 128, WindowsClangAlignmentGNU),
        ] {
            let record = find(name);
            let attribute = children(record)
                .into_iter()
                .find(|cursor| unsafe { clang_isAttribute(cursor.kind) } != 0)
                .unwrap();
            let mut value = WindowsClangAlignmentInfo::default();
            assert_eq!(
                unsafe { windows_clang_getAlignment(attribute, &raw mut value) },
                WindowsClangSuccess
            );
            assert_eq!((value.bits, value.kind), (bits, kind), "{target}: {name}");
            assert_eq!(
                unsafe { windows_clang_getAlignment(record, &raw mut value) },
                WindowsClangWrongKind
            );
            assert_eq!((value.bits, value.kind), (bits, kind));
            assert_eq!(
                unsafe { windows_clang_getAlignment(attribute, std::ptr::null_mut()) },
                WindowsClangInvalidArgument
            );
        }
        let layout = |name| {
            let ty = unsafe { clang_getCursorType(find(name)) };
            unsafe { (clang_Type_getSizeOf(ty), clang_Type_getAlignOf(ty)) }
        };
        assert_eq!(layout("AlignFour"), layout("AlignEight"));
        let dependent = children(find("AlignDependent"))
            .into_iter()
            .find(|cursor| unsafe { clang_isAttribute(cursor.kind) } != 0)
            .unwrap();
        let mut value = WindowsClangAlignmentInfo {
            bits: 99,
            ..Default::default()
        };
        assert_eq!(
            unsafe { windows_clang_getAlignment(dependent, &raw mut value) },
            WindowsClangDependent
        );
        assert_eq!(value.bits, 99);
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
            ("UnalignedByteRecord", WindowsClangUnaligned),
            ("InheritedUnalignedByteRecord", WindowsClangUnaligned),
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
        for (name, alignment) in [
            ("UnalignedInt", 4),
            ("InheritedUnalignedInt", 4),
            ("UnalignedByteRecord", 1),
            ("InheritedUnalignedByteRecord", 1),
        ] {
            let ty = unsafe {
                clang_getUnqualifiedType(clang_getCanonicalType(
                    clang_getTypedefDeclUnderlyingType(find(name)),
                ))
            };
            let mut qualifiers = 99;
            assert_eq!(
                unsafe { windows_clang_getTypeQualifiers(ty, &raw mut qualifiers) },
                WindowsClangSuccess
            );
            assert_eq!(qualifiers, 0, "{target}: {name}");
            assert_eq!(
                unsafe { clang_Type_getAlignOf(ty) },
                alignment,
                "{target}: {name}"
            );
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
                assert_eq!(unsafe { windows_clang_getABIVersion() }, 16);
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
