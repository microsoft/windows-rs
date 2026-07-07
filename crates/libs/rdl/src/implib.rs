//! Reader for COFF *import libraries* — the SDK `.lib` archives such as
//! `kernel32.lib`.
//!
//! A header declares *that* a function exists, but not *which* DLL exports it.
//! That last piece of truth lives in the import libraries the SDK ships
//! alongside the headers: each exported symbol is recorded in a "short import"
//! archive member that names both the symbol and its implementing DLL. Reading
//! it directly is faithful to the SDK and needs no hand-curated mapping.
//!
//! This module only parses the archive; it does not decide *which* libraries to
//! read. A per-DLL library (`kernel32.lib`) maps every symbol to the real DLL
//! (`KERNEL32.dll`), whereas the umbrella/apiset libraries (`onecore.lib`,
//! `mincore.lib`, …) map the same symbols to virtual `api-ms-win-*.dll` apiset
//! names, so callers building a symbol → DLL index should prefer the per-DLL
//! libraries.

use crate::Error;

/// A single exported symbol and the DLL that implements it, as recorded by one
/// short-import member of an import library.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Import {
    /// The exported symbol name (undecorated on 64-bit targets, e.g.
    /// `CreateFileW`).
    pub symbol: String,
    /// The implementing DLL exactly as named in the import library, e.g.
    /// `KERNEL32.dll`.
    pub dll: String,
}

const ARCHIVE_MAGIC: &[u8] = b"!<arch>\n";
const MEMBER_HEADER_LEN: usize = 60;
/// `IMPORT_OBJECT_HEADER`: `Sig1` = `IMAGE_FILE_MACHINE_UNKNOWN` followed by
/// `Sig2` = `0xFFFF`, which together distinguish a short-import member from a
/// regular COFF object.
const IMPORT_SIGNATURE: &[u8] = &[0x00, 0x00, 0xFF, 0xFF];
/// Size of `IMPORT_OBJECT_HEADER`, after which the symbol and DLL name strings
/// begin.
const IMPORT_HEADER_LEN: usize = 20;
/// Byte offset of the `SizeOfData` field within `IMPORT_OBJECT_HEADER`.
const SIZE_OF_DATA_OFFSET: usize = 12;

/// Parse a COFF import-library archive, returning every short-import symbol it
/// declares together with the DLL that exports it.
///
/// The archive's bookkeeping members (the linker symbol indexes named `/` and
/// the long-name table named `//`) and any full COFF objects (the import
/// descriptor, null thunk, …) carry no `symbol → DLL` fact and are skipped.
/// Imports are returned in archive order; duplicates are preserved so the
/// caller can apply its own conflict policy.
pub fn read(bytes: &[u8]) -> Result<Vec<Import>, Error> {
    if bytes.len() < ARCHIVE_MAGIC.len() || &bytes[..ARCHIVE_MAGIC.len()] != ARCHIVE_MAGIC {
        return Err(err("not a COFF archive (missing `!<arch>` magic)"));
    }

    let mut imports = vec![];
    let mut pos = ARCHIVE_MAGIC.len();

    while pos + MEMBER_HEADER_LEN <= bytes.len() {
        let header = &bytes[pos..pos + MEMBER_HEADER_LEN];

        // Member name occupies the first 16 bytes; size is an ASCII decimal in
        // bytes 48..58. The two-byte end marker (`0x60 0x0A`) guards against a
        // misaligned walk.
        let name = trim(&header[0..16]);
        let size = parse_decimal(&header[48..58])?;
        if header[58..60] != [0x60, 0x0A] {
            return Err(err("malformed archive member header (bad end marker)"));
        }

        let data_start = pos + MEMBER_HEADER_LEN;
        let data_end = data_start
            .checked_add(size)
            .filter(|&end| end <= bytes.len())
            .ok_or_else(|| err("archive member extends past end of data"))?;
        let data = &bytes[data_start..data_end];

        // Skip the bookkeeping members; parse every other member that carries
        // the short-import signature.
        if name != b"/" && name != b"//" && data.starts_with(IMPORT_SIGNATURE) {
            imports.push(parse_short_import(data)?);
        }

        // Members are padded to an even byte boundary.
        pos = data_end + (size & 1);
    }

    Ok(imports)
}

/// Parse the symbol and DLL names out of one short-import member's data, which
/// is an `IMPORT_OBJECT_HEADER` followed by the two null-terminated names.
fn parse_short_import(data: &[u8]) -> Result<Import, Error> {
    let size_of_data = u32::from_le_bytes(
        data.get(SIZE_OF_DATA_OFFSET..SIZE_OF_DATA_OFFSET + 4)
            .ok_or_else(|| err("short import member is shorter than its header"))?
            .try_into()
            .unwrap(),
    ) as usize;

    let strings = data
        .get(IMPORT_HEADER_LEN..IMPORT_HEADER_LEN + size_of_data)
        .ok_or_else(|| err("short import names extend past member data"))?;

    let mut parts = strings.split(|&b| b == 0);
    let symbol = next_string(&mut parts, "symbol")?;
    let dll = next_string(&mut parts, "DLL")?;

    Ok(Import { symbol, dll })
}

fn next_string<'a>(
    parts: &mut impl Iterator<Item = &'a [u8]>,
    what: &str,
) -> Result<String, Error> {
    let bytes = parts
        .next()
        .filter(|b| !b.is_empty())
        .ok_or_else(|| err(&format!("short import missing {what} name")))?;
    std::str::from_utf8(bytes)
        .map(str::to_string)
        .map_err(|_| err(&format!("short import {what} name is not valid UTF-8")))
}

/// Parse an ASCII decimal field, ignoring the trailing space padding archives
/// use to right-pad fixed-width header fields.
fn parse_decimal(field: &[u8]) -> Result<usize, Error> {
    let text = std::str::from_utf8(field)
        .map_err(|_| err("archive member size is not valid ASCII"))?
        .trim();
    text.parse::<usize>()
        .map_err(|_| err("archive member has an invalid size field"))
}

/// Trim the trailing-space padding from a fixed-width archive name field.
fn trim(field: &[u8]) -> &[u8] {
    let end = field.iter().rposition(|&b| b != b' ').map_or(0, |i| i + 1);
    &field[..end]
}

fn err(message: &str) -> Error {
    Error::new(message, "", 0, 0)
}
