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

    pub fn attributes(&self) -> impl Iterator<Item = Attribute> {
        self.0.file.attributes(HasAttribute::Param(self.clone()))
    }

    pub fn has_attribute(&self, name: &str) -> bool {
        self.attributes().any(|attribute| attribute.name() == name)
    }

    pub fn is_input(&self) -> bool {
        self.flags().input()
    }

    pub fn is_const(&self) -> bool {
        self.has_attribute("ConstAttribute")
    }

    pub fn gen_name(&self) -> Ident {
        to_ident(&self.name().to_lowercase())
    }

    pub fn gen_abi_size_name(&self) -> Ident {
        to_ident(&format!("{}_array_size", self.name()))
    }
}
