use super::*;
use clang_sys::{CXCursor, CXIndex, CXSourceLocation, CXTranslationUnit};
use libloading::Library;
use sha2::{Digest, Sha256};
use std::ffi::{CStr, c_void};
use std::io::Read;
use std::path::Path;

#[repr(C)]
#[derive(Default)]
struct NativeContract {
    packing: u32,
    alignment: u32,
    required_alignment: u32,
    type_visibility: u32,
}

type Query = unsafe extern "C" fn(CXCursor, *mut NativeContract) -> u32;
type FieldQuery = unsafe extern "C" fn(CXCursor, *mut u32) -> u32;

#[repr(C)]
struct NativeInterface {
    size: u32,
    inherited: u32,
    no_vtable: u32,
    packing: u32,
    guid: [u8; 36],
}

type InterfaceQuery = unsafe extern "C" fn(CXCursor, *mut NativeInterface) -> u32;

#[repr(C)]
struct NativeMacro {
    definition: CXSourceLocation,
    function_like: u32,
    variadic: u32,
}

type MacroToken = unsafe extern "C" fn(*mut c_void, u32, *const u8, u32) -> u32;
type MacroQuery = unsafe extern "C" fn(
    CXTranslationUnit,
    CXSourceLocation,
    *const std::ffi::c_char,
    *mut NativeMacro,
    *mut c_void,
    MacroToken,
) -> u32;

pub(super) struct Bridge {
    pub evidence: LayoutBridgeEvidence,
    query: Query,
    field_query: FieldQuery,
    interface_query: InterfaceQuery,
    method_query: FieldQuery,
    helper_query: FieldQuery,
    macro_query: MacroQuery,
    _library: Library,
    _parser: Library,
}

impl Bridge {
    pub fn load(parser: &Path, bridge: &Path) -> Result<Self> {
        let error = |message| Error::new(Stage::Capture, message);
        let parent = |path: &Path| {
            path.parent()
                .ok_or_else(|| error("native library has no parent directory".to_string()))
                .and_then(|path| std::fs::canonicalize(path).map_err(|err| error(err.to_string())))
        };
        if parent(parser)? != parent(bridge)? {
            return Err(error(
                "layout bridge must accompany its matched parser".to_string(),
            ));
        }
        let parser_library =
            unsafe { Library::new(parser) }.map_err(|err| error(err.to_string()))?;
        let library = unsafe { Library::new(bridge) }.map_err(|err| error(err.to_string()))?;
        unsafe {
            let abi = library
                .get::<unsafe extern "C" fn() -> u32>(b"windows_clang_layout_abi\0")
                .map_err(|err| error(err.to_string()))?();
            if abi != 4 {
                return Err(error("layout bridge ABI mismatch".to_string()));
            }
            let expected = library
                .get::<unsafe extern "C" fn() -> *const c_void>(b"windows_clang_layout_parser\0")
                .map_err(|err| error(err.to_string()))?();
            let actual = *parser_library
                .get::<unsafe extern "C" fn(i32, i32) -> CXIndex>(b"clang_createIndex\0")
                .map_err(|err| error(err.to_string()))?;
            if expected != actual as *const () as *const c_void {
                return Err(error(
                    "layout bridge imports a different parser image".to_string(),
                ));
            }
            let fingerprint = library
                .get::<unsafe extern "C" fn() -> *const std::ffi::c_char>(
                    b"windows_clang_layout_fingerprint\0",
                )
                .map_err(|err| error(err.to_string()))?();
            if fingerprint.is_null() {
                return Err(error("layout bridge has no parser fingerprint".to_string()));
            }
            let mut file = std::fs::File::open(parser).map_err(|err| error(err.to_string()))?;
            let mut hash = Sha256::new();
            let mut buffer = [0; 65536];
            loop {
                let count = file
                    .read(&mut buffer)
                    .map_err(|err| error(err.to_string()))?;
                if count == 0 {
                    break;
                }
                hash.update(&buffer[..count]);
            }
            let parser_sha256 = format!("{:x}", hash.finalize());
            if CStr::from_ptr(fingerprint).to_bytes() != parser_sha256.as_bytes() {
                return Err(error(
                    "layout bridge parser build fingerprint mismatch".to_string(),
                ));
            }
            let query = *library
                .get::<Query>(b"windows_clang_layout_record\0")
                .map_err(|err| error(err.to_string()))?;
            let field_query = *library
                .get::<FieldQuery>(b"windows_clang_layout_field\0")
                .map_err(|err| error(err.to_string()))?;
            let interface_query = *library
                .get::<InterfaceQuery>(b"windows_clang_layout_interface\0")
                .map_err(|err| error(err.to_string()))?;
            let method_query = *library
                .get::<FieldQuery>(b"windows_clang_layout_method\0")
                .map_err(|err| error(err.to_string()))?;
            let helper_query = *library
                .get::<FieldQuery>(b"windows_clang_layout_helper\0")
                .map_err(|err| error(err.to_string()))?;
            let macro_query = *library
                .get::<MacroQuery>(b"windows_clang_layout_macro\0")
                .map_err(|err| error(err.to_string()))?;
            Ok(Self {
                evidence: LayoutBridgeEvidence {
                    abi,
                    library: std::fs::canonicalize(bridge).map_err(|err| error(err.to_string()))?,
                    parser_sha256,
                },
                query,
                field_query,
                interface_query,
                method_query,
                helper_query,
                macro_query,
                _library: library,
                _parser: parser_library,
            })
        }
    }

