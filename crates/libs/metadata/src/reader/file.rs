use super::*;

pub struct File {
    bytes: Vec<u8>,
    strings: std::ops::Range<usize>,
    blobs: std::ops::Range<usize>,
    tables: [Table; 24],
}

impl File {
    pub fn read<P: AsRef<std::path::Path>>(path: P) -> Option<Self> {
        std::fs::read(path).ok().and_then(Self::new)
    }

    /// Reads a PE metadata container, checking header, stream, and table extents.
    ///
    /// Malformed row contents or signatures can still panic during subsequent reads.
    pub fn new(bytes: Vec<u8>) -> Option<Self> {
        let mut result = Self {
            bytes,
            strings: 0..0,
            blobs: 0..0,
            tables: Default::default(),
        };

        let dos = result.bytes.copy_as::<IMAGE_DOS_HEADER>(0)?;
        let nt_offset = usize::try_from(dos.e_lfanew).ok()?;

        if dos.e_magic != IMAGE_DOS_SIGNATURE as u16
            || result.bytes.copy_as::<u32>(nt_offset)? != IMAGE_NT_SIGNATURE as u32
        {
            return None;
        }

        let file_offset = nt_offset + size_of::<u32>();
        let file = result.bytes.copy_as::<IMAGE_FILE_HEADER>(file_offset)?;

        let optional_offset = file_offset + size_of::<IMAGE_FILE_HEADER>();
        let sections_offset = optional_offset.checked_add(file.SizeOfOptionalHeader as usize)?;
        let optional = result.bytes.get(optional_offset..sections_offset)?;
        let (directories, directory_count) = match optional.copy_as::<u16>(0)? as i32 {
            IMAGE_NT_OPTIONAL_HDR32_MAGIC => {
                let optional = optional.copy_as::<IMAGE_OPTIONAL_HEADER32>(0)?;
                (optional.DataDirectory, optional.NumberOfRvaAndSizes)
            }
            IMAGE_NT_OPTIONAL_HDR64_MAGIC => {
                let optional = optional.copy_as::<IMAGE_OPTIONAL_HEADER64>(0)?;
                (optional.DataDirectory, optional.NumberOfRvaAndSizes)
            }
            _ => return None,
        };
        if directory_count <= IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR as u32 {
            return None;
        }
        let com = directories[IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR as usize];
        if com.Size < size_of::<IMAGE_COR20_HEADER>() as u32 {
            return None;
        }

        let sections_len = file.NumberOfSections as usize * size_of::<IMAGE_SECTION_HEADER>();
        let sections = result
            .bytes
            .get(sections_offset..sections_offset.checked_add(sections_len)?)?;

        let clr_range = range_from_rva(sections, com.VirtualAddress, com.Size as usize)?;
        let clr = result
            .bytes
            .get(clr_range)?
            .copy_as::<IMAGE_COR20_HEADER>(0)?;

        if clr.cb != size_of::<IMAGE_COR20_HEADER>() as u32 {
            return None;
        }

        let metadata_range = range_from_rva(
            sections,
            clr.MetaData.VirtualAddress,
            clr.MetaData.Size as usize,
        )?;
        let metadata_offset = metadata_range.start;
        let metadata = result.bytes.get(metadata_range)?;

        if metadata.copy_as::<u32>(0)? != METADATA_SIGNATURE {
            return None;
        }

        let version_len = metadata.copy_as::<u32>(12)? as usize;
        let mut view = version_len.checked_add(20)?;
        let mut tables_data = None;

        for _ in 0..metadata.copy_as::<u16>(view - 2)? {
            let stream_offset = metadata.copy_as::<u32>(view)? as usize;
            let stream_len = metadata.copy_as::<u32>(view + 4)? as usize;
            let stream_end = stream_offset.checked_add(stream_len)?;
            metadata.get(stream_offset..stream_end)?;
            let range = metadata_offset + stream_offset..metadata_offset + stream_end;
            let stream_name = metadata.view_as_str(view + 8)?;
            match stream_name {
                b"#Strings" => result.strings = range,
                b"#Blob" => result.blobs = range,
                b"#~" => tables_data = Some(range),
                b"#GUID" | b"#US" => {}
                _ => return None,
            }
            let padding = 4 - stream_name.len() % 4;
            view += 8 + stream_name.len() + padding;
            metadata.get(..view)?;
        }

        let tables_data = tables_data?;
        let table_bytes = result.bytes.get(tables_data.clone())?;
        table_bytes.get(..24)?;
        let heap_sizes = table_bytes.copy_as::<u8>(6)?;
        let string_index_size = if (heap_sizes & 1) == 1 { 4 } else { 2 };
        let guid_index_size = if ((heap_sizes >> 1) & 1) == 1 { 4 } else { 2 };
        let blob_index_size = if ((heap_sizes >> 2) & 1) == 1 { 4 } else { 2 };
        let valid_bits = table_bytes.copy_as::<u64>(8)?;
        view = 24;

        // These tables determine sizes and offsets for later tables.
        let unused_empty = Table::default();
        let mut unused_assembly_os = Table::default();
        let mut unused_assembly_processor = Table::default();
        let mut unused_assembly_ref_os = Table::default();
        let mut unused_assembly_ref = Table::default();
        let mut unused_assembly_ref_processor = Table::default();
        let mut unused_decl_security = Table::default();
        let mut unused_exported_type = Table::default();
        let mut unused_field_marshal = Table::default();
        let mut unused_field_rva = Table::default();
        let mut unused_file = Table::default();
        let mut unused_generic_param_constraint = Table::default();
        let mut unused_manifest_resource = Table::default();
        let mut unused_method_impl = Table::default();
        let mut unused_method_spec = Table::default();
        let mut unused_standalone_sig = Table::default();
        let mut unused_module = Table::default();

        for i in 0..64 {
            if ((valid_bits >> i) & 1) == 0 {
                continue;
            }

            let len = table_bytes.copy_as::<u32>(view)? as usize;
            view += 4;

            match i {
                0x00 => unused_module.len = len,
                0x01 => result.tables[TypeRef::TABLE].len = len,
                0x02 => result.tables[TypeDef::TABLE].len = len,
                0x04 => result.tables[Field::TABLE].len = len,
                0x06 => result.tables[MethodDef::TABLE].len = len,
                0x08 => result.tables[MethodParam::TABLE].len = len,
                0x09 => result.tables[InterfaceImpl::TABLE].len = len,
                0x0a => result.tables[MemberRef::TABLE].len = len,
                0x0b => result.tables[Constant::TABLE].len = len,
                0x0c => result.tables[Attribute::TABLE].len = len,
                0x0d => unused_field_marshal.len = len,
                0x0e => unused_decl_security.len = len,
                0x0f => result.tables[ClassLayout::TABLE].len = len,
                0x10 => result.tables[FieldLayout::TABLE].len = len,
                0x11 => unused_standalone_sig.len = len,
                0x12 => result.tables[EventMap::TABLE].len = len,
                0x14 => result.tables[Event::TABLE].len = len,
                0x15 => result.tables[PropertyMap::TABLE].len = len,
                0x17 => result.tables[Property::TABLE].len = len,
                0x18 => result.tables[MethodSemantics::TABLE].len = len,
                0x19 => unused_method_impl.len = len,
                0x1a => result.tables[ModuleRef::TABLE].len = len,
                0x1b => result.tables[TypeSpec::TABLE].len = len,
                0x1c => result.tables[ImplMap::TABLE].len = len,
                0x1d => unused_field_rva.len = len,
                0x20 => result.tables[Assembly::TABLE].len = len,
                0x21 => unused_assembly_processor.len = len,
                0x22 => unused_assembly_os.len = len,
                0x23 => unused_assembly_ref.len = len,
                0x24 => unused_assembly_ref_processor.len = len,
                0x25 => unused_assembly_ref_os.len = len,
                0x26 => unused_file.len = len,
                0x27 => unused_exported_type.len = len,
                0x28 => unused_manifest_resource.len = len,
                0x29 => result.tables[NestedClass::TABLE].len = len,
                0x2a => result.tables[GenericParam::TABLE].len = len,
                0x2b => unused_method_spec.len = len,
                0x2c => unused_generic_param_constraint.len = len,
                _ => return None,
            };
        }

        let tables = &result.tables;
        let type_def_or_ref = coded_index_size(&[
            tables[TypeDef::TABLE].len,
            tables[TypeRef::TABLE].len,
            tables[TypeSpec::TABLE].len,
        ]);
        let has_constant = coded_index_size(&[
            tables[Field::TABLE].len,
            tables[MethodParam::TABLE].len,
            result.tables[Property::TABLE].len,
        ]);
        let has_field_marshal =
            coded_index_size(&[tables[Field::TABLE].len, tables[MethodParam::TABLE].len]);
        let has_decl_security = coded_index_size(&[
            tables[TypeDef::TABLE].len,
            tables[MethodDef::TABLE].len,
            tables[Assembly::TABLE].len,
        ]);
        let member_ref_parent = coded_index_size(&[
            tables[TypeDef::TABLE].len,
            tables[TypeRef::TABLE].len,
            tables[ModuleRef::TABLE].len,
            tables[MethodDef::TABLE].len,
            tables[TypeSpec::TABLE].len,
        ]);
        let has_semantics = coded_index_size(&[
            result.tables[Event::TABLE].len,
            result.tables[Property::TABLE].len,
        ]);
        let method_def_or_ref =
            coded_index_size(&[tables[MethodDef::TABLE].len, tables[MemberRef::TABLE].len]);
        let member_forwarded =
            coded_index_size(&[tables[Field::TABLE].len, tables[MethodDef::TABLE].len]);
        let implementation = coded_index_size(&[
            unused_file.len,
            unused_assembly_ref.len,
            unused_exported_type.len,
        ]);
        let custom_attribute_type = coded_index_size(&[
            tables[MethodDef::TABLE].len,
            tables[MemberRef::TABLE].len,
            unused_empty.len,
            unused_empty.len,
            unused_empty.len,
        ]);
        let resolution_scope = coded_index_size(&[
            unused_module.len,
            tables[ModuleRef::TABLE].len,
            unused_assembly_ref.len,
            tables[TypeRef::TABLE].len,
        ]);
        let type_or_method_def =
            coded_index_size(&[tables[TypeDef::TABLE].len, tables[MethodDef::TABLE].len]);

        let has_custom_attribute = coded_index_size(&[
            tables[MethodDef::TABLE].len,
            tables[Field::TABLE].len,
            tables[TypeRef::TABLE].len,
            tables[TypeDef::TABLE].len,
            tables[MethodParam::TABLE].len,
            tables[InterfaceImpl::TABLE].len,
            tables[MemberRef::TABLE].len,
            unused_module.len,
            result.tables[Property::TABLE].len,
            result.tables[Event::TABLE].len,
            unused_standalone_sig.len,
            tables[ModuleRef::TABLE].len,
            tables[TypeSpec::TABLE].len,
            tables[Assembly::TABLE].len,
            unused_assembly_ref.len,
            unused_file.len,
            unused_exported_type.len,
            unused_manifest_resource.len,
            tables[GenericParam::TABLE].len,
            unused_generic_param_constraint.len,
            unused_method_spec.len,
        ]);

        result.tables[Assembly::TABLE].set_columns(
            4,
            8,
            4,
            blob_index_size,
            string_index_size,
            string_index_size,
        );
        unused_assembly_os.set_columns(4, 4, 4, 0, 0, 0);
        unused_assembly_processor.set_columns(4, 0, 0, 0, 0, 0);
        unused_assembly_ref.set_columns(
            8,
            4,
            blob_index_size,
            string_index_size,
            string_index_size,
            blob_index_size,
        );
        unused_assembly_ref_os.set_columns(4, 4, 4, unused_assembly_ref.index_width(), 0, 0);
        unused_assembly_ref_processor.set_columns(4, unused_assembly_ref.index_width(), 0, 0, 0, 0);
        result.tables[ClassLayout::TABLE].set_columns(
            2,
            4,
            result.tables[TypeDef::TABLE].index_width(),
            0,
            0,
            0,
        );
        result.tables[Constant::TABLE].set_columns(2, has_constant, blob_index_size, 0, 0, 0);
        result.tables[Attribute::TABLE].set_columns(
            has_custom_attribute,
            custom_attribute_type,
            blob_index_size,
            0,
            0,
            0,
        );
        unused_decl_security.set_columns(2, has_decl_security, blob_index_size, 0, 0, 0);
        result.tables[EventMap::TABLE].set_columns(
            result.tables[TypeDef::TABLE].index_width(),
            result.tables[Event::TABLE].index_width(),
            0,
            0,
            0,
            0,
        );
        result.tables[Event::TABLE].set_columns(2, string_index_size, type_def_or_ref, 0, 0, 0);
        unused_exported_type.set_columns(
            4,
            4,
            string_index_size,
            string_index_size,
            implementation,
            0,
        );
        result.tables[Field::TABLE].set_columns(2, string_index_size, blob_index_size, 0, 0, 0);
        result.tables[FieldLayout::TABLE].set_columns(
            4,
            result.tables[Field::TABLE].index_width(),
            0,
            0,
            0,
            0,
        );
        unused_field_marshal.set_columns(has_field_marshal, blob_index_size, 0, 0, 0, 0);
        unused_field_rva.set_columns(4, result.tables[Field::TABLE].index_width(), 0, 0, 0, 0);
        unused_file.set_columns(4, string_index_size, blob_index_size, 0, 0, 0);
        result.tables[GenericParam::TABLE].set_columns(
            2,
            2,
            type_or_method_def,
            string_index_size,
            0,
            0,
        );
        unused_generic_param_constraint.set_columns(
            result.tables[GenericParam::TABLE].index_width(),
            type_def_or_ref,
            0,
            0,
            0,
            0,
        );
        result.tables[ImplMap::TABLE].set_columns(
            2,
            member_forwarded,
            string_index_size,
            result.tables[ModuleRef::TABLE].index_width(),
            0,
            0,
        );
        result.tables[InterfaceImpl::TABLE].set_columns(
            result.tables[TypeDef::TABLE].index_width(),
            type_def_or_ref,
            0,
            0,
            0,
            0,
        );
        unused_manifest_resource.set_columns(4, 4, string_index_size, implementation, 0, 0);
        result.tables[MemberRef::TABLE].set_columns(
            member_ref_parent,
            string_index_size,
            blob_index_size,
            0,
            0,
            0,
        );
        result.tables[MethodDef::TABLE].set_columns(
            4,
            2,
            2,
            string_index_size,
            blob_index_size,
            result.tables[MethodParam::TABLE].index_width(),
        );
        unused_method_impl.set_columns(
            result.tables[TypeDef::TABLE].index_width(),
            method_def_or_ref,
            method_def_or_ref,
            0,
            0,
            0,
        );
        result.tables[MethodSemantics::TABLE].set_columns(
            2,
            result.tables[MethodDef::TABLE].index_width(),
            has_semantics,
            0,
            0,
            0,
        );
        unused_method_spec.set_columns(method_def_or_ref, blob_index_size, 0, 0, 0, 0);
        unused_module.set_columns(
            2,
            string_index_size,
            guid_index_size,
            guid_index_size,
            guid_index_size,
            0,
        );
        result.tables[ModuleRef::TABLE].set_columns(string_index_size, 0, 0, 0, 0, 0);
        result.tables[NestedClass::TABLE].set_columns(
            result.tables[TypeDef::TABLE].index_width(),
            result.tables[TypeDef::TABLE].index_width(),
            0,
            0,
            0,
            0,
        );
        result.tables[MethodParam::TABLE].set_columns(2, 2, string_index_size, 0, 0, 0);
        result.tables[Property::TABLE].set_columns(2, string_index_size, blob_index_size, 0, 0, 0);
        result.tables[PropertyMap::TABLE].set_columns(
            result.tables[TypeDef::TABLE].index_width(),
            result.tables[Property::TABLE].index_width(),
            0,
            0,
            0,
            0,
        );
        unused_standalone_sig.set_columns(blob_index_size, 0, 0, 0, 0, 0);
        result.tables[TypeDef::TABLE].set_columns(
            4,
            string_index_size,
            string_index_size,
            type_def_or_ref,
            result.tables[Field::TABLE].index_width(),
            result.tables[MethodDef::TABLE].index_width(),
        );
        result.tables[TypeRef::TABLE].set_columns(
            resolution_scope,
            string_index_size,
            string_index_size,
            0,
            0,
            0,
        );
        result.tables[TypeSpec::TABLE].set_columns(blob_index_size, 0, 0, 0, 0, 0);

        view += tables_data.start;
        unused_module.set_data(&mut view)?;
        result.tables[TypeRef::TABLE].set_data(&mut view)?;
        result.tables[TypeDef::TABLE].set_data(&mut view)?;
        result.tables[Field::TABLE].set_data(&mut view)?;
        result.tables[MethodDef::TABLE].set_data(&mut view)?;
        result.tables[MethodParam::TABLE].set_data(&mut view)?;
        result.tables[InterfaceImpl::TABLE].set_data(&mut view)?;
        result.tables[MemberRef::TABLE].set_data(&mut view)?;
        result.tables[Constant::TABLE].set_data(&mut view)?;
        result.tables[Attribute::TABLE].set_data(&mut view)?;
        unused_field_marshal.set_data(&mut view)?;
        unused_decl_security.set_data(&mut view)?;
        result.tables[ClassLayout::TABLE].set_data(&mut view)?;
        result.tables[FieldLayout::TABLE].set_data(&mut view)?;
        unused_standalone_sig.set_data(&mut view)?;
        result.tables[EventMap::TABLE].set_data(&mut view)?;
        result.tables[Event::TABLE].set_data(&mut view)?;
        result.tables[PropertyMap::TABLE].set_data(&mut view)?;
        result.tables[Property::TABLE].set_data(&mut view)?;
        result.tables[MethodSemantics::TABLE].set_data(&mut view)?;
        unused_method_impl.set_data(&mut view)?;
        result.tables[ModuleRef::TABLE].set_data(&mut view)?;
        result.tables[TypeSpec::TABLE].set_data(&mut view)?;
        result.tables[ImplMap::TABLE].set_data(&mut view)?;
        unused_field_rva.set_data(&mut view)?;
        result.tables[Assembly::TABLE].set_data(&mut view)?;
        unused_assembly_processor.set_data(&mut view)?;
        unused_assembly_os.set_data(&mut view)?;
        unused_assembly_ref.set_data(&mut view)?;
        unused_assembly_ref_processor.set_data(&mut view)?;
        unused_assembly_ref_os.set_data(&mut view)?;
        unused_file.set_data(&mut view)?;
        unused_exported_type.set_data(&mut view)?;
        unused_manifest_resource.set_data(&mut view)?;
        result.tables[NestedClass::TABLE].set_data(&mut view)?;
        result.tables[GenericParam::TABLE].set_data(&mut view)?;
        unused_method_spec.set_data(&mut view)?;
        unused_generic_param_constraint.set_data(&mut view)?;
        if view > tables_data.end {
            return None;
        }

        Some(result)
    }

