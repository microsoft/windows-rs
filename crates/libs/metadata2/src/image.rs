use crate::{Column, Error, TableId};
use std::collections::HashSet;
use std::ops::Range;
use std::path::Path;
use std::sync::Arc;

const DOS_SIGNATURE: u16 = 0x5a4d;
const PE_SIGNATURE: u32 = 0x0000_4550;
const PE32_MAGIC: u16 = 0x010b;
const PE32_PLUS_MAGIC: u16 = 0x020b;
const METADATA_SIGNATURE: u32 = 0x424a_5342;
const CLI_DIRECTORY: usize = 14;
const SECTION_SIZE: usize = 40;

/// The byte range occupied by a metadata stream.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Stream {
    name: String,
    range: Range<usize>,
}

impl Stream {
    /// Returns the stream name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the stream's absolute byte range in the image.
    pub fn range(&self) -> Range<usize> {
        self.range.clone()
    }
}

/// The layout of one ECMA-335 metadata table.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Table {
    offset: usize,
    rows: u32,
    row_size: usize,
}

impl Table {
    /// Returns the absolute offset of the first row.
    pub const fn offset(self) -> usize {
        self.offset
    }

    /// Returns the number of rows.
    pub const fn rows(self) -> u32 {
        self.rows
    }

    /// Returns the encoded size of each row.
    pub const fn row_size(self) -> usize {
        self.row_size
    }
}

/// An owned, structurally validated ECMA-335 metadata image.
#[derive(Clone)]
pub struct Image {
    bytes: Arc<[u8]>,
    streams: Vec<Stream>,
    tables: [Table; TableId::COUNT],
}

impl Image {
    /// Reads and validates a metadata image from disk.
    pub fn read(path: impl AsRef<Path>) -> Result<Self, Error> {
        Self::new(std::fs::read(path)?)
    }

    /// Validates an owned metadata image.
    pub fn new(bytes: impl Into<Arc<[u8]>>) -> Result<Self, Error> {
        let bytes = bytes.into();
        let parsed = Parser::new(&bytes).parse()?;
        Ok(Self {
            bytes,
            streams: parsed.streams,
            tables: parsed.tables,
        })
    }

    /// Returns the complete encoded image.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns all metadata streams in declaration order.
    pub fn streams(&self) -> &[Stream] {
        &self.streams
    }

    /// Returns a metadata stream by name.
    pub fn stream(&self, name: &str) -> Option<&[u8]> {
        self.streams
            .iter()
            .find(|stream| stream.name == name)
            .map(|stream| &self.bytes[stream.range.clone()])
    }

    /// Returns the layout of a metadata table.
    pub const fn table(&self, id: TableId) -> Table {
        self.tables[id as usize]
    }
}

struct Parsed {
    streams: Vec<Stream>,
    tables: [Table; TableId::COUNT],
}

struct Parser<'a> {
    bytes: &'a [u8],
}

impl<'a> Parser<'a> {
    const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    fn parse(&self) -> Result<Parsed, Error> {
        let pe = self.pe_offset()?;
        let coff = self.add(pe, 4)?;
        let section_count = self.u16(self.add(coff, 2)?)? as usize;
        let optional_size = self.u16(self.add(coff, 16)?)? as usize;
        let optional = self.add(coff, 20)?;
        self.slice(optional, optional_size)?;

        let (directories, directory_count) = match self.u16(optional)? {
            PE32_MAGIC => (96, 92),
            PE32_PLUS_MAGIC => (112, 108),
            _ => return Err(Error::invalid(optional, "unsupported PE optional header")),
        };
        if self.u32(self.add(optional, directory_count)?)? as usize <= CLI_DIRECTORY {
            return Err(Error::invalid(optional, "missing CLI data directory"));
        }
        if directories + (CLI_DIRECTORY + 1) * 8 > optional_size {
            return Err(Error::invalid(
                optional,
                "CLI data directory is outside the optional header",
            ));
        }
        let cli_entry = self.add(optional, directories + CLI_DIRECTORY * 8)?;
        let cli_rva = self.u32(cli_entry)?;
        if cli_rva == 0 {
            return Err(Error::invalid(cli_entry, "missing CLI header"));
        }

        let sections_offset = self.add(optional, optional_size)?;
        let sections = self.sections(sections_offset, section_count)?;
        let cli = self.rva_offset(cli_rva, &sections)?;
        let cli_size = self.u32(cli)? as usize;
        if cli_size < 16 {
            return Err(Error::invalid(cli, "CLI header is too small"));
        }
        self.slice(cli, cli_size)?;

        let metadata_rva = self.u32(self.add(cli, 8)?)?;
        let metadata = self.rva_offset(metadata_rva, &sections)?;
        self.parse_metadata(metadata)
    }

