use super::*;
macros::table!(TypeDef);

impl TypeDef {
    pub fn flags(&self) -> TypeFlags {
        TypeFlags(self.reader.u32(self.row, 0))
    }

    pub fn name(&self) -> &'static str {
        self.reader.str(self.row, 1)
    }

    pub fn namespace(&self) -> &'static str {
        self.reader.str(self.row, 2)
    }

    // TODO: all "full_name" methods should return a FullName struct that provides a fast compare for match expressions
    pub fn full_name(&self) -> (&'static str, &'static str) {
        (self.namespace(), self.name())
    }

    fn extends(&self) -> TypeDefOrRef {
        self.reader.decode(self.row, 3)
    }

    pub fn bases(&self) -> impl Iterator<Item = TypeDef> + '_ {
        Bases(*self)
    }

    pub fn fields(&self) -> impl Iterator<Item = Field> + '_ {
        self.reader
            .list(self.row, TableIndex::Field, 4)
            .map(move |row| Field {
                reader: self.reader,
                row,
            })
    }

    pub fn methods(&self) -> impl Iterator<Item = MethodDef> + '_ {
        self.reader
            .list(self.row, TableIndex::MethodDef, 5)
            .map(move |row| MethodDef {
                reader: self.reader,
                row,
            })
    }

    pub fn generics(&self) -> impl Iterator<Item = GenericParam> + '_ {
        self.reader
            .equal_range(
                self.row.file_index,
                TableIndex::GenericParam,
                2,
                TypeOrMethodDef::TypeDef(*self).encode(),
            )
            .map(move |row| GenericParam {
                reader: self.reader,
                row,
            })
    }

    pub fn interface_impls(&self) -> impl Iterator<Item = InterfaceImpl> + '_ {
        self.reader
            .equal_range(
                self.row.file_index,
                TableIndex::InterfaceImpl,
                0,
                self.row.index + 1,
            )
            .map(move |row| InterfaceImpl {
                reader: self.reader,
                row,
            })
    }

    // TODO: this should be an iterator...
    pub fn nested_types(&self) -> Vec<tables::TypeDef> {
        self.reader.nested_types(self)
    }

    pub fn attributes(&self) -> impl Iterator<Item = Attribute> + '_ {
        self.reader
            .equal_range(
                self.row.file_index,
                TableIndex::CustomAttribute,
                0,
                HasAttribute::TypeDef(*self).encode(),
            )
            .map(move |row| Attribute {
                reader: self.reader,
                row,
            })
    }

    pub fn has_attribute(&self, name: &str) -> bool {
        self.attributes().any(|attribute| attribute.name() == name)
    }

    pub fn is_winrt(&self) -> bool {
        self.flags().windows_runtime()
    }

    pub fn is_exclusive(&self) -> bool {
        self.has_attribute("ExclusiveToAttribute")
    }

    pub fn is_agile(&self) -> bool {
        self.attributes().any(|attribute| {
            if attribute.name() == "MarshalingBehaviorAttribute" {
                if let Some((_, ConstantValue::I32(2))) = attribute.args().get(0) {
                    return true;
                }
            }

            false
        })
    }

    pub fn kind(&self) -> TypeKind {
        if self.flags().interface() {
            TypeKind::Interface
        } else {
            match self.extends().full_name() {
                ("System", "Enum") => TypeKind::Enum,
                ("System", "MulticastDelegate") => TypeKind::Delegate,
                ("System", "ValueType") => TypeKind::Struct,
                _ => TypeKind::Class,
            }
        }
    }

    pub fn version(&self) -> (u16, u16) {
        for attribute in self.attributes() {
            match attribute.name() {
                "ContractVersionAttribute" | "VersionAttribute" => {
                    for (_, value) in attribute.args() {
                        if let ConstantValue::U32(value) = value {
                            return ((value >> 16) as u16, (value & 0xFFFF) as u16);
                        }
                    }
                }
                _ => {}
            }
        }

        (0, 0)
    }

    pub fn guid(&self) -> Guid {
        Guid::from_type_def(self).expect("TypeDef::guid")
    }

    pub fn enclosing_type(&self) -> Option<Self> {
        self.reader
            .equal_range(
                self.row.file_index,
                TableIndex::NestedClass,
                0,
                self.row.index + 1,
            )
            .map(move |row| NestedClass {
                reader: self.reader,
                row,
            })
            .next()
            .map(|nested| nested.enclosing_type())
    }

    fn scoped_name(&self) -> String {
        if let Some(enclosing_type) = self.enclosing_type() {
            for (index, nested_type) in enclosing_type.nested_types().iter().enumerate() {
                if nested_type.name() == self.name() {
                    return format!("{}_{}", enclosing_type.scoped_name(), index);
                }
            }
        }

        self.name().to_string()
    }

    pub fn gen_name(&self, gen: &Gen) -> TokenStream {
        let namespace = self.namespace();

        if namespace.is_empty() {
            let name = to_ident(&self.scoped_name());
            quote! { #name }
        } else {
            let name = to_ident(self.name());
            let namespace = gen.namespace(self.namespace());
            quote! { #namespace#name }
        }
    }

    pub fn gen_abi_name(&self, gen: &Gen) -> TokenStream {
        let namespace = self.namespace();

        if namespace.is_empty() {
            let name = to_abi_ident(&self.scoped_name());
            quote! { #name }
        } else {
            let name = to_abi_ident(self.name());
            let namespace = gen.namespace(self.namespace());
            quote! { #namespace#name }
        }
    }
}

impl std::fmt::Debug for TypeDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.namespace(), self.name())
    }
}

struct Bases(TypeDef);

impl Iterator for Bases {
    type Item = TypeDef;

    fn next(&mut self) -> Option<Self::Item> {
        let extends = self.0.extends();

        if extends.full_name() == ("System", "Object") {
            None
        } else {
            self.0 = extends.resolve();
            Some(self.0)
        }
    }
}
