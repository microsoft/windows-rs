use windows_metadata::reader::File;

#[test]
fn optional_header_bounds_data_directories() {
    let mut bytes = windows_default::WINRT.to_vec();
    let pe = u32::from_le_bytes(bytes[0x3c..0x40].try_into().unwrap()) as usize;
    let coff = pe + 4;

    bytes[coff + 16..coff + 18].copy_from_slice(&2u16.to_le_bytes());

    assert!(File::new(bytes).is_none());
}
