//! Reader for SDK COFF import libraries, which record the DLL exporting each symbol.

use crate::Error;

/// One short-import symbol and its implementing DLL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Import {
    pub symbol: String,
    pub dll: String,
}

/// A checked short-import contract, including COFF's explicit export-name policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportContract {
    pub symbol: String,
    pub dll: String,
    pub machine: u16,
    pub kind: ImportKind,
    pub target: ImportTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportKind {
    Code,
    Data,
    Constant,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportTarget {
    Name(String),
    Ordinal(u16),
}

const ARCHIVE_MAGIC: &[u8] = b"!<arch>\n";
const MEMBER_HEADER_LEN: usize = 60;
/// `IMPORT_OBJECT_HEADER` short-import signature.
const IMPORT_SIGNATURE: &[u8] = &[0x00, 0x00, 0xFF, 0xFF];
const IMPORT_HEADER_LEN: usize = 20;
const SIZE_OF_DATA_OFFSET: usize = 12;

/// Parses every short-import member, preserving archive order and duplicates.
pub fn read(bytes: &[u8]) -> Result<Vec<Import>, Error> {
    read_with(bytes, parse_short_import, false)
}

/// Parses machine, import kind, and export name/ordinal without guessing from symbol spelling.
pub fn read_contracts(bytes: &[u8]) -> Result<Vec<ImportContract>, Error> {
    read_with(bytes, parse_contract, true)
}

fn read_with<T>(
    bytes: &[u8],
    parse: fn(&[u8]) -> Result<T, Error>,
    strict: bool,
) -> Result<Vec<T>, Error> {
    if bytes.len() < ARCHIVE_MAGIC.len() || &bytes[..ARCHIVE_MAGIC.len()] != ARCHIVE_MAGIC {
        return Err(err("not a COFF archive (missing `!<arch>` magic)"));
    }

    let mut imports = vec![];
    let mut pos = ARCHIVE_MAGIC.len();

    while pos + MEMBER_HEADER_LEN <= bytes.len() {
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
            imports.push(parse(data)?);
        }

        pos = data_end + (size & 1);
    }

    if strict && pos != bytes.len() {
        return Err(err("archive has truncated trailing member data or padding"));
    }
    Ok(imports)
}

fn parse_contract(data: &[u8]) -> Result<ImportContract, Error> {
    let import = parse_short_import(data)?;
    let size = u32::from_le_bytes(data[12..16].try_into().unwrap()) as usize;
    if data[20..20 + size].last() != Some(&0) {
        return Err(err("short import names are not null-terminated"));
    }
    let word = |offset| -> Result<u16, Error> {
        Ok(u16::from_le_bytes(
            data.get(offset..offset + 2)
                .ok_or_else(|| err("short import header is incomplete"))?
                .try_into()
                .unwrap(),
        ))
    };
    if word(4)? > 1 {
        return Err(err("short import version is unsupported"));
    }
    let info = word(18)?;
    if info & !0x1f != 0 {
        return Err(err("short import has nonzero reserved type bits"));
    }
    let kind = match info & 3 {
        0 => ImportKind::Code,
        1 => ImportKind::Data,
        2 => ImportKind::Constant,
        _ => return Err(err("short import kind is invalid")),
    };
    let target = match (info >> 2) & 7 {
        0 => ImportTarget::Ordinal(word(16)?),
        1 => ImportTarget::Name(import.symbol.clone()),
        2 | 3 => {
            let name = import
                .symbol
                .strip_prefix(['?', '@', '_'])
                .unwrap_or(&import.symbol);
            let name = if (info >> 2) & 7 == 3 {
                name.split('@').next().unwrap()
            } else {
                name
            };
            if name.is_empty() {
                return Err(err("short import has an empty transformed export name"));
            }
            ImportTarget::Name(name.to_string())
        }
        4 => {
            let mut names = data[20..20 + size].split(|byte| *byte == 0);
            next_string(&mut names, "symbol")?;
            next_string(&mut names, "DLL")?;
            ImportTarget::Name(next_string(&mut names, "export")?)
        }
        _ => return Err(err("short import name type is invalid")),
    };
    Ok(ImportContract {
        symbol: import.symbol,
        dll: import.dll,
        machine: word(6)?,
        kind,
        target,
    })
}

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
