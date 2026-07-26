//! Reader for SDK COFF import libraries, which record the DLL exporting each symbol.

use crate::Error;

/// One short-import symbol and its implementing DLL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Import {
    pub symbol: String,
    pub dll: String,
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
            imports.push(parse_short_import(data)?);
        }

        pos = data_end + (size & 1);
    }

    Ok(imports)
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
