use super::*;

#[derive(Clone)]
pub struct Param(pub Row);

impl Param {
    pub fn flags(&self) -> ParamFlags {
        ParamFlags(self.0.u32(0))
    }

    pub fn sequence(&self) -> u32 {
        self.0.u32(1)
    }

    pub fn name(&self) -> &'static str {
        self.0.str(2)
    }

    fn attributes(&self) -> impl Iterator<Item = Attribute> {
        self.0.file.attributes(HasAttribute::Param(self.clone()))
    }

    fn has_attribute(&self, name: &str) -> bool {
        self.attributes().any(|attribute| attribute.name() == name)
    }

    pub fn is_com_out_ptr(&self) -> bool {
        self.has_attribute("ComOutPtrAttribute")
    }

    pub fn array_info(&self) -> Option<ArrayInfo> {
        for attribute in self.attributes() {
            if attribute.name() == "NativeArrayInfoAttribute" {
                for (_, value) in attribute.args() {
                    match value {
                        ConstantValue::I16(value) => return Some(ArrayInfo::Relative(value)),
                        ConstantValue::I32(value) => return Some(ArrayInfo::Fixed(value)),
                        _ => unimplemented!(),
                    }
                }
            }
        }

        None
    }

    pub fn is_retval(&self) -> bool {
        self.has_attribute("RetValAttribute")
    }

    pub fn free_with(&self) -> Option<String> {
        for attribute in self.attributes() {
            if attribute.name() == "FreeWithAttribute" {
                for (_, arg) in attribute.args() {
                    if let ConstantValue::String(name) = arg {
                        return Some(name);
                    }
                }
            }
        }

        None
    }
}
