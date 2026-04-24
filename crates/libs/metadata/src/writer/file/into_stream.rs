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
}
