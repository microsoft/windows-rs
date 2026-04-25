use super::*;

const SECTION_ALIGNMENT: u32 = 4096;

#[repr(C)]
#[derive(Default)]
struct METADATA_HEADER {
    signature: u32,
    major_version: u16,
    minor_version: u16,
    reserved: u32,
    length: u32,
    version: [u8; 20],
    flags: u16,
    streams: u16,
}

const METADATA_SIGNATURE: u32 = 0x424A_5342;

#[repr(C)]
struct STREAM_HEADER<const LEN: usize> {
    offset: u32,
    size: u32,
    name: [u8; LEN],
}

impl<const LEN: usize> STREAM_HEADER<LEN> {
    fn new(offset: u32, size: u32, name: &[u8; LEN]) -> Self {
        Self {
            offset,
            size,
            name: *name,
        }
    }
    fn next_offset(&self) -> u32 {
        self.offset + self.size
    }
}

impl File {
    pub fn into_stream(mut self) -> Vec<u8> {
        // Sort TypeDefs for reproducible builds...

        self.sort_type_defs();

        // Flatten sorted records...

        self.records.Constant.extend(self.Constant.values());

        self.records
            .Attribute
            .extend(self.Attribute.values().flatten());

        self.records
            .GenericParam
            .extend(self.GenericParam.values().flatten());

        // Test sorted order...

        debug_assert!(self
            .records
            .ClassLayout
            .iter()
            .map(|r| r.Parent)
            .is_sorted());

        debug_assert!(self.records.Constant.iter().map(|r| r.Parent).is_sorted());
        debug_assert!(self.records.Attribute.iter().map(|r| r.Parent).is_sorted());

        debug_assert!(self
            .records
            .GenericParam
            .iter()
            .map(|r| r.Owner)
            .is_sorted());

        debug_assert!(self
            .records
            .ImplMap
            .iter()
            .map(|r| r.MemberForwarded)
            .is_sorted());

        debug_assert!(self
            .records
            .NestedClass
            .iter()
            .map(|r| r.NestedClass)
            .is_sorted());

        // Serialize...

        let mut strings = self.strings.into_stream();
        let mut blobs = self.blobs.into_stream();
        let mut records = self.records.into_stream();

        if [records.len(), strings.len(), blobs.len()]
            .iter()
            .any(|len| *len > u32::MAX as usize)
        {
            panic!("heap too large");
        }

        let mut guids = vec![0u8; 16]; // zero guid
        let size_of_streams = records.len() + guids.len() + strings.len() + blobs.len();

        let dos = IMAGE_DOS_HEADER {
            e_magic: IMAGE_DOS_SIGNATURE,
            e_lfarlc: 64,
            e_lfanew: core::mem::size_of::<IMAGE_DOS_HEADER>() as i32,
            ..Default::default()
        };

        let file = IMAGE_FILE_HEADER {
            Machine: IMAGE_FILE_MACHINE_I386,
            NumberOfSections: 1,
            SizeOfOptionalHeader: core::mem::size_of::<IMAGE_OPTIONAL_HEADER32>() as u16,
            Characteristics: IMAGE_FILE_DLL
                | IMAGE_FILE_32BIT_MACHINE
                | IMAGE_FILE_EXECUTABLE_IMAGE,
            ..Default::default()
        };

        let mut optional = IMAGE_OPTIONAL_HEADER32 {
            Magic: IMAGE_NT_OPTIONAL_HDR32_MAGIC,
            MajorLinkerVersion: 11,
            SizeOfInitializedData: 1024,
            ImageBase: 0x400000,
            SectionAlignment: SECTION_ALIGNMENT,
            FileAlignment: 512,
            MajorOperatingSystemVersion: 6,
            MinorOperatingSystemVersion: 2,
            MajorSubsystemVersion: 6,
            MinorSubsystemVersion: 2,
            SizeOfHeaders: 512,
            Subsystem: IMAGE_SUBSYSTEM_WINDOWS_CUI,
            DllCharacteristics: IMAGE_DLLCHARACTERISTICS_NX_COMPAT
                | IMAGE_DLLCHARACTERISTICS_NO_SEH
                | IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE,
            SizeOfStackReserve: 0x100000,
            SizeOfHeapReserve: 4096,
            LoaderFlags: 0x100000,
            NumberOfRvaAndSizes: 16,
            ..Default::default()
        };

        let mut section = IMAGE_SECTION_HEADER {
            Name: *b".text\0\0\0",
            Characteristics: 0x4000_0020,
            VirtualAddress: SECTION_ALIGNMENT,
            ..Default::default()
        };

        let mut clr = IMAGE_COR20_HEADER {
            cb: core::mem::size_of::<IMAGE_COR20_HEADER>() as u32,
            MajorRuntimeVersion: 2,
            MinorRuntimeVersion: 5,
            Flags: 1,
            ..Default::default()
        };

        let metadata = METADATA_HEADER {
            signature: METADATA_SIGNATURE,
            major_version: 1,
            minor_version: 1,
            length: 20,
            version: *b"WindowsRuntime 1.4\0\0",
            streams: 4,
            ..Default::default()
        };

        type TablesHeader = STREAM_HEADER<4>;
        type StringsHeader = STREAM_HEADER<12>;
        type GuidsHeader = STREAM_HEADER<8>;
        type BlobsHeader = STREAM_HEADER<8>;

        let size_of_stream_headers = core::mem::size_of::<TablesHeader>()
            + core::mem::size_of::<StringsHeader>()
            + core::mem::size_of::<GuidsHeader>()
            + core::mem::size_of::<BlobsHeader>();

        let size_of_image = optional.FileAlignment as usize
            + core::mem::size_of::<IMAGE_COR20_HEADER>()
            + core::mem::size_of::<METADATA_HEADER>()
            + size_of_stream_headers
            + size_of_streams;

        let virtual_size = size_of_image as u32 - optional.FileAlignment;

        // Writing a union field is safe; the union variants (VirtualSize and PhysicalAddress)
        // are both u32, so any bit pattern is valid for either variant.
        section.Misc.VirtualSize = virtual_size;

        section.SizeOfRawData =
            round(virtual_size as usize, optional.FileAlignment as usize) as u32;

        optional.SizeOfImage = round(
            SECTION_ALIGNMENT as usize + section.SizeOfRawData as usize,
            optional.SectionAlignment as usize,
        ) as u32;

        optional.DataDirectory[IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR as usize] =
            IMAGE_DATA_DIRECTORY {
                VirtualAddress: SECTION_ALIGNMENT,
                Size: core::mem::size_of::<IMAGE_COR20_HEADER>() as u32,
            };

        section.PointerToRawData = optional.FileAlignment;

        clr.MetaData.VirtualAddress =
            SECTION_ALIGNMENT + core::mem::size_of::<IMAGE_COR20_HEADER>() as u32;

        clr.MetaData.Size = virtual_size - core::mem::size_of::<IMAGE_COR20_HEADER>() as u32;

        let mut buffer = Vec::<u8>::new();
        buffer.write_header(&dos);
        buffer.write_u32(IMAGE_NT_SIGNATURE);
        buffer.write_header(&file);
        buffer.write_header(&optional);
        buffer.write_header(&section);
        debug_assert!(buffer.len() < optional.FileAlignment as usize);
        buffer.resize(optional.FileAlignment as usize, 0);
        buffer.write_header(&clr);
        let metadata_offset = buffer.len();
        buffer.write_header(&metadata);

        let stream_offset = buffer.len() - metadata_offset + size_of_stream_headers;

        let tables_header =
            TablesHeader::new(stream_offset as u32, records.len() as u32, b"#~\0\0");

        let strings_header = StringsHeader::new(
            tables_header.next_offset(),
            strings.len() as u32,
            b"#Strings\0\0\0\0",
        );

        let guids_header = GuidsHeader::new(
            strings_header.next_offset(),
            guids.len() as u32,
            b"#GUID\0\0\0",
        );

        let blobs_header = BlobsHeader::new(
            guids_header.next_offset(),
            blobs.len() as u32,
            b"#Blob\0\0\0",
        );

        buffer.write_header(&tables_header);
        buffer.write_header(&strings_header);
        buffer.write_header(&guids_header);
        buffer.write_header(&blobs_header);

        buffer.append(&mut records);
        buffer.append(&mut strings);
        buffer.append(&mut guids);
        buffer.append(&mut blobs);

        let unpadded_size = buffer.len();
        buffer.resize(
            optional.FileAlignment as usize + section.SizeOfRawData as usize,
            0,
        );

        assert_eq!(clr.MetaData.Size as usize, unpadded_size - metadata_offset);
        assert_eq!(
            optional.FileAlignment as usize + section.SizeOfRawData as usize,
            buffer.len()
        );

        buffer
    }

