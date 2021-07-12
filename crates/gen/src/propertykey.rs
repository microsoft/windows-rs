use super::*;

#[derive(Clone, PartialEq, Default)]
pub struct PropertyKey {
    pub fmtid: Guid,
    pub pid: u32,
}

impl PropertyKey {
    pub fn from_attributes<I: IntoIterator<Item = tables::Attribute>>(
        attributes: I,
    ) -> Option<Self> {
        for attribute in attributes {
            if attribute.name() == "PropertyKeyAttribute" {
                let args = attribute.args();
                return Some(Self {
                    fmtid: Guid::from_args(&args),
                    pid: args[11].1.unwrap_u32(),
                });
            }
        }

        None
    }
}