    pub fn macro_definition(
        &self,
        unit: CXTranslationUnit,
        location: CXSourceLocation,
        name: &str,
    ) -> Result<Option<MacroDefinitionEvidence>> {
        struct Tokens {
            parameters: Vec<String>,
            replacement: Vec<String>,
            failed: bool,
        }
        unsafe extern "C" fn emit(
            context: *mut c_void,
            kind: u32,
            data: *const u8,
            size: u32,
        ) -> u32 {
            let tokens = unsafe { &mut *context.cast::<Tokens>() };
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                if data.is_null() || kind > 1 {
                    return false;
                }
                let bytes = unsafe { std::slice::from_raw_parts(data, size as usize) };
                let Ok(text) = std::str::from_utf8(bytes) else {
                    return false;
                };
                if kind == 0 {
                    tokens.parameters.push(text.into());
                } else {
                    tokens.replacement.push(text.into());
                }
                true
            }));
            if !matches!(result, Ok(true)) {
                tokens.failed = true;
                return 1;
            }
            0
        }
        let name_c = std::ffi::CString::new(name)
            .map_err(|_| Error::new(Stage::Capture, "macro name contains NUL"))?;
        let mut native = NativeMacro {
            definition: unsafe { clang_sys::clang_getNullLocation() },
            function_like: 0,
            variadic: 0,
        };
        let mut tokens = Tokens {
            parameters: Vec::new(),
            replacement: Vec::new(),
            failed: false,
        };
        let status = unsafe {
            (self.macro_query)(
                unit,
                location,
                name_c.as_ptr(),
                &raw mut native,
                (&raw mut tokens).cast(),
                emit,
            )
        };
        if status == 1 {
            return Ok(None);
        }
        if status != 0 || tokens.failed || native.function_like > 1 || native.variadic > 1 {
            return Err(Error::new(
                Stage::Capture,
                format!("compiler macro definition rejected with {status}"),
            ));
        }
        let mut file = std::ptr::null_mut();
        let mut line = 0;
        let mut column = 0;
        let mut offset = 0;
        unsafe {
            clang_sys::clang_getSpellingLocation(
                native.definition,
                &raw mut file,
                &raw mut line,
                &raw mut column,
                &raw mut offset,
            );
        }
        let filename = unsafe { clang_sys::clang_getFileName(file) };
        let filename_text = unsafe { clang_sys::clang_getCString(filename) };
        if !file.is_null() && filename_text.is_null() {
            unsafe {
                clang_sys::clang_disposeString(filename);
            }
            return Err(Error::new(
                Stage::Capture,
                "macro definition has no compiler source file",
            ));
        }
        let filename_value = if file.is_null() {
            String::new()
        } else {
            unsafe { CStr::from_ptr(filename_text) }
                .to_string_lossy()
                .into_owned()
        };
        unsafe {
            clang_sys::clang_disposeString(filename);
        }
        Ok(Some(MacroDefinitionEvidence {
            name: name.into(),
            location: Location {
                file: filename_value,
                line,
                column,
                offset,
            },
            function_like: native.function_like != 0,
            variadic: native.variadic != 0,
            parameters: tokens.parameters,
            tokens: tokens.replacement,
        }))
    }

    pub fn record(&self, cursor: CXCursor) -> Result<RecordLayoutContract> {
        let mut native = NativeContract::default();
        let status = unsafe { (self.query)(cursor, &raw mut native) };
        if status != 0 {
            return Err(Error::new(
                Stage::Capture,
                format!("native layout contract rejected with {status}"),
            ));
        }

        let value = |value: u32| {
            if value == 0 {
                return Ok(None);
            }
            if !value.is_power_of_two() {
                return Err(Error::new(
                    Stage::Capture,
                    "invalid native alignment contract",
                ));
            }
            u16::try_from(value).map(Some).map_err(|_| {
                Error::new(Stage::Representation, "native alignment exceeds RDL range")
            })
        };
        let packing = value(native.packing)?;
        if packing.is_some_and(|packing| packing > 16) {
            return Err(Error::new(
                Stage::Representation,
                "unsupported MSVC packing contract",
            ));
        }
        value(native.required_alignment)?;
        Ok(RecordLayoutContract {
            packing,
            alignment: value(native.alignment)?,
            required_alignment: native.required_alignment,
            type_visibility: match native.type_visibility {
                0 => None,
                1 => Some(TypeVisibility::Default),
                2 => Some(TypeVisibility::Hidden),
                3 => Some(TypeVisibility::Protected),
                _ => return Err(Error::new(Stage::Capture, "invalid native type visibility")),
            },
        })
    }

    pub fn anonymous_field(&self, cursor: CXCursor) -> Result<bool> {
        let mut flags = 0;
        let status = unsafe { (self.field_query)(cursor, &raw mut flags) };
        if status != 0 {
            return Err(Error::new(
                Stage::Capture,
                format!("native field identity rejected with {status}"),
            ));
        }
        match flags {
            1 => Ok(false),
            2 => Ok(true),
            _ => Err(Error::new(
                Stage::Capture,
                "unsupported native field identity",
            )),
        }
    }

    pub fn interface(&self, cursor: CXCursor) -> Result<Option<InterfaceEvidence>> {
        let mut native = NativeInterface {
            size: 0,
            inherited: 0,
            no_vtable: 0,
            packing: 0,
            guid: [0; 36],
        };
        let status = unsafe { (self.interface_query)(cursor, &raw mut native) };
        if status == 1 {
            return Ok(None);
        }
        if status != 0 {
            return Err(Error::new(
                Stage::Capture,
                match status {
                    2 => "interface has unsupported storage or inheritance",
                    3 => "interface has an unsupported native attribute",
                    4 => "interface has unsupported method behavior",
                    _ => "interface lacks a supported native vtable or UUID",
                },
            ));
        }
        let guid = std::str::from_utf8(&native.guid)
            .map_err(|_| Error::new(Stage::Capture, "native UUID is not ASCII"))?;
        if !guid.bytes().enumerate().all(|(position, byte)| {
            if [8, 13, 18, 23].contains(&position) {
                byte == b'-'
            } else {
                byte.is_ascii_hexdigit()
            }
        }) || native.inherited > native.size
            || native.no_vtable > 1
            || (native.packing != 0 && (!native.packing.is_power_of_two() || native.packing > 16))
        {
            return Err(Error::new(
                Stage::Capture,
                "invalid native interface descriptor",
            ));
        }
        Ok(Some(InterfaceEvidence {
            guid: guid.to_ascii_lowercase(),
            vtable_size: native.size,
            inherited: native.inherited,
            no_vtable: native.no_vtable != 0,
            packing: if native.packing == 0 {
                None
            } else {
                Some(native.packing as u16)
            },
        }))
    }

    pub fn method_slot(&self, cursor: CXCursor) -> Result<u32> {
        let mut slot = 0;
        let status = unsafe { (self.method_query)(cursor, &raw mut slot) };
        if status != 0 {
            return Err(Error::new(
                Stage::Capture,
                format!("native method slot rejected with {status}"),
            ));
        }
        Ok(slot)
    }

    pub fn check_helper(&self, cursor: CXCursor) -> Result<()> {
        let mut flags = 0;
        let status = unsafe { (self.helper_query)(cursor, &raw mut flags) };
        if status != 0 || flags != 1 {
            return Err(Error::new(
                Stage::Capture,
                "interface helper must be a compiler-confirmed nonvirtual inline method",
            ));
        }
        Ok(())
    }
}
