use super::*;
macros::table!(Param);

impl Param {
    pub fn flags(&self) -> ParamFlags {
        ParamFlags(self.reader.u32(self.row, 0))
    }

    pub fn sequence(&self) -> u32 {
        self.reader.u32(self.row, 1)
    }

    pub fn name(&self) -> &'static str {
        self.reader.str(self.row, 2)
    }

    pub fn attributes(&self) -> impl Iterator<Item = Attribute> + '_ {
        self.reader
            .equal_range(
                self.row.file_index,
                TableIndex::CustomAttribute,
                0,
                HasAttribute::Param(*self).encode(),
            )
            .map(move |row| Attribute {
                reader: self.reader,
                row,
            })
    }

    pub fn has_attribute(&self, namespace: &str, name: &str) -> bool {
        self.attributes()
            .any(|attribute| attribute.full_name() == (namespace, name))
    }

    pub fn is_input(&self) -> bool {
        self.flags().input()
    }

    pub fn is_const(&self) -> bool {
        self.has_attribute("Windows.Win32.Interop", "ConstAttribute")
    }

    pub fn gen_name(&self) -> Ident {
        to_ident(&to_snake(self.name()))
    }

    pub fn gen_abi_size_name(&self) -> Ident {
        let mut name = to_snake(self.name());
        name.push_str("_array_size");
        to_ident(&name)
    }
}

impl std::fmt::Debug for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
