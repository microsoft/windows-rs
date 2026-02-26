pub struct File(pub(crate) windows_metadata::reader::File);

impl File {
    pub fn new(bytes: Vec<u8>) -> Option<Self> {
        windows_metadata::reader::File::new(bytes).map(File)
    }
}