    fn pe_offset(&self) -> Result<usize, Error> {
        if self.u16(0)? != DOS_SIGNATURE {
            return Err(Error::invalid(0, "invalid DOS signature"));
        }
        let pe = self.u32(0x3c)? as usize;
        if self.u32(pe)? != PE_SIGNATURE {
            return Err(Error::invalid(pe, "invalid PE signature"));
        }
        Ok(pe)
    }

    fn sections(&self, offset: usize, count: usize) -> Result<Vec<Section>, Error> {
        let length = self.mul(count, SECTION_SIZE, offset)?;
        self.slice(offset, length)?;
        let mut sections = Vec::with_capacity(count);
        for index in 0..count {
            let section = self.add(offset, index * SECTION_SIZE)?;
            sections.push(Section {
                virtual_size: self.u32(self.add(section, 8)?)?,
                virtual_address: self.u32(self.add(section, 12)?)?,
                raw_size: self.u32(self.add(section, 16)?)?,
                raw_offset: self.u32(self.add(section, 20)?)?,
            });
        }
        Ok(sections)
    }

    fn rva_offset(&self, rva: u32, sections: &[Section]) -> Result<usize, Error> {
        for section in sections {
            let size = section.virtual_size.max(section.raw_size);
            let Some(end) = section.virtual_address.checked_add(size) else {
                continue;
            };
            if rva >= section.virtual_address && rva < end {
                let delta = rva - section.virtual_address;
                if delta >= section.raw_size {
                    return Err(Error::invalid(
                        section.raw_offset as usize,
                        "RVA points outside section data",
                    ));
                }
                let offset = section
                    .raw_offset
                    .checked_add(delta)
                    .ok_or_else(|| Error::invalid(0, "RVA offset overflow"))?
                    as usize;
                self.slice(offset, 1)?;
                return Ok(offset);
            }
        }
        Err(Error::invalid(
            rva as usize,
            "RVA is not mapped by a section",
        ))
    }

    fn parse_metadata(&self, metadata: usize) -> Result<Parsed, Error> {
        if self.u32(metadata)? != METADATA_SIGNATURE {
            return Err(Error::invalid(metadata, "invalid metadata signature"));
        }

        let version_len_offset = self.add(metadata, 12)?;
        let version_len = self.u32(version_len_offset)? as usize;
        let version = self.add(metadata, 16)?;
        self.slice(version, version_len)?;
        let flags = self.align4(self.add(version, version_len)?)?;
        let stream_count = self.u16(self.add(flags, 2)?)? as usize;
        let mut cursor = self.add(flags, 4)?;
        let mut streams = Vec::with_capacity(stream_count);
        let mut names = HashSet::new();

        for _ in 0..stream_count {
            let relative_offset = self.u32(cursor)? as usize;
            let size = self.u32(self.add(cursor, 4)?)? as usize;
            let name_offset = self.add(cursor, 8)?;
            let (name, name_len) = self.stream_name(name_offset)?;
            if !names.insert(name.clone()) {
                return Err(Error::DuplicateStream(name));
            }
            let start = self.add(metadata, relative_offset)?;
            let end = self.add(start, size)?;
            self.slice(start, size)?;
            streams.push(Stream {
                name,
                range: start..end,
            });
            cursor = self.align4(self.add(name_offset, name_len + 1)?)?;
        }
        if streams
            .iter()
            .map(|stream| stream.range.start)
            .min()
            .is_some_and(|first_stream| cursor > first_stream)
        {
            return Err(Error::invalid(
                cursor,
                "stream directory overlaps stream data",
            ));
        }

        let table_stream = streams
            .iter()
            .find(|stream| stream.name == "#~")
            .or_else(|| streams.iter().find(|stream| stream.name == "#-"))
            .ok_or(Error::MissingStream("#~"))?;
        let tables = self.parse_tables(table_stream.range.clone())?;
        Ok(Parsed { streams, tables })
    }

