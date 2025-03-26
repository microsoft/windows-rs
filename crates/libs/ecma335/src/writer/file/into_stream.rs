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

        unsafe {
            let mut guids = vec![0; 16]; // zero guid
            let size_of_streams = records.len() + guids.len() + strings.len() + blobs.len();

            let mut dos: IMAGE_DOS_HEADER = core::mem::zeroed();
            dos.e_magic = IMAGE_DOS_SIGNATURE;
            dos.e_lfarlc = 64;
            dos.e_lfanew = core::mem::size_of::<IMAGE_DOS_HEADER>() as i32;

            let mut file: IMAGE_FILE_HEADER = core::mem::zeroed();
            file.Machine = IMAGE_FILE_MACHINE_I386;
            file.NumberOfSections = 1;
            file.SizeOfOptionalHeader = core::mem::size_of::<IMAGE_OPTIONAL_HEADER32>() as u16;
            file.Characteristics =
                IMAGE_FILE_DLL | IMAGE_FILE_32BIT_MACHINE | IMAGE_FILE_EXECUTABLE_IMAGE;

            let mut optional: IMAGE_OPTIONAL_HEADER32 = core::mem::zeroed();
            optional.Magic = IMAGE_NT_OPTIONAL_HDR32_MAGIC;
            optional.MajorLinkerVersion = 11;
            optional.SizeOfInitializedData = 1024;
            optional.ImageBase = 0x400000;
            optional.SectionAlignment = SECTION_ALIGNMENT;
            optional.FileAlignment = 512;
            optional.MajorOperatingSystemVersion = 6;
            optional.MinorOperatingSystemVersion = 2;
            optional.MajorSubsystemVersion = 6;
            optional.MinorSubsystemVersion = 2;
            optional.SizeOfHeaders = 512;
            optional.Subsystem = IMAGE_SUBSYSTEM_WINDOWS_CUI;
            optional.DllCharacteristics = IMAGE_DLLCHARACTERISTICS_NX_COMPAT
                | IMAGE_DLLCHARACTERISTICS_NO_SEH
                | IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE;
            optional.SizeOfStackReserve = 0x100000;
            optional.SizeOfHeapReserve = 4096;
            optional.LoaderFlags = 0x100000;
            optional.NumberOfRvaAndSizes = 16;

            let mut section: IMAGE_SECTION_HEADER = core::mem::zeroed();
            section.Name = *b".text\0\0\0";
            section.Characteristics = 0x4000_0020;
            section.VirtualAddress = SECTION_ALIGNMENT;

            let mut clr: IMAGE_COR20_HEADER = core::mem::zeroed();
            clr.cb = core::mem::size_of::<IMAGE_COR20_HEADER>() as u32;
            clr.MajorRuntimeVersion = 2;
            clr.MinorRuntimeVersion = 5;
            clr.Flags = 1;

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

            optional.SizeOfImage = round(size_of_image, optional.SectionAlignment as usize) as u32;
            section.Misc.VirtualSize = size_of_image as u32 - optional.FileAlignment;

            section.SizeOfRawData = round(
                section.Misc.VirtualSize as usize,
                optional.FileAlignment as usize,
            ) as u32;

            optional.DataDirectory[14] = IMAGE_DATA_DIRECTORY {
                VirtualAddress: SECTION_ALIGNMENT,
                Size: core::mem::size_of::<IMAGE_COR20_HEADER>() as u32,
            };

            section.PointerToRawData = optional.FileAlignment;

            clr.MetaData.VirtualAddress =
                SECTION_ALIGNMENT + core::mem::size_of::<IMAGE_COR20_HEADER>() as u32;

            clr.MetaData.Size =
                section.Misc.VirtualSize - core::mem::size_of::<IMAGE_COR20_HEADER>() as u32;

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

            assert_eq!(clr.MetaData.Size as usize, buffer.len() - metadata_offset);
            assert_eq!(size_of_image, buffer.len());

            buffer
        }
    }
}
