use super::*;

#[derive(Clone, PartialEq, Default)]
pub struct PropertyKey {
    pub fmtid: GUID,
    pub pid: u32,
}

impl PropertyKey {
    pub fn from_attributes<I: IntoIterator<Item = Attribute>>(attributes: I) -> Option<Self> {
        attributes
            .into_iter()
            .find(|attribute| attribute.name() == "PropertyKeyAttribute")
            .map(|attribute| {
                let args = attribute.args();
                Self {
                    fmtid: GUID::from_args(&args),
                    pid: args[11].1.unwrap_u32(),
                }
            })
    }
}
