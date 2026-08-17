use super::*;

pub struct File {
    bytes: Vec<u8>,
    strings: usize,
    blobs: usize,
    tables: [Table; 18],
}

impl File {
    pub fn read<P: AsRef<std::path::Path>>(path: P) -> Option<Self> {
        std::fs::read(path).ok().and_then(Self::new)
    }

    pub fn new(bytes: Vec<u8>) -> Option<Self> {
        let mut result = Self {
            bytes,
            strings: 0,
            blobs: 0,
            tables: Default::default(),
        };

        let dos = result.bytes.view_as::<IMAGE_DOS_HEADER>(0)?;

        if dos.e_magic != IMAGE_DOS_SIGNATURE as u16
            || result.bytes.copy_as::<u32>(dos.e_lfanew as usize)? != IMAGE_NT_SIGNATURE as u32
        {
            return None;
        }

        let file_offset = dos.e_lfanew as usize + size_of::<u32>();
        let file = result.bytes.view_as::<IMAGE_FILE_HEADER>(file_offset)?;

        let optional_offset = file_offset + size_of::<IMAGE_FILE_HEADER>();
        let optional_size = file.SizeOfOptionalHeader as usize;
        result
            .bytes
            .view_as_slice_of::<u8>(optional_offset, optional_size)?;

        let (directories_offset, directory_count_offset): (usize, usize) =
            match result.bytes.copy_as::<u16>(optional_offset)? as i32 {
                IMAGE_NT_OPTIONAL_HDR32_MAGIC => (
                    std::mem::offset_of!(IMAGE_OPTIONAL_HEADER32, DataDirectory),
                    std::mem::offset_of!(IMAGE_OPTIONAL_HEADER32, NumberOfRvaAndSizes),
                ),
                IMAGE_NT_OPTIONAL_HDR64_MAGIC => (
                    std::mem::offset_of!(IMAGE_OPTIONAL_HEADER64, DataDirectory),
                    std::mem::offset_of!(IMAGE_OPTIONAL_HEADER64, NumberOfRvaAndSizes),
                ),
                _ => return None,
            };
        if directory_count_offset + size_of::<u32>() > optional_size {
            return None;
        }
        let directory_count = result
            .bytes
            .copy_as::<u32>(optional_offset + directory_count_offset)?
            as usize;
        let directory = IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR as usize;
        if directory_count <= directory {
            return None;
        }
        let directory_offset = directories_offset.checked_add(directory.checked_mul(8)?)?;
        if directory_offset.checked_add(8)? > optional_size {
            return None;
        }
        let com_virtual_address = result
            .bytes
            .copy_as::<u32>(optional_offset + directory_offset)?;
        let com_size = result
            .bytes
            .copy_as::<u32>(optional_offset + directory_offset + 4)?
            as usize;
        if com_size < size_of::<IMAGE_COR20_HEADER>() {
            return None;
        }
        let sections = result.bytes.view_as_slice_of::<IMAGE_SECTION_HEADER>(
            optional_offset.checked_add(optional_size)?,
            file.NumberOfSections as usize,
        )?;

        let clr_offset = offset_from_rva(
            sections,
            com_virtual_address,
            size_of::<IMAGE_COR20_HEADER>(),
        )?;
        let clr = result.bytes.view_as::<IMAGE_COR20_HEADER>(clr_offset)?;

        if clr.cb != size_of::<IMAGE_COR20_HEADER>() as u32 {
            return None;
        }

        let metadata_size = clr.MetaData.Size as usize;
        let metadata_offset =
            offset_from_rva(sections, clr.MetaData.VirtualAddress, metadata_size)?;
        if metadata_size < 20 || result.bytes.copy_as::<u32>(metadata_offset)? != METADATA_SIGNATURE
        {
            return None;
        }
        let metadata_end = metadata_offset.checked_add(metadata_size)?;
        let version_len = result.bytes.copy_as::<u32>(metadata_offset + 12)? as usize;
        let version_end = metadata_offset.checked_add(16)?.checked_add(version_len)?;
        let flags = version_end.checked_add(3)? & !3;
        if flags.checked_add(4)? > metadata_end {
            return None;
        }
        let stream_count = result.bytes.copy_as::<u16>(flags + 2)? as usize;
        let mut view = flags + 4;
        let mut tables_data: (usize, usize) = (0, 0);

        for _ in 0..stream_count {
            if view.checked_add(8)? > metadata_end {
                return None;
            }
            let stream_offset = result.bytes.copy_as::<u32>(view)? as usize;
            let stream_len = result.bytes.copy_as::<u32>(view + 4)? as usize;
            let name_offset = view + 8;
            let name_bytes = result.bytes.get(name_offset..metadata_end)?;
            let name_len = name_bytes.iter().position(|byte| *byte == 0)?;
            let stream_name = &name_bytes[..name_len];
            let stream_start = metadata_offset.checked_add(stream_offset)?;
            if stream_start.checked_add(stream_len)? > metadata_end {
                return None;
            }
            match stream_name {
                b"#Strings" => result.strings = stream_start,
                b"#Blob" => result.blobs = stream_start,
                b"#~" => tables_data = (stream_start, stream_len),
                b"#GUID" | b"#US" => {}
                rest => panic!("{rest:?}"),
            }
            view = name_offset.checked_add(name_len.checked_add(4)? & !3)?;
            if view > metadata_end {
                return None;
            }
        }

        if tables_data.1 < 24 {
            return None;
        }
        let tables_end = tables_data.0.checked_add(tables_data.1)?;
        let heap_sizes = result.bytes.copy_as::<u8>(tables_data.0 + 6)?;
        let string_index_size = if (heap_sizes & 1) == 1 { 4 } else { 2 };
        let guid_index_size = if ((heap_sizes >> 1) & 1) == 1 { 4 } else { 2 };
        let blob_index_size = if ((heap_sizes >> 2) & 1) == 1 { 4 } else { 2 };
        let valid_bits = result.bytes.copy_as::<u64>(tables_data.0 + 8)?;
        view = tables_data.0 + 24;

        // These tables determine sizes and offsets for later tables.
        let unused_empty = Table::default();
        let mut unused_assembly_os = Table::default();
        let mut unused_assembly_processor = Table::default();
        let mut unused_assembly_ref_os = Table::default();
        let mut unused_assembly_ref = Table::default();
        let mut unused_assembly_ref_processor = Table::default();
        let mut unused_decl_security = Table::default();
        let mut unused_event = Table::default();
        let mut unused_event_map = Table::default();
        let mut unused_exported_type = Table::default();
        let mut unused_field_layout = Table::default();
        let mut unused_field_marshal = Table::default();
        let mut unused_field_rva = Table::default();
        let mut unused_file = Table::default();
        let mut unused_generic_param_constraint = Table::default();
        let mut unused_manifest_resource = Table::default();
        let mut unused_method_impl = Table::default();
        let mut unused_method_semantics = Table::default();
        let mut unused_method_spec = Table::default();
        let mut unused_property = Table::default();
        let mut unused_property_map = Table::default();
        let mut unused_standalone_sig = Table::default();
        let mut unused_module = Table::default();

        for i in 0..64 {
            if ((valid_bits >> i) & 1) == 0 {
                continue;
            }

            if view.checked_add(4)? > tables_end {
                return None;
            }
            let len = result.bytes.copy_as::<u32>(view)? as usize;
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
                0x10 => unused_field_layout.len = len,
                0x11 => unused_standalone_sig.len = len,
                0x12 => unused_event_map.len = len,
                0x14 => unused_event.len = len,
                0x15 => unused_property_map.len = len,
                0x17 => unused_property.len = len,
                0x18 => unused_method_semantics.len = len,
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
                _ => unreachable!(),
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
            unused_property.len,
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
        let has_semantics = coded_index_size(&[unused_event.len, unused_property.len]);
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
            unused_property.len,
            unused_event.len,
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
        unused_event_map.set_columns(
            result.tables[TypeDef::TABLE].index_width(),
            unused_event.index_width(),
            0,
            0,
            0,
            0,
        );
        unused_event.set_columns(2, string_index_size, type_def_or_ref, 0, 0, 0);
        unused_exported_type.set_columns(
            4,
            4,
            string_index_size,
            string_index_size,
            implementation,
            0,
        );
        result.tables[Field::TABLE].set_columns(2, string_index_size, blob_index_size, 0, 0, 0);
        unused_field_layout.set_columns(4, result.tables[Field::TABLE].index_width(), 0, 0, 0, 0);
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
        unused_method_semantics.set_columns(
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
        unused_property.set_columns(2, string_index_size, blob_index_size, 0, 0, 0);
        unused_property_map.set_columns(
            result.tables[TypeDef::TABLE].index_width(),
            unused_property.index_width(),
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

        unused_module.set_data(&mut view, tables_end)?;
        result.tables[TypeRef::TABLE].set_data(&mut view, tables_end)?;
        result.tables[TypeDef::TABLE].set_data(&mut view, tables_end)?;
        result.tables[Field::TABLE].set_data(&mut view, tables_end)?;
        result.tables[MethodDef::TABLE].set_data(&mut view, tables_end)?;
        result.tables[MethodParam::TABLE].set_data(&mut view, tables_end)?;
        result.tables[InterfaceImpl::TABLE].set_data(&mut view, tables_end)?;
        result.tables[MemberRef::TABLE].set_data(&mut view, tables_end)?;
        result.tables[Constant::TABLE].set_data(&mut view, tables_end)?;
        result.tables[Attribute::TABLE].set_data(&mut view, tables_end)?;
        unused_field_marshal.set_data(&mut view, tables_end)?;
        unused_decl_security.set_data(&mut view, tables_end)?;
        result.tables[ClassLayout::TABLE].set_data(&mut view, tables_end)?;
        unused_field_layout.set_data(&mut view, tables_end)?;
        unused_standalone_sig.set_data(&mut view, tables_end)?;
        unused_event_map.set_data(&mut view, tables_end)?;
        unused_event.set_data(&mut view, tables_end)?;
        unused_property_map.set_data(&mut view, tables_end)?;
        unused_property.set_data(&mut view, tables_end)?;
        unused_method_semantics.set_data(&mut view, tables_end)?;
        unused_method_impl.set_data(&mut view, tables_end)?;
        result.tables[ModuleRef::TABLE].set_data(&mut view, tables_end)?;
        result.tables[TypeSpec::TABLE].set_data(&mut view, tables_end)?;
        result.tables[ImplMap::TABLE].set_data(&mut view, tables_end)?;
        unused_field_rva.set_data(&mut view, tables_end)?;
        result.tables[Assembly::TABLE].set_data(&mut view, tables_end)?;
        unused_assembly_processor.set_data(&mut view, tables_end)?;
        unused_assembly_os.set_data(&mut view, tables_end)?;
        unused_assembly_ref.set_data(&mut view, tables_end)?;
        unused_assembly_ref_processor.set_data(&mut view, tables_end)?;
        unused_assembly_ref_os.set_data(&mut view, tables_end)?;
        unused_file.set_data(&mut view, tables_end)?;
        unused_exported_type.set_data(&mut view, tables_end)?;
        unused_manifest_resource.set_data(&mut view, tables_end)?;
        result.tables[NestedClass::TABLE].set_data(&mut view, tables_end)?;
        result.tables[GenericParam::TABLE].set_data(&mut view, tables_end)?;
        unused_method_spec.set_data(&mut view, tables_end)?;
        unused_generic_param_constraint.set_data(&mut view, tables_end)?;

        let trailing = result.bytes.get(view..tables_end)?;
        if trailing.len() > 4 || trailing.iter().any(|byte| *byte != 0) {
            return None;
        }

        Some(result)
    }

    pub(crate) fn usize(&self, row: usize, table: usize, column: usize) -> usize {
        let table = &self.tables[table];
        let column = &table.columns[column];
        let offset = table.offset + row * table.width + column.offset;
        match column.width {
            1 => self.bytes.copy_as::<u8>(offset).map_or(0, |v| v as usize),
            2 => self.bytes.copy_as::<u16>(offset).map_or(0, |v| v as usize),
            4 => self.bytes.copy_as::<u32>(offset).map_or(0, |v| v as usize),
            _ => self.bytes.copy_as::<u64>(offset).map_or(0, |v| v as usize),
        }
    }

    pub(crate) fn str(&self, row: usize, table: usize, column: usize) -> &str {
        let offset = self.strings + self.usize(row, table, column);
        let bytes = &self.bytes[offset..];
        let nul_pos = bytes
            .iter()
            .position(|&c| c == 0)
            .expect("expected null-terminated C-string");
        std::str::from_utf8(&bytes[..nul_pos]).expect("expected valid utf-8 C-string")
    }

    pub(crate) fn blob(&self, row: usize, table: usize, column: usize) -> &[u8] {
        let offset = self.blobs + self.usize(row, table, column);
        let initial_byte = self.bytes[offset];

        let (blob_size, blob_size_bytes) = match initial_byte >> 5 {
            0..=3 => (initial_byte & 0x7f, 1),
            4..=5 => (initial_byte & 0x3f, 2),
            6 => (initial_byte & 0x1f, 4),
            rest => panic!("{rest:?}"),
        };

        let mut blob_size = blob_size as usize;

        for byte in &self.bytes[offset + 1..offset + blob_size_bytes] {
            blob_size = blob_size.checked_shl(8).unwrap_or(0) + (*byte as usize);
        }

        let offset = offset + blob_size_bytes;
        &self.bytes[offset..offset + blob_size]
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

    pub(crate) fn NestedClass(&self) -> std::ops::Range<usize> {
        0..self.tables[NestedClass::TABLE].len
    }
}

fn offset_from_rva(sections: &[IMAGE_SECTION_HEADER], rva: u32, size: usize) -> Option<usize> {
    for section in sections {
        let virtual_size = unsafe { section.Misc.VirtualSize };
        let section_size = virtual_size.max(section.SizeOfRawData);
        let section_end = section.VirtualAddress.checked_add(section_size)?;
        if rva >= section.VirtualAddress && rva < section_end {
            let offset = rva - section.VirtualAddress;
            if (offset as usize).checked_add(size)? > section.SizeOfRawData as usize {
                return None;
            }
            return section
                .PointerToRawData
                .checked_add(offset)
                .map(|value| value as usize);
        }
    }
    None
}

trait View {
    fn view_as<T>(&self, offset: usize) -> Option<&T>;
    fn view_as_slice_of<T>(&self, offset: usize, len: usize) -> Option<&[T]>;
    fn copy_as<T: Copy>(&self, offset: usize) -> Option<T>;
    fn is_proper_length<T>(&self, offset: usize, count: usize) -> Option<()>;
    fn is_proper_length_and_alignment<T>(&self, offset: usize, count: usize) -> Option<*const T>;
}

impl View for [u8] {
    fn view_as<T>(&self, offset: usize) -> Option<&T> {
        unsafe { Some(&*self.is_proper_length_and_alignment(offset, 1)?) }
    }

    fn view_as_slice_of<T>(&self, offset: usize, len: usize) -> Option<&[T]> {
        unsafe {
            Some(std::slice::from_raw_parts(
                self.is_proper_length_and_alignment(offset, len)?,
                len,
            ))
        }
    }

    fn copy_as<T>(&self, offset: usize) -> Option<T> {
        self.is_proper_length::<T>(offset, 1)?;

        // SAFETY: bounds verified by is_proper_length; T: Copy (as required by the trait
        // definition) ensures the value can be bitwise-copied; read_unaligned handles any
        // alignment of the source pointer.
        Some(unsafe { (self[offset..].as_ptr() as *const T).read_unaligned() })
    }

    fn is_proper_length<T>(&self, offset: usize, count: usize) -> Option<()> {
        if offset.checked_add(count.checked_mul(size_of::<T>())?)? <= self.len() {
            Some(())
        } else {
            None
        }
    }

    fn is_proper_length_and_alignment<T>(&self, offset: usize, count: usize) -> Option<*const T> {
        self.is_proper_length::<T>(offset, count)?;
        let ptr = &self[offset] as *const u8 as *const T;

        if ptr.align_offset(align_of::<T>()) == 0 {
            Some(ptr)
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

    fn set_data(&mut self, offset: &mut usize, end: usize) -> Option<()> {
        if self.len != 0 {
            let next = self.len.checked_mul(self.width)?.checked_add(*offset)?;
            if next > end {
                return None;
            }
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
