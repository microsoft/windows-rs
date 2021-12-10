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

    pub fn is_input(&self) -> bool {
        self.flags().input()
    }

    pub fn is_optional(&self) -> bool {
        self.flags().optional()
    }

    pub fn is_com_out_ptr(&self) -> bool {
        self.has_attribute("ComOutPtrAttribute")
    }

    pub fn array_info(&self) -> bool {
        // TODO: replace bool return with actual array info from attribute
        self.has_attribute("NativeArrayInfoAttribute")
    }

    pub fn has_alternate_success_code(&self) -> bool {
        self.has_attribute("AlternateSuccessCodesAttribute")
    }
}
