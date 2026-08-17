use windows_metadata::reader::File;

fn optional_header(bytes: &[u8]) -> usize {
    let pe = u32::from_le_bytes(bytes[0x3c..0x40].try_into().unwrap()) as usize;
    pe + 4 + 20
}

fn u16_at(bytes: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes(bytes[offset..offset + 2].try_into().unwrap())
}

fn u32_at(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
}

fn rva_offset(bytes: &[u8], rva: u32) -> usize {
    let optional = optional_header(bytes);
    let coff = optional - 20;
    let sections = optional + u16_at(bytes, coff + 16) as usize;
    for section in 0..u16_at(bytes, coff + 2) as usize {
        let section = sections + section * 40;
        let virtual_size = u32_at(bytes, section + 8);
        let virtual_address = u32_at(bytes, section + 12);
        let raw_size = u32_at(bytes, section + 16);
        if rva >= virtual_address && rva < virtual_address + virtual_size.max(raw_size) {
            return (rva - virtual_address + u32_at(bytes, section + 20)) as usize;
        }
    }
    panic!()
}

fn cli_header(bytes: &[u8]) -> usize {
    let optional = optional_header(bytes);
    let directories = optional
        + match u16_at(bytes, optional) {
            0x10b => 96,
            0x20b => 112,
            _ => panic!(),
        };
    rva_offset(bytes, u32_at(bytes, directories + 14 * 8))
}

fn metadata_root(bytes: &[u8]) -> (usize, usize) {
    let cli = cli_header(bytes);
    (rva_offset(bytes, u32_at(bytes, cli + 8)), cli + 12)
}

fn metadata_streams(bytes: &[u8]) -> Vec<(String, usize, usize, usize)> {
    let (metadata, _) = metadata_root(bytes);
    let version_len = u32_at(bytes, metadata + 12) as usize;
    let stream_count = u16_at(bytes, metadata + version_len + 18) as usize;
    let mut stream = metadata + version_len + 20;
    let mut result = Vec::with_capacity(stream_count);

    for _ in 0..stream_count {
        let offset = u32_at(bytes, stream) as usize;
        let len = u32_at(bytes, stream + 4) as usize;
        let name = &bytes[stream + 8..];
        let name_len = name.iter().position(|byte| *byte == 0).unwrap();
        result.push((
            std::str::from_utf8(&name[..name_len]).unwrap().to_string(),
            stream,
            metadata + offset,
            len,
        ));
        stream += 8 + (name_len + 1).next_multiple_of(4);
    }
    result
}

#[test]
fn optional_header_bounds_data_directories() {
    let mut bytes = windows_default::WINRT.to_vec();
    let optional = optional_header(&bytes);
    let coff = optional - 20;

    bytes[coff + 16..coff + 18].copy_from_slice(&2u16.to_le_bytes());

    assert!(File::new(bytes).is_none());
}

#[test]
fn cli_header_is_bounded_by_data_directory() {
    let mut bytes = windows_default::WINRT.to_vec();
    let optional = optional_header(&bytes);
    let directories = optional
        + match u16_at(&bytes, optional) {
            0x10b => 96,
            0x20b => 112,
            _ => panic!(),
        };
    let cli = directories + 14 * 8;

    bytes[cli + 4..cli + 8].copy_from_slice(&16u32.to_le_bytes());

    assert!(File::new(bytes).is_none());
}

#[test]
fn metadata_root_is_bounded_by_directory() {
    let mut bytes = windows_default::WINRT.to_vec();
    let (_, metadata_size) = metadata_root(&bytes);

    bytes[metadata_size..metadata_size + 4].copy_from_slice(&19u32.to_le_bytes());

    assert!(File::new(bytes).is_none());
}

#[test]
fn metadata_streams_are_bounded_by_directory() {
    let mut bytes = windows_default::WINRT.to_vec();
    let (metadata, metadata_size) = metadata_root(&bytes);
    let stream_end = metadata_streams(&bytes)
        .into_iter()
        .map(|(_, _, start, len)| start + len - metadata)
        .max()
        .unwrap();

    bytes[metadata_size..metadata_size + 4].copy_from_slice(&(stream_end as u32 - 1).to_le_bytes());

    assert!(File::new(bytes).is_none());
}

#[test]
fn table_rows_are_bounded_by_stream() {
    let mut bytes = windows_default::WINRT.to_vec();
    let (_, header, _, _) = metadata_streams(&bytes)
        .into_iter()
        .find(|(name, _, _, _)| name == "#~")
        .unwrap();

    bytes[header + 4..header + 8].copy_from_slice(&24u32.to_le_bytes());

    assert!(File::new(bytes).is_none());
}

#[test]
fn table_stream_rejects_nonzero_trailing_data() {
    let mut bytes = windows_default::WINRT.to_vec();
    let (_, _, start, len) = metadata_streams(&bytes)
        .into_iter()
        .find(|(name, _, _, _)| name == "#~")
        .unwrap();
    assert_eq!(bytes[start + len - 1], 0);

    bytes[start + len - 1] = 1;

    assert!(File::new(bytes).is_none());
}

#[test]
fn table_stream_rejects_excessive_zero_padding() {
    let mut bytes = windows_default::WINRT.to_vec();
    let (_, header, start, len) = metadata_streams(&bytes)
        .into_iter()
        .find(|(name, _, _, _)| name == "#~")
        .unwrap();

    bytes[header + 4..header + 8].copy_from_slice(&(len as u32 + 5).to_le_bytes());
    bytes[start + len..start + len + 5].fill(0);

    assert!(File::new(bytes).is_none());
}
