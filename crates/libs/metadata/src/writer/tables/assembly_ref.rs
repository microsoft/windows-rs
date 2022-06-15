#[derive(Default)]
pub struct AssemblyRef {
    pub major_version: u16,
    pub minor_version: u16,
    pub build_number: u16,
    pub revision_number: u16,
    pub name: String,
}

impl AssemblyRef {
    pub fn mscorlib() -> Self {
        Self { major_version: 4, name: "mscorlib".to_string(), ..Default::default() }
    }
}