    pub(crate) fn usize(&self, row: usize, table: usize, column: usize) -> usize {
        let table = &self.tables[table];
        assert!(row < table.len, "metadata row is out of bounds");
        let column = &table.columns[column];
        let offset = table.offset + row * table.width + column.offset;
        match column.width {
            1 => self.bytes.copy_as::<u8>(offset).unwrap() as usize,
            2 => self.bytes.copy_as::<u16>(offset).unwrap() as usize,
            4 => self.bytes.copy_as::<u32>(offset).unwrap() as usize,
            8 => self.bytes.copy_as::<u64>(offset).unwrap() as usize,
            _ => unreachable!(),
        }
    }

    pub(crate) fn str(&self, row: usize, table: usize, column: usize) -> &str {
        let heap = &self.bytes[self.strings.clone()];
        let offset = self.usize(row, table, column);
        let bytes = &heap[offset..];
        let nul_pos = bytes
            .iter()
            .position(|&c| c == 0)
            .expect("expected null-terminated C-string");
        std::str::from_utf8(&bytes[..nul_pos]).expect("expected valid utf-8 C-string")
    }

    pub(crate) fn blob(&self, row: usize, table: usize, column: usize) -> &[u8] {
        let heap = &self.bytes[self.blobs.clone()];
        let offset = self.usize(row, table, column);
        let initial_byte = heap[offset];

        let (blob_size, blob_size_bytes) = match initial_byte >> 5 {
            0..=3 => (initial_byte & 0x7f, 1),
            4..=5 => (initial_byte & 0x3f, 2),
            6 => (initial_byte & 0x1f, 4),
            rest => panic!("{rest:?}"),
        };

        let mut blob_size = blob_size as usize;

        for byte in &heap[offset + 1..offset + blob_size_bytes] {
            blob_size = blob_size.checked_shl(8).unwrap_or(0) + (*byte as usize);
        }

        let offset = offset + blob_size_bytes;
        &heap[offset..offset + blob_size]
    }

