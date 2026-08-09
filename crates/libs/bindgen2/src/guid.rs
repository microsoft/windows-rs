use super::*;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct Guid {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8],
}

impl Guid {
    pub(super) fn find(
        database: &Database,
        namespace: &str,
        name: &str,
        owner: &str,
    ) -> Result<Option<Self>, Error> {
        let mut result = None;
        for entity in database.type_definitions(namespace, name) {
            let definition = database.definition(*entity).unwrap();
            let Some(guid) = Self::from_definition(definition, owner)? else {
                continue;
            };
            if result.is_some_and(|existing| existing != guid) {
                return Err(Error::InvalidValue {
                    name: owner.to_string(),
                    message: "referenced type has conflicting GUIDs",
                });
            }
            result = Some(guid);
        }
        Ok(result)
    }

    fn from_definition(definition: TypeDefinition<'_>, owner: &str) -> Result<Option<Self>, Error> {
        let Some(attribute) = definition.find_attribute("GuidAttribute")? else {
            return Ok(None);
        };
        Self::from_attribute(attribute, owner).map(Some)
    }

    pub(super) fn from_field(
        field: windows_metadata2::FieldDefinition<'_>,
        owner: &str,
    ) -> Result<Option<Self>, Error> {
        let Some(attribute) = field.find_attribute("GuidAttribute")? else {
            return Ok(None);
        };
        Self::from_attribute(attribute, owner).map(Some)
    }

    fn from_attribute(
        attribute: windows_metadata2::AttributeDefinition<'_>,
        owner: &str,
    ) -> Result<Self, Error> {
        let arguments = attribute.arguments(&())?;
        let values = arguments
            .iter()
            .map(|argument| match argument {
                AttributeArgument::Fixed { value, .. } => Ok(value),
                _ => Err(Error::InvalidValue {
                    name: owner.to_string(),
                    message: "GuidAttribute has a named argument",
                }),
            })
            .collect::<Result<Vec<_>, _>>()?;
        let [
            data1,
            data2,
            data3,
            data4,
            data5,
            data6,
            data7,
            data8,
            data9,
            data10,
            data11,
        ] = values.as_slice()
        else {
            return Err(Error::InvalidValue {
                name: owner.to_string(),
                message: "GuidAttribute does not have 11 arguments",
            });
        };
        Ok(Self {
            data1: u32_value(data1, owner)?,
            data2: u16_value(data2, owner)?,
            data3: u16_value(data3, owner)?,
            data4: [
                u8_value(data4, owner)?,
                u8_value(data5, owner)?,
                u8_value(data6, owner)?,
                u8_value(data7, owner)?,
                u8_value(data8, owner)?,
                u8_value(data9, owner)?,
                u8_value(data10, owner)?,
                u8_value(data11, owner)?,
            ],
        })
    }

    pub(super) fn write_u128(self) -> proc_macro2::TokenStream {
        format!(
            "0x{:08x}_{:04x}_{:04x}_{:02x}{:02x}_{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.data1,
            self.data2,
            self.data3,
            self.data4[0],
            self.data4[1],
            self.data4[2],
            self.data4[3],
            self.data4[4],
            self.data4[5],
            self.data4[6],
            self.data4[7],
        )
        .parse()
        .unwrap()
    }
}

impl std::fmt::Display for Guid {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.data1,
            self.data2,
            self.data3,
            self.data4[0],
            self.data4[1],
            self.data4[2],
            self.data4[3],
            self.data4[4],
            self.data4[5],
            self.data4[6],
            self.data4[7],
        )
    }
}

fn u32_value(value: &AttributeValue, owner: &str) -> Result<u32, Error> {
    match value {
        AttributeValue::U32(value) => Ok(*value),
        _ => Err(invalid_guid(owner)),
    }
}

fn u16_value(value: &AttributeValue, owner: &str) -> Result<u16, Error> {
    match value {
        AttributeValue::U16(value) => Ok(*value),
        _ => Err(invalid_guid(owner)),
    }
}

fn u8_value(value: &AttributeValue, owner: &str) -> Result<u8, Error> {
    match value {
        AttributeValue::U8(value) => Ok(*value),
        _ => Err(invalid_guid(owner)),
    }
}

fn invalid_guid(owner: &str) -> Error {
    Error::InvalidValue {
        name: owner.to_string(),
        message: "GuidAttribute argument has the wrong type",
    }
}
