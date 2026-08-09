use super::*;
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
const MAX_COLUMNS: usize = 9;

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
pub struct TableLayout {
    offset: usize,
    rows: u32,
    row_size: usize,
    columns: ColumnLayout,
}

impl TableLayout {
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
    tables: [TableLayout; TableId::COUNT],
    sorted: u64,
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
        let mut image = Self {
            bytes,
            streams: parsed.streams,
            tables: parsed.tables,
            sorted: 0,
        };
        image.validate_columns()?;
        image.validate_signatures()?;
        image.sorted = image.validated_sorted_tables(parsed.sorted)?;
        Ok(image)
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
    pub const fn table(&self, id: TableId) -> TableLayout {
        self.tables[id as usize]
    }

    /// Returns a typed identifier for a one-based row number.
    pub fn row<T: Table>(&self, number: u32) -> Option<RowId<T>> {
        if number <= self.table(T::ID).rows {
            RowId::new(number)
        } else {
            None
        }
    }

    /// Iterates every row in a typed metadata table.
    pub fn rows<T: Table>(&self) -> Rows<T> {
        Rows::new(self.table(T::ID).rows)
    }

    /// Returns rows whose encoded column equals `value`.
    pub fn matching_rows<T: Table>(
        &self,
        column: usize,
        value: u32,
    ) -> Result<RowMatches<T>, Error> {
        if T::ID.schema().columns().get(column).is_none() {
            return Err(Error::invalid(
                self.table(T::ID).offset,
                "column is out of bounds",
            ));
        }
        if T::ID.schema().sorted_column() == Some(column)
            && self.sorted & (1u64 << T::ID.as_u8()) != 0
        {
            let start = self.lower_bound::<T>(column, value, false)?;
            let end = self.lower_bound::<T>(column, value, true)?;
            return Ok(RowMatches::Range(Rows::range(start, end)));
        }

        let mut rows = Vec::new();
        for row in self.rows::<T>() {
            if self.column_data(row, column)?.0 == value {
                rows.push(row);
            }
        }
        Ok(RowMatches::Sparse(rows.into_iter()))
    }