    pub(crate) fn list(
        &self,
        row: usize,
        table: usize,
        column: usize,
        other_table: usize,
    ) -> std::ops::Range<usize> {
        let first = self.usize(row, table, column) - 1;
        let next = row + 1;
        let last = if next < self.tables[table].len {
            self.usize(next, table, column) - 1
        } else {
            self.tables[other_table].len
        };
        first..last
    }

    pub(crate) fn equal_range(
        &self,
        table: usize,
        column: usize,
        value: usize,
    ) -> std::ops::Range<usize> {
        let mut first = 0;
        let mut last = self.tables[table].len;
        let mut count = last;

        loop {
            if count == 0 {
                last = first;
                break;
            }

            let count2 = count / 2;
            let middle = first + count2;
            let middle_value = self.usize(middle, table, column);

            match middle_value.cmp(&value) {
                Ordering::Less => {
                    first = middle + 1;
                    count -= count2 + 1;
                }
                Ordering::Greater => count = count2,
                Ordering::Equal => {
                    let first2 = self.lower_bound(table, first, middle, column, value);
                    first += count;
                    last = self.upper_bound(table, middle + 1, first, column, value);
                    first = first2;
                    break;
                }
            }
        }

        first..last
    }

    pub(crate) fn parent(&self, row: usize, table: usize, column: usize) -> usize {
        self.upper_bound(table, 0, self.tables[table].len, column, row + 1) - 1
    }

