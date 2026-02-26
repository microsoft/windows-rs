use super::*;

#[allow(dead_code)]
pub type Row = windows_metadata::reader::Row<'static>;
pub type RowIterator<R> = windows_metadata::reader::RowIterator<'static, R>;
pub use windows_metadata::reader::AsRow;

pub trait RowExt {
    fn reader(&self) -> &'static Reader;
}

impl<T: AsRow<'static>> RowExt for T {
    fn reader(&self) -> &'static Reader {
        reader_from_index(self.to_row().index)
    }
}

pub trait HasAttributes {
    fn attributes(&self) -> RowIterator<Attribute>;
    fn find_attribute(&self, name: &str) -> Option<Attribute>;
    fn has_attribute(&self, name: &str) -> bool;
    fn guid_attribute(&self) -> Option<GUID>;
    fn arches(&self) -> i32;
}

impl<R: AsRow<'static> + Into<HasAttribute>> HasAttributes for R {
    fn attributes(&self) -> RowIterator<Attribute> {
        self.equal_range(0, Into::<HasAttribute>::into(*self).encode())
    }

    fn find_attribute(&self, name: &str) -> Option<Attribute> {
        self.attributes()
            .find(|attribute| attribute.name() == name)
    }

    fn has_attribute(&self, name: &str) -> bool {
        self.find_attribute(name).is_some()
    }

    fn guid_attribute(&self) -> Option<GUID> {
        self.find_attribute("GuidAttribute").map(|attribute| {
            fn unwrap_u32(value: &Value) -> u32 {
                match value {
                    Value::U32(value) => *value,
                    _ => panic!(),
                }
            }
            fn unwrap_u16(value: &Value) -> u16 {
                match value {
                    Value::U16(value) => *value,
                    rest => panic!("{rest:?}"),
                }
            }
            fn unwrap_u8(value: &Value) -> u8 {
                match value {
                    Value::U8(value) => *value,
                    rest => panic!("{rest:?}"),
                }
            }

            let args = attribute.args();

            GUID(
                unwrap_u32(&args[0].1),
                unwrap_u16(&args[1].1),
                unwrap_u16(&args[2].1),
                unwrap_u8(&args[3].1),
                unwrap_u8(&args[4].1),
                unwrap_u8(&args[5].1),
                unwrap_u8(&args[6].1),
                unwrap_u8(&args[7].1),
                unwrap_u8(&args[8].1),
                unwrap_u8(&args[9].1),
                unwrap_u8(&args[10].1),
            )
        })
    }

    fn arches(&self) -> i32 {
        let mut arches = 0;

        if let Some(attribute) = self.find_attribute("SupportedArchitectureAttribute") {
            if let Some((_, Value::I32(value))) = attribute.args().first() {
                arches = *value;
            }
        }

        arches
    }
}
