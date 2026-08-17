use windows_metadata::reader::File;

fn optional_header(bytes: &[u8]) -> usize {
    let pe = u32::from_le_bytes(bytes[0x3c..0x40].try_into().unwrap()) as usize;
    pe + 4 + 20
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
    let magic = u16::from_le_bytes(bytes[optional..optional + 2].try_into().unwrap());
    let directories = optional
        + match magic {
            0x10b => 96,
            0x20b => 112,
            _ => panic!(),
        };
    let cli = directories + 14 * 8;

    bytes[cli + 4..cli + 8].copy_from_slice(&16u32.to_le_bytes());

    assert!(File::new(bytes).is_none());
}