    fn lower_bound(
        &self,
        table: usize,
        mut first: usize,
        last: usize,
        column: usize,
        value: usize,
    ) -> usize {
        let mut count = last - first;
        while count > 0 {
            let count2 = count / 2;
            let middle = first + count2;
            if self.usize(middle, table, column) < value {
                first = middle + 1;
                count -= count2 + 1;
            } else {
                count = count2;
            }
        }
        first
    }

    fn upper_bound(
        &self,
        table: usize,
        mut first: usize,
        last: usize,
        column: usize,
        value: usize,
    ) -> usize {
        let mut count = last - first;
        while count > 0 {
            let count2 = count / 2;
            let middle = first + count2;
            if value < self.usize(middle, table, column) {
                count = count2;
            } else {
                first = middle + 1;
                count -= count2 + 1;
            }
        }
        first
    }

    pub fn assembly_name(&self) -> Option<&str> {
        match self.tables[Assembly::TABLE].len {
            0 => None,
            1 => Some(self.str(0, Assembly::TABLE, 4)),
            rest => panic!("{rest:?}"),
        }
    }

    pub(crate) fn TypeDef(&self) -> std::ops::Range<usize> {
        0..self.tables[TypeDef::TABLE].len
    }

    pub(crate) fn rows(&self, table: usize) -> std::ops::Range<usize> {
        0..self.tables[table].len
    }

