use crate::*;

pub(crate) struct Tables {}

impl Tables {
    pub fn new() -> Self {
        Self {}
    }

    pub fn into_stream(&self, strings: &mut Strings) -> Vec<u8> {
        let mut buffer = Vec::new();
        let header = Header::new();
        buffer.write(&header);

        // row sizes
        buffer.write(&1u32); // Module
        buffer.write(&1u32); // AssemblyRef

        Module::write(&mut buffer);
        AssemblyRef::write(&mut buffer, strings);

        buffer.resize(round(buffer.len(), 4), 0);
        buffer
    }
}

#[repr(C)]
#[derive(Default)]
struct Header {
    reserved1: u32,
    major_version: u8,
    minor_version: u8,
    heap_sizes: u8,
    reserved2: u8,
    valid: u64,
    sorted: u64,
}

impl Header {
    fn new() -> Self {
        Self {
            major_version: 2,
            reserved2: 1,
            heap_sizes: 0b111, // 4 byte indexes
            valid: Module::ID | AssemblyRef::ID,
            ..Default::default()
        }
    }
}

struct Module;

impl Module {
    const ID: u64 =  1 << 0;

    fn write(buffer: &mut Vec<u8>) {
        buffer.write(&0u16); // Generation (reserved)
        buffer.write(&0u32); // Name (none)
        buffer.write(&1u32); // Mvid (zero guid)
        buffer.write(&0u32); // EncId (reserved)
        buffer.write(&0u32); // EncBaseId (reserved)
    }
}

struct AssemblyRef;

impl AssemblyRef {
    const ID: u64 = 1 << 0x23;

    fn write(buffer: &mut Vec<u8>, strings: &mut Strings) {
        // TODO: This is only required by ILSpy https://github.com/icsharpcode/ILSpy/issues/2657
        buffer.write(&0u16); // MajorVersion
        buffer.write(&0u16); // MinorVersion
        buffer.write(&0u16); // BuildNumber
        buffer.write(&0u16); // RevisionNumber
        buffer.write(&0u32); // Flags (none)
        buffer.write(&0u32); // PublicKeyOrToken (none)
        buffer.write(&strings.insert("mscorlib"));
        buffer.write(&0u32); // Culture (none)
        buffer.write(&0u32); // HashValue (none)
    }
}
