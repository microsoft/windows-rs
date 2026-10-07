//! Reader for SDK COFF import libraries, which record the DLL exporting each symbol.

use crate::Error;

/// One short-import symbol and its implementing DLL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Import {
    pub symbol: String,
    pub dll: String,
    pub target: ImportTarget,
    pub kind: ImportKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportTarget {
    Name(String),
    Ordinal(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportKind {
    Code,
    Data,
    Const,
}

const ARCHIVE_MAGIC: &[u8] = b"!<arch>\n";
const MEMBER_HEADER_LEN: usize = 60;
/// `IMPORT_OBJECT_HEADER` short-import signature.
const IMPORT_SIGNATURE: &[u8] = &[0x00, 0x00, 0xFF, 0xFF];
const IMPORT_HEADER_LEN: usize = 20;
const SIZE_OF_DATA_OFFSET: usize = 12;

/// Parses every short-import member, preserving archive order and duplicates.
pub fn read(bytes: &[u8]) -> Result<Vec<Import>, Error> {
    if bytes.len() < ARCHIVE_MAGIC.len() || &bytes[..ARCHIVE_MAGIC.len()] != ARCHIVE_MAGIC {
        return Err(err("not a COFF archive (missing `!<arch>` magic)"));
    }

    let mut imports = vec![];
    let mut pos = ARCHIVE_MAGIC.len();

    while pos < bytes.len() {
        if bytes.len() - pos < MEMBER_HEADER_LEN {
            return Err(err("truncated archive member header"));
        }
        let header = &bytes[pos..pos + MEMBER_HEADER_LEN];

        // The end marker guards against a misaligned archive walk.
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

        // Skip archive bookkeeping members.
        if name != b"/" && name != b"//" && data.starts_with(IMPORT_SIGNATURE) {
            imports.push(parse_short_import(data)?);
        }

        pos = data_end + (size & 1);
        if pos > bytes.len() {
            return Err(err("missing archive member padding"));
        }
    }

    Ok(imports)
}

fn parse_short_import(data: &[u8]) -> Result<Import, Error> {
    if data.len() < IMPORT_HEADER_LEN {
        return Err(err("short import member is shorter than its header"));
    }
    let flags = u16::from_le_bytes(data[18..20].try_into().unwrap());
    let kind = match flags & 3 {
        0 => ImportKind::Code,
        1 => ImportKind::Data,
        2 => ImportKind::Const,
        _ => return Err(err("invalid short import type")),
    };
    if flags >> 5 != 0 {
        return Err(err("nonzero reserved short import bits"));
    }
    let size_of_data = u32::from_le_bytes(
        data.get(SIZE_OF_DATA_OFFSET..SIZE_OF_DATA_OFFSET + 4)
            .ok_or_else(|| err("short import member is shorter than its header"))?
            .try_into()
            .unwrap(),
    ) as usize;

    let end = IMPORT_HEADER_LEN
        .checked_add(size_of_data)
        .ok_or_else(|| err("short import size overflow"))?;
    let mut strings = data
        .get(IMPORT_HEADER_LEN..end)
        .ok_or_else(|| err("short import names extend past member data"))?;

    let symbol = next_string(&mut strings, "symbol")?;
    let dll = next_string(&mut strings, "DLL")?;
    let target = match (flags >> 2) & 7 {
        0 => ImportTarget::Ordinal(u16::from_le_bytes(data[16..18].try_into().unwrap())),
        1 => ImportTarget::Name(symbol.clone()),
        2 | 3 => {
            let name = symbol.strip_prefix(['?', '@', '_']).unwrap_or(&symbol);
            let name = if (flags >> 2) & 7 == 3 {
                name.split('@').next().unwrap()
            } else {
                name
            };
            if name.is_empty() {
                return Err(err("empty short import export name"));
            }
            ImportTarget::Name(name.to_string())
        }
        4 => ImportTarget::Name(next_string(&mut strings, "export")?),
        _ => return Err(err("invalid short import name type")),
    };

    Ok(Import {
        symbol,
        dll,
        target,
        kind,
    })
}

fn next_string(parts: &mut &[u8], what: &str) -> Result<String, Error> {
    let end = parts
        .iter()
        .position(|byte| *byte == 0)
        .filter(|end| *end > 0)
        .ok_or_else(|| err(&format!("short import missing terminated {what} name")))?;
    let bytes = &parts[..end];
    *parts = &parts[end + 1..];
    std::str::from_utf8(bytes)
        .map(str::to_string)
        .map_err(|_| err(&format!("short import {what} name is not valid UTF-8")))
}

fn parse_decimal(field: &[u8]) -> Result<usize, Error> {
    let text = std::str::from_utf8(field)
        .map_err(|_| err("archive member size is not valid ASCII"))?
        .trim();
    text.parse::<usize>()
        .map_err(|_| err("archive member has an invalid size field"))
}

fn trim(field: &[u8]) -> &[u8] {
    let end = field.iter().rposition(|&b| b != b' ').map_or(0, |i| i + 1);
    &field[..end]
}

fn err(message: &str) -> Error {
    Error::new(message, "", 0, 0)
}