    pub(crate) fn NestedClass(&self) -> std::ops::Range<usize> {
        0..self.tables[NestedClass::TABLE].len
    }
}

fn range_from_rva(sections: &[u8], rva: u32, len: usize) -> Option<std::ops::Range<usize>> {
    sections
        .as_chunks::<{ size_of::<IMAGE_SECTION_HEADER>() }>()
        .0
        .iter()
        .find_map(|bytes| {
            let section = bytes.copy_as::<IMAGE_SECTION_HEADER>(0)?;
            let virtual_end = section
                .VirtualAddress
                .checked_add(unsafe { section.Misc.VirtualSize })?;
            if rva < section.VirtualAddress || rva >= virtual_end {
                return None;
            }
            let relative = (rva - section.VirtualAddress) as usize;
            let end = relative.checked_add(len)?;
            if end > unsafe { section.Misc.VirtualSize } as usize
                || end > section.SizeOfRawData as usize
            {
                return None;
            }
            let start = (section.PointerToRawData as usize).checked_add(relative)?;
            Some(start..start.checked_add(len)?)
        })
}

trait View {
    fn copy_as<T: Copy>(&self, offset: usize) -> Option<T>;
    fn view_as_str(&self, offset: usize) -> Option<&[u8]>;
    fn is_proper_length<T>(&self, offset: usize, count: usize) -> Option<()>;
}