    fn sort_type_defs(&mut self) {
        let n = self.records.TypeDef.len();

        // Nothing to sort if only <Module> is present.
        if n <= 1 {
            return;
        }

        // Build maps: nested_in[i] = parent index, children[outer] = [inner, ...]
        let mut nested_in: HashMap<usize, usize> = HashMap::new();
        let mut children: HashMap<usize, Vec<usize>> = HashMap::new();

        for nc in &self.records.NestedClass {
            let inner = nc.NestedClass as usize;
            let outer = nc.EnclosingClass as usize;
            nested_in.insert(inner, outer);
            children.entry(outer).or_default().push(inner);
        }

        // Build the TypeDef permutation:
        //   - <Module> (index 0) is always first
        //   - Top-level non-nested types sorted by (namespace, name)
        //   - Nested types follow their parent in DFS order, sorted by name within each parent
        let mut top_level: Vec<usize> = (1..n)
            .filter(|i| !nested_in.contains_key(i))
            .collect();

        top_level.sort_by(|&a, &b| self.typedef_sort_keys[a].cmp(&self.typedef_sort_keys[b]));

        let mut typedef_perm = vec![0usize]; // <Module> always first
        for &td in &top_level {
            Self::dfs_append(td, &children, &self.typedef_sort_keys, &mut typedef_perm);
        }

        debug_assert_eq!(typedef_perm.len(), n);

        // If already in sorted order, nothing to do.
        if typedef_perm.iter().enumerate().all(|(i, &j)| i == j) {
            return;
        }

        // Build old->new TypeDef remapping.
        let mut typedef_old_to_new = vec![0u32; n];
        for (new_pos, &old_pos) in typedef_perm.iter().enumerate() {
            typedef_old_to_new[old_pos] = new_pos as u32;
        }

        // Compute field and method ranges for each TypeDef (in old order).
        let n_field = self.records.Field.len();
        let n_method = self.records.MethodDef.len();
        let n_param = self.records.Param.len();

        let typedef_field_range: Vec<std::ops::Range<usize>> = (0..n)
            .map(|i| {
                let start = self.records.TypeDef[i].FieldList as usize;
                let end = if i + 1 < n {
                    self.records.TypeDef[i + 1].FieldList as usize
                } else {
                    n_field
                };
                start..end
            })
            .collect();

        let typedef_method_range: Vec<std::ops::Range<usize>> = (0..n)
            .map(|i| {
                let start = self.records.TypeDef[i].MethodList as usize;
                let end = if i + 1 < n {
                    self.records.TypeDef[i + 1].MethodList as usize
                } else {
                    n_method
                };
                start..end
            })
            .collect();

        // Build field and method permutations following the TypeDef permutation.
        let mut field_perm: Vec<usize> = Vec::with_capacity(n_field);
        let mut method_perm: Vec<usize> = Vec::with_capacity(n_method);

        for &td in &typedef_perm {
            field_perm.extend(typedef_field_range[td].clone());
            method_perm.extend(typedef_method_range[td].clone());
        }

        // Build param permutation following the method permutation.
        let method_param_range: Vec<std::ops::Range<usize>> = (0..n_method)
            .map(|i| {
                let start = self.records.MethodDef[i].ParamList as usize;
                let end = if i + 1 < n_method {
                    self.records.MethodDef[i + 1].ParamList as usize
                } else {
                    n_param
                };
                start..end
            })
            .collect();

        let mut param_perm: Vec<usize> = Vec::with_capacity(n_param);
        for &method in &method_perm {
            param_perm.extend(method_param_range[method].clone());
        }

        // Build reverse (old->new) mappings.
        let mut field_old_to_new = vec![0u32; n_field];
        for (new_pos, &old_pos) in field_perm.iter().enumerate() {
            field_old_to_new[old_pos] = new_pos as u32;
        }

        let mut method_old_to_new = vec![0u32; n_method];
        for (new_pos, &old_pos) in method_perm.iter().enumerate() {
            method_old_to_new[old_pos] = new_pos as u32;
        }

        let mut param_old_to_new = vec![0u32; n_param];
        for (new_pos, &old_pos) in param_perm.iter().enumerate() {
            param_old_to_new[old_pos] = new_pos as u32;
        }

        // Reorder TypeDef records, updating FieldList and MethodList.
        let old_typedefs = std::mem::take(&mut self.records.TypeDef);
        let mut field_offset = 0u32;
        let mut method_offset = 0u32;
        self.records.TypeDef = typedef_perm
            .iter()
            .map(|&old| {
                let mut r = old_typedefs[old];
                r.FieldList = field_offset;
                r.MethodList = method_offset;
                field_offset += typedef_field_range[old].len() as u32;
                method_offset += typedef_method_range[old].len() as u32;
                r
            })
            .collect();

        // Also reorder typedef_sort_keys to match.
        let old_sort_keys = std::mem::take(&mut self.typedef_sort_keys);
        self.typedef_sort_keys = typedef_perm.iter().map(|&i| old_sort_keys[i].clone()).collect();

        // Reorder Field records.
        let old_fields = std::mem::take(&mut self.records.Field);
        self.records.Field = field_perm.iter().map(|&i| old_fields[i]).collect();

        // Reorder MethodDef records, updating ParamList.
        let old_methods = std::mem::take(&mut self.records.MethodDef);
        let mut param_offset = 0u32;
        self.records.MethodDef = method_perm
            .iter()
            .map(|&old| {
                let mut r = old_methods[old];
                r.ParamList = param_offset;
                param_offset += method_param_range[old].len() as u32;
                r
            })
            .collect();

        // Reorder Param records.
        let old_params = std::mem::take(&mut self.records.Param);
        self.records.Param = param_perm.iter().map(|&i| old_params[i]).collect();

        // Update ClassLayout.Parent and re-sort by Parent.
        for r in &mut self.records.ClassLayout {
            r.Parent = typedef_old_to_new[r.Parent as usize];
        }
        self.records.ClassLayout.sort_by_key(|r| r.Parent);

        // Update NestedClass and re-sort by NestedClass.
        for r in &mut self.records.NestedClass {
            r.NestedClass = typedef_old_to_new[r.NestedClass as usize];
            r.EnclosingClass = typedef_old_to_new[r.EnclosingClass as usize];
        }
        self.records.NestedClass.sort_by_key(|r| r.NestedClass);

        // Update InterfaceImpl.Class, sort by Class, and build InterfaceImpl remap.
        for r in &mut self.records.InterfaceImpl {
            r.Class = id::TypeDef(typedef_old_to_new[r.Class.0 as usize]);
        }
        let n_ii = self.records.InterfaceImpl.len();
        let mut ii_perm: Vec<usize> = (0..n_ii).collect();
        ii_perm.sort_by_key(|&i| self.records.InterfaceImpl[i].Class.0);
        let mut ii_old_to_new = vec![0u32; n_ii];
        for (new_pos, &old_pos) in ii_perm.iter().enumerate() {
            ii_old_to_new[old_pos] = new_pos as u32;
        }
        let old_ii = std::mem::take(&mut self.records.InterfaceImpl);
        self.records.InterfaceImpl = ii_perm.iter().map(|&i| old_ii[i]).collect();

        // Update FieldLayout.Field and re-sort by Field.
        for r in &mut self.records.FieldLayout {
            r.Field = field_old_to_new[r.Field as usize];
        }
        self.records.FieldLayout.sort_by_key(|r| r.Field);

        // Update ImplMap.MemberForwarded and re-sort.
        for r in &mut self.records.ImplMap {
            let MemberForwarded::MethodDef(ref mut m) = r.MemberForwarded;
            m.0 = method_old_to_new[m.0 as usize];
        }
        self.records.ImplMap.sort_by_key(|r| r.MemberForwarded);

        // Rebuild Constant staging with remapped Field keys. Both the BTreeMap key and the
        // Parent field within each rec::Constant record must be updated.
        let old_constant = std::mem::take(&mut self.Constant);
        for (key, value) in old_constant {
            let new_key = match key {
                HasConstant::Field(f) => HasConstant::Field(id::Field(field_old_to_new[f.0 as usize])),
            };
            let mut remapped = value;
            remapped.Parent = new_key;
            self.Constant.insert(new_key, remapped);
        }

        // Rebuild Attribute staging with remapped keys. Both the BTreeMap key and the
        // Parent field within each rec::Attribute record must be updated.
        let old_attribute = std::mem::take(&mut self.Attribute);
        for (key, value) in old_attribute {
            let new_key = match key {
                HasAttribute::TypeDef(t) => HasAttribute::TypeDef(id::TypeDef(typedef_old_to_new[t.0 as usize])),
                HasAttribute::Field(f) => HasAttribute::Field(id::Field(field_old_to_new[f.0 as usize])),
                HasAttribute::MethodDef(m) => HasAttribute::MethodDef(id::MethodDef(method_old_to_new[m.0 as usize])),
                HasAttribute::Param(p) => HasAttribute::Param(id::Param(param_old_to_new[p.0 as usize])),
                HasAttribute::InterfaceImpl(ii) => HasAttribute::InterfaceImpl(id::InterfaceImpl(ii_old_to_new[ii.0 as usize])),
                other => other,
            };
            let remapped: Vec<rec::Attribute> = value
                .into_iter()
                .map(|mut a| {
                    a.Parent = new_key;
                    a
                })
                .collect();
            self.Attribute.entry(new_key).or_default().extend(remapped);
        }

        // Rebuild GenericParam staging with remapped TypeDef keys. Both the BTreeMap key
        // and the Owner field within each rec::GenericParam record must be updated.
        let old_generic = std::mem::take(&mut self.GenericParam);
        for (key, value) in old_generic {
            let new_key = match key {
                TypeOrMethodDef::TypeDef(t) => TypeOrMethodDef::TypeDef(id::TypeDef(typedef_old_to_new[t.0 as usize])),
            };
            let remapped: Vec<rec::GenericParam> = value
                .into_iter()
                .map(|mut g| {
                    g.Owner = new_key;
                    g
                })
                .collect();
            self.GenericParam.entry(new_key).or_default().extend(remapped);
        }
    }

    fn dfs_append(
        td: usize,
        children: &HashMap<usize, Vec<usize>>,
        sort_keys: &[(String, String)],
        result: &mut Vec<usize>,
    ) {
        result.push(td);
        if let Some(kids) = children.get(&td) {
            let mut indices: Vec<usize> = (0..kids.len()).collect();
            indices.sort_by(|&a, &b| sort_keys[kids[a]].1.cmp(&sort_keys[kids[b]].1));
            for i in indices {
                Self::dfs_append(kids[i], children, sort_keys, result);
            }
        }
    }
}