    /// Resolves a typed row identity against this image.
    pub fn view<T: Table>(&self, id: RowId<T>) -> Option<Row<'_, T>> {
        (id.number() <= self.table(T::ID).rows).then(|| Row::new(self, id))
    }

    /// Iterates the half-open range between two list-start indexes.
    pub fn list_range<T: Table>(&self, start: ListIndex<T>, end: ListIndex<T>) -> Option<Rows<T>> {
        let limit = self.table(T::ID).rows.checked_add(1)?;
        (start.number() <= end.number() && end.number() <= limit)
            .then(|| Rows::range(start.number(), end.number()))
    }

    /// Reads a null-terminated UTF-8 string from the `#Strings` heap.
    pub fn string(&self, id: StringId) -> Result<&str, Error> {
        let stream = self.heap("#Strings")?;
        let offset = id.value() as usize;
        let absolute = self.heap_offset("#Strings", offset)?;
        let bytes = stream
            .get(offset..)
            .ok_or_else(|| Error::invalid(absolute, "string index is out of bounds"))?;
        let length = bytes
            .iter()
            .position(|byte| *byte == 0)
            .ok_or_else(|| Error::invalid(absolute, "unterminated string"))?;
        std::str::from_utf8(&bytes[..length])
            .map_err(|_| Error::invalid(absolute, "string is not UTF-8"))
    }

    /// Reads a length-prefixed value from the `#Blob` heap.
    pub fn blob(&self, id: BlobId) -> Result<&[u8], Error> {
        self.blob_data(id).map(|(bytes, _)| bytes)
    }

    /// Opens a checked cursor over a value in the `#Blob` heap.
    pub fn blob_reader(&self, id: BlobId) -> Result<BlobReader<'_>, Error> {
        let (bytes, offset) = self.blob_data(id)?;
        Ok(BlobReader::new(bytes, offset))
    }

    fn blob_data(&self, id: BlobId) -> Result<(&[u8], usize), Error> {
        let stream = self.heap("#Blob")?;
        let offset = id.value() as usize;
        let absolute = self.heap_offset("#Blob", offset)?;
        let bytes = stream
            .get(offset..)
            .ok_or_else(|| Error::invalid(absolute, "blob index is out of bounds"))?;
        let (length, prefix) =
            compressed_u32(bytes).ok_or_else(|| Error::invalid(absolute, "invalid blob length"))?;
        let start = prefix;
        let end = start
            .checked_add(length as usize)
            .ok_or_else(|| Error::invalid(absolute, "blob length overflow"))?;
        let value = bytes
            .get(start..end)
            .ok_or_else(|| Error::invalid(absolute, "blob is truncated"))?;
        Ok((value, absolute + prefix))
    }

    /// Reads a GUID from the `#GUID` heap.
    pub fn guid(&self, id: GuidId) -> Result<Option<[u8; 16]>, Error> {
        if id.value() == 0 {
            return Ok(None);
        }
        let stream = self.heap("#GUID")?;
        let offset = (id.value() as usize - 1)
            .checked_mul(16)
            .ok_or_else(|| Error::invalid(0, "GUID index overflow"))?;
        let end = offset
            .checked_add(16)
            .ok_or_else(|| Error::invalid(0, "GUID index overflow"))?;
        let absolute = self.heap_offset("#GUID", offset)?;
        let bytes = stream
            .get(offset..end)
            .ok_or_else(|| Error::invalid(absolute, "GUID index is out of bounds"))?;
        Ok(Some(bytes.try_into().unwrap()))
    }

    fn raw_column_data(
        &self,
        id: TableId,
        row: u32,
        column: usize,
    ) -> Result<(u32, Column, usize), Error> {
        let layout = self.table(id);
        if row == 0 || row > layout.rows {
            return Err(Error::invalid(layout.offset, "row is out of bounds"));
        }
        let Some(kind) = id.schema().columns().get(column).copied() else {
            return Err(Error::invalid(layout.offset, "column is out of bounds"));
        };
        let Some((offset, width)) = layout.column(column) else {
            return Err(Error::invalid(layout.offset, "column is out of bounds"));
        };
        let row_offset = layout.offset + (row as usize - 1) * layout.row_size + offset;
        let value = match width {
            2 => u16::from_le_bytes(self.bytes[row_offset..row_offset + 2].try_into().unwrap())
                as u32,
            4 => u32::from_le_bytes(self.bytes[row_offset..row_offset + 4].try_into().unwrap()),
            _ => unreachable!(),
        };
        Ok((value, kind, row_offset))
    }

    pub(crate) fn column_data<T: Table>(
        &self,
        row: RowId<T>,
        column: usize,
    ) -> Result<(u32, Column, usize), Error> {
        self.raw_column_data(T::ID, row.number(), column)
    }

    pub(crate) fn decode_coded(
        &self,
        code: CodedIndex,
        value: u32,
        offset: usize,
    ) -> Result<Option<AnyRowId>, Error> {
        if value == 0 {
            return Ok(None);
        }
        let tag_mask = (1 << code.tag_bits()) - 1;
        let tag = value & tag_mask;
        let row = value >> code.tag_bits();
        let table = code
            .target(tag)
            .ok_or_else(|| Error::invalid(offset, "coded index has an invalid tag"))?;
        if row == 0 || row > self.table(table).rows {
            return Err(Error::invalid(offset, "coded index row is out of bounds"));
        }
        Ok(Some(AnyRowId::new(table, row).unwrap()))
    }

    fn validate_columns(&self) -> Result<(), Error> {
        if let Some(strings) = self.stream("#Strings")
            && strings.first() != Some(&0)
        {
            return Err(Error::invalid(
                self.heap_offset("#Strings", 0)?,
                "string heap does not begin with an empty string",
            ));
        }
        if let Some(blobs) = self.stream("#Blob")
            && blobs.first() != Some(&0)
        {
            return Err(Error::invalid(
                self.heap_offset("#Blob", 0)?,
                "blob heap does not begin with an empty blob",
            ));
        }
        if let Some(guids) = self.stream("#GUID")
            && guids.len() % 16 != 0
        {
            return Err(Error::invalid(
                self.heap_offset("#GUID", guids.len() / 16 * 16)?,
                "GUID heap has a partial entry",
            ));
        }

        for schema in &TABLES {
            let layout = self.table(schema.id());
            for row in 1..=layout.rows {
                for (column, kind) in schema.columns().iter().enumerate() {
                    let (value, _, offset) = self.raw_column_data(schema.id(), row, column)?;
                    match kind {
                        Column::String => {
                            self.string(StringId::new(value))?;
                        }
                        Column::Blob => {
                            self.blob(BlobId::new(value))?;
                        }
                        Column::Guid => {
                            self.guid(GuidId::new(value))?;
                        }
                        Column::Table(table) => {
                            if value > self.table(*table).rows {
                                return Err(Error::invalid(offset, "table index is out of bounds"));
                            }
                        }
                        Column::List(table) => {
                            let limit = self.table(*table).rows.checked_add(1);
                            if value == 0 || limit.is_none_or(|limit| value > limit) {
                                return Err(Error::invalid(offset, "list index is out of bounds"));
                            }
                        }
                        Column::Coded(code) => {
                            self.decode_coded(*code, value, offset)?;
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    #[cfg(test)]
    fn column<T: Table>(&self, row: RowId<T>, column: usize) -> Result<u32, Error> {
        self.column_data(row, column).map(|(value, _, _)| value)
    }

    fn heap(&self, name: &'static str) -> Result<&[u8], Error> {
        self.stream(name).ok_or(Error::MissingStream(name))
    }

    fn heap_offset(&self, name: &'static str, offset: usize) -> Result<usize, Error> {
        let stream = self
            .streams
            .iter()
            .find(|stream| stream.name == name)
            .ok_or(Error::MissingStream(name))?;
        stream
            .range
            .start
            .checked_add(offset)
            .ok_or_else(|| Error::invalid(stream.range.start, "heap offset overflow"))
    }

    fn lower_bound<T: Table>(
        &self,
        column: usize,
        value: u32,
        after_equal: bool,
    ) -> Result<u32, Error> {
        let mut low = 1;
        let mut high = self
            .table(T::ID)
            .rows
            .checked_add(1)
            .ok_or_else(|| Error::invalid(self.table(T::ID).offset, "row range overflow"))?;
        while low < high {
            let middle = low + (high - low) / 2;
            let row = RowId::<T>::new(middle).unwrap();
            let current = self.column_data(row, column)?.0;
            if current < value || (after_equal && current == value) {
                low = middle + 1;
            } else {
                high = middle;
            }
        }
        Ok(low)
    }

    fn validated_sorted_tables(&self, declared: u64) -> Result<u64, Error> {
        let mut sorted = 0;
        for schema in TABLES
            .iter()
            .filter(|schema| schema.sorted_column().is_some())
        {
            let column = schema.sorted_column().unwrap();
            let mut previous = 0;
            let mut ordered = true;
            for row in 1..=self.table(schema.id()).rows {
                let (value, _, offset) = self.raw_column_data(schema.id(), row, column)?;
                if value < previous {
                    ordered = false;
                    if declared & (1u64 << schema.id().as_u8()) != 0 {
                        return Err(Error::invalid(offset, "table violates declared sort order"));
                    }
                    break;
                }
                previous = value;
            }
            if ordered {
                sorted |= 1u64 << schema.id().as_u8();
            }
        }
        Ok(sorted)
    }
}

struct Parsed {
    streams: Vec<Stream>,
    tables: [TableLayout; TableId::COUNT],
    sorted: u64,
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
        if directory_count + 4 > optional_size {
            return Err(Error::invalid(
                optional,
                "data directory count is outside the optional header",
            ));
        }
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
        let cli_directory_size = self.u32(self.add(cli_entry, 4)?)? as usize;
        if cli_rva == 0 {
            return Err(Error::invalid(cli_entry, "missing CLI header"));
        }
        if cli_directory_size < 16 {
            return Err(Error::invalid(cli_entry, "CLI data directory is too small"));
        }

        let sections_offset = self.add(optional, optional_size)?;
        let sections = self.sections(sections_offset, section_count)?;
        let cli = self.rva_offset(cli_rva, 4, &sections)?;
        let cli_size = self.u32(cli)? as usize;
        if cli_size < 16 || cli_size > cli_directory_size {
            return Err(Error::invalid(cli, "invalid CLI header size"));
        }
        let cli = self.rva_offset(cli_rva, cli_size, &sections)?;

        let metadata_rva = self.u32(self.add(cli, 8)?)?;
        let metadata_size = self.u32(self.add(cli, 12)?)? as usize;
        if metadata_size < 20 {
            return Err(Error::invalid(cli, "metadata directory is too small"));
        }
        let metadata = self.rva_offset(metadata_rva, metadata_size, &sections)?;
        self.parse_metadata(metadata, metadata_size)
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

    fn rva_offset(&self, rva: u32, length: usize, sections: &[Section]) -> Result<usize, Error> {
        for section in sections {
            let size = section.virtual_size.max(section.raw_size);
            let Some(end) = section.virtual_address.checked_add(size) else {
                continue;
            };
            if rva >= section.virtual_address && rva < end {
                let delta = rva - section.virtual_address;
                if (delta as usize)
                    .checked_add(length)
                    .is_none_or(|end| end > section.raw_size as usize)
                {
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
                self.slice(offset, length)?;
                return Ok(offset);
            }
        }
        Err(Error::invalid(
            rva as usize,
            "RVA is not mapped by a section",
        ))
    }

    fn parse_metadata(&self, metadata: usize, size: usize) -> Result<Parsed, Error> {
        let metadata_end = self.add(metadata, size)?;
        if self.u32(metadata)? != METADATA_SIGNATURE {
            return Err(Error::invalid(metadata, "invalid metadata signature"));
        }

        let version_len_offset = self.add(metadata, 12)?;
        let version_len = self.u32(version_len_offset)? as usize;
        let version = self.add(metadata, 16)?;
        if self.add(version, version_len)? > metadata_end {
            return Err(Error::invalid(version, "metadata version is truncated"));
        }
        let flags = self.align4(self.add(version, version_len)?)?;
        if self.add(flags, 4)? > metadata_end {
            return Err(Error::invalid(flags, "metadata stream count is truncated"));
        }
        let stream_count = self.u16(self.add(flags, 2)?)? as usize;
        let mut cursor = self.add(flags, 4)?;
        let mut streams = Vec::with_capacity(stream_count);
        let mut names = HashSet::new();

        for _ in 0..stream_count {
            if self.add(cursor, 8)? > metadata_end {
                return Err(Error::invalid(cursor, "stream directory is truncated"));
            }
            let relative_offset = self.u32(cursor)? as usize;
            let size = self.u32(self.add(cursor, 4)?)? as usize;
            let name_offset = self.add(cursor, 8)?;
            let (name, name_len) = self.stream_name(name_offset, metadata_end)?;
            if !names.insert(name.clone()) {
                return Err(Error::DuplicateStream(name));
            }
            let start = self.add(metadata, relative_offset)?;
            let end = self.add(start, size)?;
            if end > metadata_end {
                return Err(Error::invalid(start, "metadata stream is out of bounds"));
            }
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
        let (tables, sorted) = self.parse_tables(table_stream.range.clone())?;
        Ok(Parsed {
            streams,
            tables,
            sorted,
        })
    }

    fn stream_name(&self, offset: usize, limit: usize) -> Result<(String, usize), Error> {
        let bytes = self
            .bytes
            .get(offset..limit)
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

    fn parse_tables(
        &self,
        range: Range<usize>,
    ) -> Result<([TableLayout; TableId::COUNT], u64), Error> {
        self.slice(range.start, range.len())?;
        if range.len() < 24 {
            return Err(Error::invalid(
                range.start,
                "table stream header is truncated",
            ));
        }

        let heap_sizes = self.u8(self.add(range.start, 6)?)?;
        let valid = self.u64(self.add(range.start, 8)?)?;
        let sorted = self.u64(self.add(range.start, 16)?)?;
        if valid >> TableId::COUNT != 0 {
            return Err(Error::invalid(
                self.add(range.start, 8)?,
                "unknown metadata table is present",
            ));
        }
        if sorted >> TableId::COUNT != 0 {
            return Err(Error::invalid(
                range.start + 16,
                "sorted mask contains unknown tables",
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
        for id in [
            TableId::FieldPtr,
            TableId::MethodPtr,
            TableId::ParamPtr,
            TableId::EventPtr,
            TableId::PropertyPtr,
            TableId::EncLog,
            TableId::EncMap,
        ] {
            let rows = rows[id as usize];
            if rows != 0 {
                return Err(Error::UnsupportedTable {
                    table: id.schema().name(),
                    rows,
                });
            }
        }
        let mut tables = [TableLayout::default(); TableId::COUNT];
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
            tables[number] = TableLayout {
                offset: cursor,
                rows: row_count,
                row_size,
                columns: widths.columns(id),
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
        Ok((tables, sorted))
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
            .map(|column| self.column(*column))
            .sum()
    }

    fn columns(&self, id: TableId) -> ColumnLayout {
        let mut result = ColumnLayout::default();
        for column in id.schema().columns() {
            result.push(self.column(*column));
        }
        result
    }

    fn column(&self, column: Column) -> usize {
        match column {
            Column::U16 => 2,
            Column::U32 => 4,
            Column::String => self.heap(0),
            Column::Guid => self.heap(1),
            Column::Blob => self.heap(2),
            Column::Table(table) | Column::List(table) => self.table(table),
            Column::Coded(code) => {
                let maximum = code
                    .targets()
                    .iter()
                    .map(|target| self.rows[target.table as usize])
                    .max()
                    .unwrap_or_default();
                if maximum < (1u32 << (16 - code.tag_bits())) {
                    2
                } else {
                    4
                }
            }
        }
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

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct ColumnLayout {
    offsets: [u8; MAX_COLUMNS + 1],
    count: u8,
}

impl ColumnLayout {
    fn push(&mut self, width: usize) {
        let count = self.count as usize;
        assert!(count < MAX_COLUMNS);
        self.offsets[count + 1] = self.offsets[count] + width as u8;
        self.count += 1;
    }

    fn get(self, column: usize) -> Option<(usize, usize)> {
        if column >= self.count as usize {
            return None;
        }
        Some((
            self.offsets[column] as usize,
            (self.offsets[column + 1] - self.offsets[column]) as usize,
        ))
    }
}

impl TableLayout {
    fn column(self, column: usize) -> Option<(usize, usize)> {
        self.columns.get(column)
    }
}

fn compressed_u32(bytes: &[u8]) -> Option<(u32, usize)> {
    let first = *bytes.first()?;
    match first {
        0x00..=0x7f => Some((first as u32, 1)),
        0x80..=0xbf => {
            let value = ((first as u32 & 0x3f) << 8) | *bytes.get(1)? as u32;
            (value >= 0x80).then_some((value, 2))
        }
        0xc0..=0xdf => {
            let value = ((first as u32 & 0x1f) << 24)
                | (*bytes.get(1)? as u32) << 16
                | (*bytes.get(2)? as u32) << 8
                | *bytes.get(3)? as u32;
            (value >= 0x4000).then_some((value, 4))
        }
        _ => None,
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
    fn sorted_table_ranges_match_linear_scans() {
        for bytes in [windows_default::WINRT, windows_default::WIN32] {
            let image = Image::new(bytes).unwrap();
            let mut sorted_tables = 0;
            for schema in TABLES.iter().filter(|schema| {
                schema.sorted_column().is_some() && image.table(schema.id()).rows() > 1
            }) {
                if image.sorted & (1u64 << schema.id().as_u8()) == 0 {
                    continue;
                }
                sorted_tables += 1;
                let column = schema.sorted_column().unwrap();
                let mut previous = 0;
                for row in 1..=image.table(schema.id()).rows() {
                    let value = image.raw_column_data(schema.id(), row, column).unwrap().0;
                    assert!(
                        value >= previous,
                        "{} is not ordered by column {column}",
                        schema.name()
                    );
                    previous = value;
                }
            }
            assert!(sorted_tables > 0);

            let attributes: Vec<_> = image.rows::<tables::CustomAttribute>().collect();
            if let Some(row) = attributes.get(attributes.len() / 2) {
                let value = image.column(*row, 0).unwrap();
                let expected: Vec<_> = attributes
                    .iter()
                    .copied()
                    .filter(|row| image.column(*row, 0).unwrap() == value)
                    .collect();
                let actual: Vec<_> = image
                    .matching_rows::<tables::CustomAttribute>(0, value)
                    .unwrap()
                    .collect();
                assert_eq!(actual, expected);
            }

            let first = image.rows::<tables::TypeDef>().next().unwrap();
            let value = image.column(first, 1).unwrap();
            let expected: Vec<_> = image
                .rows::<tables::TypeDef>()
                .filter(|row| image.column(*row, 1).unwrap() == value)
                .collect();
            let actual: Vec<_> = image
                .matching_rows::<tables::TypeDef>(1, value)
                .unwrap()
                .collect();
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn type_identities_match_existing_reader() {
        for bytes in [windows_default::WINRT, windows_default::WIN32] {
            let image = Image::new(bytes).unwrap();
            let mut actual: Vec<_> = image
                .rows::<tables::TypeDef>()
                .filter_map(|row| {
                    let row = image.view(row).unwrap();
                    let namespace = row.string(2).unwrap();
                    if namespace.is_empty() {
                        return None;
                    }
                    let name = row.string(1).unwrap();
                    Some((namespace.to_string(), name.to_string()))
                })
                .collect();

            let file = windows_metadata::reader::File::new(bytes.to_vec()).unwrap();
            let index = windows_metadata::reader::Index::new(vec![file]);
            let mut expected: Vec<_> = index
                .iter()
                .map(|(_, _, def)| (def.namespace().to_string(), def.name().to_string()))
                .collect();

            actual.sort();
            expected.sort();
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn type_members_and_base_types_match_existing_reader() {
        for bytes in [windows_default::WINRT, windows_default::WIN32] {
            let image = Image::new(bytes).unwrap();
            let mut actual = Vec::new();
            for id in image.rows::<tables::TypeDef>() {
                let row = image.view(id).unwrap();
                let namespace = row.string(2).unwrap();
                if namespace.is_empty() {
                    continue;
                }

                let next = id
                    .number()
                    .checked_add(1)
                    .and_then(|number| image.row::<tables::TypeDef>(number));
                let field_end = next.map_or_else(
                    || {
                        ListIndex::new(image.table(TableId::Field).rows().checked_add(1).unwrap())
                            .unwrap()
                    },
                    |next| image.view(next).unwrap().list::<tables::Field>(4).unwrap(),
                );
                let method_end = next.map_or_else(
                    || {
                        ListIndex::new(
                            image
                                .table(TableId::MethodDef)
                                .rows()
                                .checked_add(1)
                                .unwrap(),
                        )
                        .unwrap()
                    },
                    |next| {
                        image
                            .view(next)
                            .unwrap()
                            .list::<tables::MethodDef>(5)
                            .unwrap()
                    },
                );

                let fields: Vec<_> = image
                    .list_range(row.list::<tables::Field>(4).unwrap(), field_end)
                    .unwrap()
                    .map(|field| image.view(field).unwrap().string(1).unwrap().to_string())
                    .collect();
                let methods: Vec<_> = image
                    .list_range(row.list::<tables::MethodDef>(5).unwrap(), method_end)
                    .unwrap()
                    .map(|method| image.view(method).unwrap().string(3).unwrap().to_string())
                    .collect();
                let extends = row.coded(3).unwrap().map(|extends| match extends.table() {
                    TableId::TypeDef => {
                        let id = image.row::<tables::TypeDef>(extends.number()).unwrap();
                        let row = image.view(id).unwrap();
                        (
                            row.string(2).unwrap().to_string(),
                            row.string(1).unwrap().to_string(),
                        )
                    }
                    TableId::TypeRef => {
                        let id = image.row::<tables::TypeRef>(extends.number()).unwrap();
                        let row = image.view(id).unwrap();
                        (
                            row.string(2).unwrap().to_string(),
                            row.string(1).unwrap().to_string(),
                        )
                    }
                    rest => panic!("unexpected base type table {rest:?}"),
                });
                actual.push((
                    namespace.to_string(),
                    row.string(1).unwrap().to_string(),
                    fields,
                    methods,
                    extends,
                ));
            }

            let file = windows_metadata::reader::File::new(bytes.to_vec()).unwrap();
            let index = windows_metadata::reader::Index::new(vec![file]);
            let mut expected: Vec<_> = index
                .iter()
                .map(|(_, _, def)| {
                    (
                        def.namespace().to_string(),
                        def.name().to_string(),
                        def.fields()
                            .map(|field| field.name().to_string())
                            .collect::<Vec<_>>(),
                        def.methods()
                            .map(|method| method.name().to_string())
                            .collect::<Vec<_>>(),
                        def.extends().map(|extends| {
                            (extends.namespace().to_string(), extends.name().to_string())
                        }),
                    )
                })
                .collect();

            actual.sort();
            expected.sort();
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn reads_heap_boundaries() {
        let image = Image::new(windows_default::WINRT).unwrap();
        assert_eq!(image.string(StringId::new(0)).unwrap(), "");
        assert_eq!(image.blob(BlobId::new(0)).unwrap(), []);
        assert_eq!(image.guid(GuidId::new(0)).unwrap(), None);

        let module = image.rows::<tables::Module>().next().unwrap();
        let mvid = GuidId::new(image.column(module, 2).unwrap());
        assert_ne!(image.guid(mvid).unwrap(), None);

        let method = image.rows::<tables::MethodDef>().next().unwrap();
        let signature = BlobId::new(image.column(method, 4).unwrap());
        assert!(!image.blob(signature).unwrap().is_empty());

        assert!(image.string(StringId::new(u32::MAX)).is_err());
        assert!(image.blob(BlobId::new(u32::MAX)).is_err());
        assert!(image.guid(GuidId::new(u32::MAX)).is_err());
    }

    #[test]
    fn representative_signatures_match_existing_reader() {
        let image = Image::new(windows_default::WINRT).unwrap();
        let point = image
            .rows::<tables::TypeDef>()
            .find(|id| {
                let row = image.view(*id).unwrap();
                row.string(2).unwrap() == "Windows.Foundation" && row.string(1).unwrap() == "Point"
            })
            .unwrap();
        let point = image.view(point).unwrap();
        let field_start = point.list::<tables::Field>(4).unwrap().number();
        let x = image
            .view(image.row::<tables::Field>(field_start).unwrap())
            .unwrap();
        let x_type = image.field_signature(x.blob_id(2).unwrap()).unwrap();
        assert_eq!(x_type.kind, TypeKind::F32);

        let old = windows_metadata::reader::Index::new(vec![
            windows_metadata::reader::File::new(windows_default::WINRT.to_vec()).unwrap(),
        ]);
        let old_point = old.expect("Windows.Foundation", "Point");
        assert_eq!(
            old_point.fields().next().unwrap().ty(),
            windows_metadata::Type::F32
        );

        let stringable = image
            .rows::<tables::TypeDef>()
            .find(|id| {
                let row = image.view(*id).unwrap();
                row.string(2).unwrap() == "Windows.Foundation"
                    && row.string(1).unwrap() == "IStringable"
            })
            .unwrap();
        let stringable = image.view(stringable).unwrap();
        let method_start = stringable.list::<tables::MethodDef>(5).unwrap().number();
        let method = image
            .view(image.row::<tables::MethodDef>(method_start).unwrap())
            .unwrap();
        let signature = image.method_signature(method.blob_id(4).unwrap()).unwrap();
        assert_eq!(signature.return_type.kind, TypeKind::String);
        assert!(signature.parameters.is_empty());

        let old_stringable = old.expect("Windows.Foundation", "IStringable");
        let old_signature = old_stringable.methods().next().unwrap().signature(&[]);
        assert_eq!(old_signature.return_type, windows_metadata::Type::String);
        assert!(old_signature.types.is_empty());
    }

    #[test]
    fn compressed_lengths_require_canonical_encoding() {
        assert_eq!(compressed_u32(&[0x7f]), Some((0x7f, 1)));
        assert_eq!(compressed_u32(&[0x80, 0x80]), Some((0x80, 2)));
        assert_eq!(compressed_u32(&[0xbf, 0xff]), Some((0x3fff, 2)));
        assert_eq!(compressed_u32(&[0xc0, 0x00, 0x40, 0x00]), Some((0x4000, 4)));
        assert_eq!(compressed_u32(&[0x80, 0x7f]), None);
        assert_eq!(compressed_u32(&[0xc0, 0x00, 0x3f, 0xff]), None);
        assert_eq!(compressed_u32(&[0xe0]), None);
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
