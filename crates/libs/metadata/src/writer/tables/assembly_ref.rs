#[derive(Default, Clone)]
pub struct AssemblyRef {
    pub major_version: u16,
    pub minor_version: u16,
    pub build_number: u16,
    pub revision_number: u16,
    pub name: String,
}

impl AssemblyRef {
    pub fn mscorlib() -> Self {
        Self { name: "mscorlib".to_string(), major_version: 4, ..Default::default() }
    }
}