impl View for [u8] {
    fn copy_as<T>(&self, offset: usize) -> Option<T> {
        self.is_proper_length::<T>(offset, 1)?;

        // SAFETY: bounds are checked above. Callers use integers or integer-only PE headers,
        // which accept all bit patterns; read_unaligned handles the byte buffer's alignment.
        Some(unsafe { (self[offset..].as_ptr() as *const T).read_unaligned() })
    }

    fn view_as_str(&self, offset: usize) -> Option<&[u8]> {
        let buffer = self.get(offset..)?;
        let pos = buffer.iter().position(|c| *c == b'\0')?;
        Some(&self[offset..offset + pos])
    }

    fn is_proper_length<T>(&self, offset: usize, count: usize) -> Option<()> {
        if offset.checked_add(count.checked_mul(size_of::<T>())?)? <= self.len() {
            Some(())
        } else {
            None
        }
    }
}

#[derive(Default)]
struct Table {
    offset: usize,
    len: usize,
    width: usize,
    columns: [Column; 6],
}

impl Table {
    fn index_width(&self) -> usize {
        if self.len < (1 << 16) { 2 } else { 4 }
    }

    fn set_columns(&mut self, a: usize, b: usize, c: usize, d: usize, e: usize, f: usize) {
        self.width = a + b + c + d + e + f;
        self.columns[0] = Column::new(0, a);
        if b != 0 {
            self.columns[1] = Column::new(a, b);
        }
        if c != 0 {
            self.columns[2] = Column::new(a + b, c);
        }
        if d != 0 {
            self.columns[3] = Column::new(a + b + c, d);
        }
        if e != 0 {
            self.columns[4] = Column::new(a + b + c + d, e);
        }
        if f != 0 {
            self.columns[5] = Column::new(a + b + c + d + e, f);
        }
    }

    fn set_data(&mut self, offset: &mut usize) -> Option<()> {
        if self.len != 0 {
            let next = offset.checked_add(self.len.checked_mul(self.width)?)?;
            self.offset = *offset;
            *offset = next;
        }
        Some(())
    }
}

#[derive(Default)]
struct Column {
    offset: usize,
    width: usize,
}

impl Column {
    fn new(offset: usize, width: usize) -> Self {
        Self { offset, width }
    }
}

const METADATA_SIGNATURE: u32 = 0x424A_5342;

// A coded index must fit the largest table selected by its tag.
fn coded_index_size(tables: &[usize]) -> usize {
    fn small(row_count: usize, bits: u8) -> bool {
        (row_count as u64) < (1u64 << (16 - bits))
    }

    fn bits_needed(value: usize) -> u8 {
        let mut value = value - 1;
        let mut bits: u8 = 1;
        while {
            value >>= 1;
            value != 0
        } {
            bits += 1;
        }
        bits
    }

    let bits_needed = bits_needed(tables.len());

    if tables.iter().all(|table| small(*table, bits_needed)) {
        2
    } else {
        4
    }
}