    fn stream_name(&self, offset: usize) -> Result<(String, usize), Error> {
        let bytes = self
            .bytes
            .get(offset..)
            .ok_or_else(|| Error::invalid(offset, "stream name is out of bounds"))?;
        let length = bytes
            .iter()
            .position(|byte| *byte == 0)
            .ok_or_else(|| Error::invalid(offset, "unterminated stream name"))?;
        if length == 0 {
            return Err(Error::invalid(offset, "empty stream name"));
        }
        if length >= 32 {
            return Err(Error::invalid(offset, "stream name exceeds 31 bytes"));
        }
        let name = std::str::from_utf8(&bytes[..length])
            .map_err(|_| Error::invalid(offset, "stream name is not UTF-8"))?
            .to_string();
        Ok((name, length))
    }

    fn parse_tables(&self, range: Range<usize>) -> Result<[Table; TableId::COUNT], Error> {
        self.slice(range.start, range.len())?;
        if range.len() < 24 {
            return Err(Error::invalid(
                range.start,
                "table stream header is truncated",
            ));
        }

        let heap_sizes = self.u8(self.add(range.start, 6)?)?;
        let valid = self.u64(self.add(range.start, 8)?)?;
        if valid >> TableId::COUNT != 0 {
            return Err(Error::invalid(
                self.add(range.start, 8)?,
                "unknown metadata table is present",
            ));
        }

        let mut rows = [0u32; TableId::COUNT];
        let mut cursor = self.add(range.start, 24)?;
        for (number, row_count) in rows.iter_mut().enumerate() {
            if valid & (1u64 << number) != 0 {
                if self.add(cursor, 4)? > range.end {
                    return Err(Error::invalid(cursor, "table row counts are truncated"));
                }
                *row_count = self.u32(cursor)?;
                cursor = self.add(cursor, 4)?;
            }
        }

        let widths = Widths { heap_sizes, rows };
        let mut tables = [Table::default(); TableId::COUNT];
        for (number, row_count) in rows.into_iter().enumerate() {
            if valid & (1u64 << number) == 0 {
                continue;
            }
            let id = TableId::from_u8(number as u8).unwrap();
            let row_size = widths.row_size(id);
            let length = self.mul(row_count as usize, row_size, cursor)?;
            let end = self.add(cursor, length)?;
            if end > range.end {
                return Err(Error::invalid(cursor, "metadata table is truncated"));
            }
            tables[number] = Table {
                offset: cursor,
                rows: row_count,
                row_size,
            };
            cursor = end;
        }
        let trailing = &self.bytes[cursor..range.end];
        if trailing.len() > 3 || trailing.iter().any(|byte| *byte != 0) {
            return Err(Error::invalid(
                cursor,
                "table stream has unexpected trailing data",
            ));
        }
        Ok(tables)
    }

    fn u8(&self, offset: usize) -> Result<u8, Error> {
        self.bytes
            .get(offset)
            .copied()
            .ok_or_else(|| Error::invalid(offset, "unexpected end of input"))
    }

