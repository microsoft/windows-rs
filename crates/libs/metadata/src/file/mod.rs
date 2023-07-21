mod reader;
mod table;
mod view;
use super::*;
pub use reader::RowReader;
use std::cmp::Ordering;
use table::Table;
use view::View;
type Result<T> = std::result::Result<T, ()>;

#[derive(Default)]
pub struct File {
    bytes: Vec<u8>,
    strings: usize,
    blobs: usize,
    tables: [Table; 17],
}

impl File {
    pub fn new(bytes: Vec<u8>) -> Option<Self> {
        Self::ok(bytes).ok()
    }

    fn ok(bytes: Vec<u8>) -> Result<Self> {
        let mut result = File { bytes, ..Default::default() };

        let dos = result.bytes.view_as::<IMAGE_DOS_HEADER>(0)?;

        if dos.e_magic != IMAGE_DOS_SIGNATURE || result.bytes.copy_as::<u32>(dos.e_lfanew as usize)? != IMAGE_NT_SIGNATURE {
            return Err(());
        }

        let file_offset = dos.e_lfanew as usize + std::mem::size_of::<u32>();
        let file = result.bytes.view_as::<IMAGE_FILE_HEADER>(file_offset)?;

        let optional_offset = file_offset + std::mem::size_of::<IMAGE_FILE_HEADER>();

        let (com_virtual_address, sections) = match result.bytes.copy_as::<u16>(optional_offset)? {
            IMAGE_NT_OPTIONAL_HDR32_MAGIC => {
                let optional = result.bytes.view_as::<IMAGE_OPTIONAL_HEADER32>(optional_offset)?;
                (optional.DataDirectory[IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR as usize].VirtualAddress, result.bytes.view_as_slice_of::<IMAGE_SECTION_HEADER>(optional_offset + std::mem::size_of::<IMAGE_OPTIONAL_HEADER32>(), file.NumberOfSections as usize)?)
            }
            IMAGE_NT_OPTIONAL_HDR64_MAGIC => {
                let optional = result.bytes.view_as::<IMAGE_OPTIONAL_HEADER64>(optional_offset)?;
                (optional.DataDirectory[IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR as usize].VirtualAddress, result.bytes.view_as_slice_of::<IMAGE_SECTION_HEADER>(optional_offset + std::mem::size_of::<IMAGE_OPTIONAL_HEADER64>(), file.NumberOfSections as usize)?)
            }
            _ => return Err(()),
        };

        let clr = result.bytes.view_as::<IMAGE_COR20_HEADER>(offset_from_rva(section_from_rva(sections, com_virtual_address)?, com_virtual_address))?;

        if clr.cb != std::mem::size_of::<IMAGE_COR20_HEADER>() as u32 {
            return Err(());
        }

        let metadata_offset = offset_from_rva(section_from_rva(sections, clr.MetaData.VirtualAddress)?, clr.MetaData.VirtualAddress);
        let metadata = result.bytes.view_as::<METADATA_HEADER>(metadata_offset)?;

        if metadata.signature != METADATA_SIGNATURE {
            return Err(());
        }

        // The METADATA_HEADER struct is not a fixed size so have to offset a little more carefully.
        let mut view = metadata_offset + metadata.length as usize + 20;
        let mut tables_data: (usize, usize) = (0, 0);

        for _ in 0..result.bytes.copy_as::<u16>(metadata_offset + metadata.length as usize + 18)? {
            let stream_offset = result.bytes.copy_as::<u32>(view)? as usize;
            let stream_len = result.bytes.copy_as::<u32>(view + 4)? as usize;
            let stream_name = result.bytes.view_as_str(view + 8)?;
            match stream_name {
                b"#Strings" => result.strings = metadata_offset + stream_offset,
                b"#Blob" => result.blobs = metadata_offset + stream_offset,
                b"#~" => tables_data = (metadata_offset + stream_offset, stream_len),
                b"#GUID" => {}
                b"#US" => {}
                rest => unimplemented!("{rest:?}"),
            }
            let mut padding = 4 - stream_name.len() % 4;
            if padding == 0 {
                padding = 4;
            }
            view += 8 + stream_name.len() + padding;
        }

        let heap_sizes = result.bytes.copy_as::<u8>(tables_data.0 + 6)?;
        let string_index_size = if (heap_sizes & 1) == 1 { 4 } else { 2 };
        let guid_index_size = if (heap_sizes >> 1 & 1) == 1 { 4 } else { 2 };
        let blob_index_size = if (heap_sizes >> 2 & 1) == 1 { 4 } else { 2 };
        let valid_bits = result.bytes.copy_as::<u64>(tables_data.0 + 8)?;
        view = tables_data.0 + 24;

        // These tables are unused by the reader, but needed temporarily to calculate sizes and offsets for subsequent tables.
        let unused_empty = Table::default();
        let mut unused_assembly = Table::default();
        let mut unused_assembly_os = Table::default();
        let mut unused_assembly_processor = Table::default();
        let mut unused_assembly_ref_os = Table::default();
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

        for i in 0..64 {
            if (valid_bits >> i & 1) == 0 {
                continue;
            }

            let len = result.bytes.copy_as::<u32>(view)? as usize;
            view += 4;

            match i {
                0x00 => result.tables[Module::TABLE].len = len,
                0x01 => result.tables[TypeRef::TABLE].len = len,
                0x02 => result.tables[TypeDef::TABLE].len = len,
                0x04 => result.tables[Field::TABLE].len = len,
                0x06 => result.tables[MethodDef::TABLE].len = len,
                0x08 => result.tables[Param::TABLE].len = len,
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
                0x20 => unused_assembly.len = len,
                0x21 => unused_assembly_processor.len = len,
                0x22 => unused_assembly_os.len = len,
                0x23 => result.tables[AssemblyRef::TABLE].len = len,
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
        let type_def_or_ref = coded_index_size(&[tables[TypeDef::TABLE].len, tables[TypeRef::TABLE].len, tables[TypeSpec::TABLE].len]);
        let has_constant = coded_index_size(&[tables[Field::TABLE].len, tables[Param::TABLE].len, unused_property.len]);
        let has_field_marshal = coded_index_size(&[tables[Field::TABLE].len, tables[Param::TABLE].len]);
        let has_decl_security = coded_index_size(&[tables[TypeDef::TABLE].len, tables[MethodDef::TABLE].len, unused_assembly.len]);
        let member_ref_parent = coded_index_size(&[tables[TypeDef::TABLE].len, tables[TypeRef::TABLE].len, tables[ModuleRef::TABLE].len, tables[MethodDef::TABLE].len, tables[TypeSpec::TABLE].len]);
        let has_semantics = coded_index_size(&[unused_event.len, unused_property.len]);
        let method_def_or_ref = coded_index_size(&[tables[MethodDef::TABLE].len, tables[MemberRef::TABLE].len]);
        let member_forwarded = coded_index_size(&[tables[Field::TABLE].len, tables[MethodDef::TABLE].len]);
        let implementation = coded_index_size(&[unused_file.len, tables[AssemblyRef::TABLE].len, unused_exported_type.len]);
        let custom_attribute_type = coded_index_size(&[tables[MethodDef::TABLE].len, tables[MemberRef::TABLE].len, unused_empty.len, unused_empty.len, unused_empty.len]);
        let resolution_scope = coded_index_size(&[tables[Module::TABLE].len, tables[ModuleRef::TABLE].len, tables[AssemblyRef::TABLE].len, tables[TypeRef::TABLE].len]);
        let type_or_method_def = coded_index_size(&[tables[TypeDef::TABLE].len, tables[MethodDef::TABLE].len]);

        let has_custom_attribute = coded_index_size(&[
            tables[MethodDef::TABLE].len,
            tables[Field::TABLE].len,
            tables[TypeRef::TABLE].len,
            tables[TypeDef::TABLE].len,
            tables[Param::TABLE].len,
            tables[InterfaceImpl::TABLE].len,
            tables[MemberRef::TABLE].len,
            tables[Module::TABLE].len,
            unused_property.len,
            unused_event.len,
            unused_standalone_sig.len,
            tables[ModuleRef::TABLE].len,
            tables[TypeSpec::TABLE].len,
            unused_assembly.len,
            tables[AssemblyRef::TABLE].len,
            unused_file.len,
            unused_exported_type.len,
            unused_manifest_resource.len,
            tables[GenericParam::TABLE].len,
            unused_generic_param_constraint.len,
            unused_method_spec.len,
        ]);

        unused_assembly.set_columns(4, 8, 4, blob_index_size, string_index_size, string_index_size);
        unused_assembly_os.set_columns(4, 4, 4, 0, 0, 0);
        unused_assembly_processor.set_columns(4, 0, 0, 0, 0, 0);
        result.tables[AssemblyRef::TABLE].set_columns(8, 4, blob_index_size, string_index_size, string_index_size, blob_index_size);
        unused_assembly_ref_os.set_columns(4, 4, 4, result.tables[AssemblyRef::TABLE].index_width(), 0, 0);
        unused_assembly_ref_processor.set_columns(4, result.tables[AssemblyRef::TABLE].index_width(), 0, 0, 0, 0);
        result.tables[ClassLayout::TABLE].set_columns(2, 4, result.tables[TypeDef::TABLE].index_width(), 0, 0, 0);
        result.tables[Constant::TABLE].set_columns(2, has_constant, blob_index_size, 0, 0, 0);
        result.tables[Attribute::TABLE].set_columns(has_custom_attribute, custom_attribute_type, blob_index_size, 0, 0, 0);
        unused_decl_security.set_columns(2, has_decl_security, blob_index_size, 0, 0, 0);
        unused_event_map.set_columns(result.tables[TypeDef::TABLE].index_width(), unused_event.index_width(), 0, 0, 0, 0);
        unused_event.set_columns(2, string_index_size, type_def_or_ref, 0, 0, 0);
        unused_exported_type.set_columns(4, 4, string_index_size, string_index_size, implementation, 0);
        result.tables[Field::TABLE].set_columns(2, string_index_size, blob_index_size, 0, 0, 0);
        unused_field_layout.set_columns(4, result.tables[Field::TABLE].index_width(), 0, 0, 0, 0);
        unused_field_marshal.set_columns(has_field_marshal, blob_index_size, 0, 0, 0, 0);
        unused_field_rva.set_columns(4, result.tables[Field::TABLE].index_width(), 0, 0, 0, 0);
        unused_file.set_columns(4, string_index_size, blob_index_size, 0, 0, 0);
        result.tables[GenericParam::TABLE].set_columns(2, 2, type_or_method_def, string_index_size, 0, 0);
        unused_generic_param_constraint.set_columns(result.tables[GenericParam::TABLE].index_width(), type_def_or_ref, 0, 0, 0, 0);
        result.tables[ImplMap::TABLE].set_columns(2, member_forwarded, string_index_size, result.tables[ModuleRef::TABLE].index_width(), 0, 0);
        result.tables[InterfaceImpl::TABLE].set_columns(result.tables[TypeDef::TABLE].index_width(), type_def_or_ref, 0, 0, 0, 0);
        unused_manifest_resource.set_columns(4, 4, string_index_size, implementation, 0, 0);
        result.tables[MemberRef::TABLE].set_columns(member_ref_parent, string_index_size, blob_index_size, 0, 0, 0);
        result.tables[MethodDef::TABLE].set_columns(4, 2, 2, string_index_size, blob_index_size, result.tables[Param::TABLE].index_width());
        unused_method_impl.set_columns(result.tables[TypeDef::TABLE].index_width(), method_def_or_ref, method_def_or_ref, 0, 0, 0);
        unused_method_semantics.set_columns(2, result.tables[MethodDef::TABLE].index_width(), has_semantics, 0, 0, 0);
        unused_method_spec.set_columns(method_def_or_ref, blob_index_size, 0, 0, 0, 0);
        result.tables[Module::TABLE].set_columns(2, string_index_size, guid_index_size, guid_index_size, guid_index_size, 0);
        result.tables[ModuleRef::TABLE].set_columns(string_index_size, 0, 0, 0, 0, 0);
        result.tables[NestedClass::TABLE].set_columns(result.tables[TypeDef::TABLE].index_width(), result.tables[TypeDef::TABLE].index_width(), 0, 0, 0, 0);
        result.tables[Param::TABLE].set_columns(2, 2, string_index_size, 0, 0, 0);
        unused_property.set_columns(2, string_index_size, blob_index_size, 0, 0, 0);
        unused_property_map.set_columns(result.tables[TypeDef::TABLE].index_width(), unused_property.index_width(), 0, 0, 0, 0);
        unused_standalone_sig.set_columns(blob_index_size, 0, 0, 0, 0, 0);
        result.tables[TypeDef::TABLE].set_columns(4, string_index_size, string_index_size, type_def_or_ref, result.tables[Field::TABLE].index_width(), result.tables[MethodDef::TABLE].index_width());
        result.tables[TypeRef::TABLE].set_columns(resolution_scope, string_index_size, string_index_size, 0, 0, 0);
        result.tables[TypeSpec::TABLE].set_columns(blob_index_size, 0, 0, 0, 0, 0);

        result.tables[Module::TABLE].set_data(&mut view);
        result.tables[TypeRef::TABLE].set_data(&mut view);
        result.tables[TypeDef::TABLE].set_data(&mut view);
        result.tables[Field::TABLE].set_data(&mut view);
        result.tables[MethodDef::TABLE].set_data(&mut view);
        result.tables[Param::TABLE].set_data(&mut view);
        result.tables[InterfaceImpl::TABLE].set_data(&mut view);
        result.tables[MemberRef::TABLE].set_data(&mut view);
        result.tables[Constant::TABLE].set_data(&mut view);
        result.tables[Attribute::TABLE].set_data(&mut view);
        unused_field_marshal.set_data(&mut view);
        unused_decl_security.set_data(&mut view);
        result.tables[ClassLayout::TABLE].set_data(&mut view);
        unused_field_layout.set_data(&mut view);
        unused_standalone_sig.set_data(&mut view);
        unused_event_map.set_data(&mut view);
        unused_event.set_data(&mut view);
        unused_property_map.set_data(&mut view);
        unused_property.set_data(&mut view);
        unused_method_semantics.set_data(&mut view);
        unused_method_impl.set_data(&mut view);
        result.tables[ModuleRef::TABLE].set_data(&mut view);
        result.tables[TypeSpec::TABLE].set_data(&mut view);
        result.tables[ImplMap::TABLE].set_data(&mut view);
        unused_field_rva.set_data(&mut view);
        unused_assembly.set_data(&mut view);
        unused_assembly_processor.set_data(&mut view);
        unused_assembly_os.set_data(&mut view);
        result.tables[AssemblyRef::TABLE].set_data(&mut view);
        unused_assembly_ref_processor.set_data(&mut view);
        unused_assembly_ref_os.set_data(&mut view);
        unused_file.set_data(&mut view);
        unused_exported_type.set_data(&mut view);
        unused_manifest_resource.set_data(&mut view);
        result.tables[NestedClass::TABLE].set_data(&mut view);
        result.tables[GenericParam::TABLE].set_data(&mut view);

        Ok(result)
    }

    fn usize(&self, row: usize, table: usize, column: usize) -> usize {
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

    fn lower_bound_of(&self, table: usize, mut first: usize, last: usize, column: usize, value: usize) -> usize {
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

    fn upper_bound_of(&self, table: usize, mut first: usize, last: usize, column: usize, value: usize) -> usize {
        let mut count = last - first;
        while count > 0 {
            let count2 = count / 2;
            let middle = first + count2;
            if value < self.usize(middle, table, column) {
                count = count2
            } else {
                first = middle + 1;
                count -= count2 + 1;
            }
        }
        first
    }

    pub fn table<R: AsRow>(&self, file: usize) -> RowIterator<R> {
        RowIterator::new(file, 0..self.tables[R::TABLE].len)
    }
}

fn section_from_rva(sections: &[IMAGE_SECTION_HEADER], rva: u32) -> Result<&IMAGE_SECTION_HEADER> {
    sections.iter().find(|&s| rva >= s.VirtualAddress && rva < s.VirtualAddress + unsafe { s.Misc.VirtualSize }).ok_or(())
}

fn offset_from_rva(section: &IMAGE_SECTION_HEADER, rva: u32) -> usize {
    (rva - section.VirtualAddress + section.PointerToRawData) as usize
}
