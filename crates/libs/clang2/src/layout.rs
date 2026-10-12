use super::*;
use clang_sys::{CXCursor, CXIndex};
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
}

type Query = unsafe extern "C" fn(CXCursor, *mut NativeContract) -> u32;
type FieldQuery = unsafe extern "C" fn(CXCursor, *mut u32) -> u32;

pub(super) struct Bridge {
    pub evidence: LayoutBridgeEvidence,
    query: Query,
    field_query: FieldQuery,
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
            if abi != 2 {
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
            Ok(Self {
                evidence: LayoutBridgeEvidence {
                    abi,
                    library: std::fs::canonicalize(bridge).map_err(|err| error(err.to_string()))?,
                    parser_sha256,
                },
                query,
                field_query,
                _library: library,
                _parser: parser_library,
            })
        }
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
}