    fn u16(&self, offset: usize) -> Result<u16, Error> {
        let bytes = self.slice(offset, 2)?;
        Ok(u16::from_le_bytes(bytes.try_into().unwrap()))
    }

    fn u32(&self, offset: usize) -> Result<u32, Error> {
        let bytes = self.slice(offset, 4)?;
        Ok(u32::from_le_bytes(bytes.try_into().unwrap()))
    }

    fn u64(&self, offset: usize) -> Result<u64, Error> {
        let bytes = self.slice(offset, 8)?;
        Ok(u64::from_le_bytes(bytes.try_into().unwrap()))
    }

    fn slice(&self, offset: usize, length: usize) -> Result<&'a [u8], Error> {
        let end = self.add(offset, length)?;
        self.bytes
            .get(offset..end)
            .ok_or_else(|| Error::invalid(offset, "unexpected end of input"))
    }

    fn add(&self, left: usize, right: usize) -> Result<usize, Error> {
        left.checked_add(right)
            .ok_or_else(|| Error::invalid(left, "byte offset overflow"))
    }

    fn mul(&self, left: usize, right: usize, offset: usize) -> Result<usize, Error> {
        left.checked_mul(right)
            .ok_or_else(|| Error::invalid(offset, "byte length overflow"))
    }

    fn align4(&self, value: usize) -> Result<usize, Error> {
        self.add(value, 3).map(|value| value & !3)
    }
}

#[derive(Clone, Copy)]
struct Section {
    virtual_size: u32,
    virtual_address: u32,
    raw_size: u32,
    raw_offset: u32,
}

struct Widths {
    heap_sizes: u8,
    rows: [u32; TableId::COUNT],
}

impl Widths {
    fn row_size(&self, id: TableId) -> usize {
        id.schema()
            .columns()
            .iter()
            .map(|column| match column {
                Column::U16 => 2,
                Column::U32 => 4,
                Column::String => self.heap(0),
                Column::Guid => self.heap(1),
                Column::Blob => self.heap(2),
                Column::Table(table) => self.table(*table),
                Column::Coded(code) => {
                    let maximum = code
                        .tables()
                        .iter()
                        .map(|table| self.rows[*table as usize])
                        .max()
                        .unwrap_or_default();
                    if maximum < (1u32 << (16 - code.tag_bits())) {
                        2
                    } else {
                        4
                    }
                }
            })
            .sum()
    }

    const fn heap(&self, bit: u8) -> usize {
        if self.heap_sizes & (1 << bit) == 0 {
            2
        } else {
            4
        }
    }

    const fn table(&self, id: TableId) -> usize {
        if self.rows[id as usize] < 1 << 16 {
            2
        } else {
            4
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_committed_windows_metadata() {
        for bytes in [windows_default::WINRT, windows_default::WIN32] {
            let image = Image::new(bytes).unwrap();
            assert!(image.stream("#Strings").is_some());
            assert!(image.stream("#Blob").is_some());
            assert!(image.table(TableId::TypeDef).rows() > 0);
            assert!(image.table(TableId::MethodDef).row_size() > 0);
        }
    }

    #[test]
    fn rejects_truncated_images_without_panicking() {
        let bytes = windows_default::WINRT;
        let image = Image::new(bytes).unwrap();
        let metadata_end = image
            .streams()
            .iter()
            .map(|stream| stream.range().end)
            .max()
            .unwrap();
        for length in [0, 1, 0x3f, 0x40, 0x100, metadata_end / 2, metadata_end - 1] {
            assert!(
                Image::new(&bytes[..length]).is_err(),
                "accepted {length} bytes"
            );
        }
    }

    #[test]
    fn rejects_invalid_dos_signature() {
        let mut bytes = windows_default::WINRT.to_vec();
        bytes[0] = 0;
        assert!(matches!(
            Image::new(bytes),
            Err(Error::Invalid {
                offset: 0,
                message: "invalid DOS signature"
            })
        ));
    }
}
