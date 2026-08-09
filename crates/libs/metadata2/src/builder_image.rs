use super::*;

pub(super) fn metadata_image(
    tables: &[u8],
    strings: &[u8],
    guids: &[u8],
    blobs: &[u8],
) -> Result<Vec<u8>, BuildError> {
    let names: [&[u8]; 4] = [b"#~", b"#Strings", b"#GUID", b"#Blob"];
    let streams = [tables, strings, guids, blobs];
    let header_size = 40
        + names
            .iter()
            .map(|name| 8 + align(name.len() + 1, 4))
            .sum::<usize>();
    let mut metadata = Vec::new();
    push_u32(&mut metadata, 0x424a_5342);
    push_u16(&mut metadata, 1);
    push_u16(&mut metadata, 1);
    push_u32(&mut metadata, 0);
    push_u32(&mut metadata, 20);
    metadata.extend(b"WindowsRuntime 1.4\0\0");
    push_u16(&mut metadata, 0);
    push_u16(&mut metadata, 4);
    let mut offset = header_size;
    for (name, stream) in names.into_iter().zip(streams) {
        push_u32(
            &mut metadata,
            offset
                .try_into()
                .map_err(|_| BuildError::new("metadata stream offset overflow"))?,
        );
        push_u32(
            &mut metadata,
            stream
                .len()
                .try_into()
                .map_err(|_| BuildError::new("metadata stream size overflow"))?,
        );
        metadata.extend(name);
        metadata.push(0);
        metadata.resize(align(metadata.len(), 4), 0);
        offset += stream.len();
    }
    for stream in streams {
        metadata.extend(stream);
    }
    pe_image(&metadata)
}

fn pe_image(metadata: &[u8]) -> Result<Vec<u8>, BuildError> {
    const FILE_ALIGNMENT: usize = 0x200;
    const SECTION_ALIGNMENT: u32 = 0x1000;
    const CLI_SIZE: usize = 72;
    let virtual_size = CLI_SIZE
        .checked_add(metadata.len())
        .ok_or(BuildError::new("metadata image size overflow"))?;
    let raw_size = align(virtual_size, FILE_ALIGNMENT);
    let size_of_image = SECTION_ALIGNMENT
        .checked_add(
            align(raw_size, SECTION_ALIGNMENT as usize)
                .try_into()
                .map_err(|_| BuildError::new("metadata image size overflow"))?,
        )
        .ok_or(BuildError::new("metadata image size overflow"))?;

    let mut bytes = vec![0; FILE_ALIGNMENT + raw_size];
    put_u16(&mut bytes, 0, 0x5a4d);
    put_u32(&mut bytes, 60, 64);
    put_u32(&mut bytes, 64, 0x0000_4550);
    let coff = 68;
    put_u16(&mut bytes, coff, 0x014c);
    put_u16(&mut bytes, coff + 2, 1);
    put_u16(&mut bytes, coff + 16, 224);
    put_u16(&mut bytes, coff + 18, 0x2102);

    let optional = coff + 20;
    put_u16(&mut bytes, optional, 0x010b);
    put_u32(&mut bytes, optional + 8, raw_size as u32);
    put_u32(&mut bytes, optional + 28, 0x0040_0000);
    put_u32(&mut bytes, optional + 32, SECTION_ALIGNMENT);
    put_u32(&mut bytes, optional + 36, FILE_ALIGNMENT as u32);
    put_u16(&mut bytes, optional + 40, 6);
    put_u16(&mut bytes, optional + 48, 6);
    put_u32(&mut bytes, optional + 56, size_of_image);
    put_u32(&mut bytes, optional + 60, FILE_ALIGNMENT as u32);
    put_u16(&mut bytes, optional + 68, 3);
    put_u16(&mut bytes, optional + 70, 0x0540);
    put_u32(&mut bytes, optional + 72, 0x0010_0000);
    put_u32(&mut bytes, optional + 76, 0x1000);
    put_u32(&mut bytes, optional + 80, 0x0010_0000);
    put_u32(&mut bytes, optional + 84, 0x1000);
    put_u32(&mut bytes, optional + 92, 16);
    let cli_directory = optional + 96 + 14 * 8;
    put_u32(&mut bytes, cli_directory, SECTION_ALIGNMENT);
    put_u32(&mut bytes, cli_directory + 4, CLI_SIZE as u32);

    let section = optional + 224;
    bytes[section..section + 8].copy_from_slice(b".text\0\0\0");
    put_u32(&mut bytes, section + 8, virtual_size as u32);
    put_u32(&mut bytes, section + 12, SECTION_ALIGNMENT);
    put_u32(&mut bytes, section + 16, raw_size as u32);
    put_u32(&mut bytes, section + 20, FILE_ALIGNMENT as u32);
    put_u32(&mut bytes, section + 36, 0x4000_0020);

    let cli = FILE_ALIGNMENT;
    put_u32(&mut bytes, cli, CLI_SIZE as u32);
    put_u16(&mut bytes, cli + 4, 2);
    put_u16(&mut bytes, cli + 6, 5);
    put_u32(&mut bytes, cli + 8, SECTION_ALIGNMENT + CLI_SIZE as u32);
    put_u32(&mut bytes, cli + 12, metadata.len() as u32);
    put_u32(&mut bytes, cli + 16, 1);
    bytes[cli + CLI_SIZE..cli + CLI_SIZE + metadata.len()].copy_from_slice(metadata);
    Ok(bytes)
}

fn push_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend(value.to_le_bytes());
}

fn push_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend(value.to_le_bytes());
}

fn put_u16(bytes: &mut [u8], offset: usize, value: u16) {
    bytes[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
}

fn put_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

const fn align(value: usize, alignment: usize) -> usize {
    (value + alignment - 1) & !(alignment - 1)
}
