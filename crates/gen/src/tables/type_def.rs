use super::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct TypeDef(Row);

impl From<Row> for TypeDef {
    fn from(row: Row) -> Self {
        Self(row)
    }
}

impl TypeDef {
    pub fn row(&self) -> &Row {
        &self.0
    }

    pub fn flags(&self) -> TypeFlags {
        TypeFlags(self.0.u32(0))
    }

    pub fn name(&self) -> &'static str {
        self.0.str(1)
    }

    pub fn namespace(&self) -> &'static str {
        self.0.str(2)
    }

    // TODO: all "full_name" methods should return a FullName struct that provides a fast compare for match expressions
    pub fn full_name(&self) -> (&'static str, &'static str) {
        (self.namespace(), self.name())
    }

    pub fn extends(&self) -> (&'static str, &'static str) {
        let extends = self.0.u32(3);

        if extends == 0 {
            ("", "")
        } else {
            TypeDefOrRef::decode(self.0.file, extends).full_name()
        }
    }

    pub fn bases(&self) -> impl Iterator<Item = TypeDef> {
        Bases(self.clone())
    }

    pub fn fields(&self) -> impl Iterator<Item = Field> {
        self.0.list(4, TableIndex::Field).map(Field)
    }

    pub fn methods(&self) -> impl Iterator<Item = MethodDef> {
        self.0.list(5, TableIndex::MethodDef).map(MethodDef)
    }

    pub fn generics(&self) -> impl Iterator<Item = GenericParam> {
        self.0
            .file
            .equal_range(
                TableIndex::GenericParam,
                2,
                TypeOrMethodDef::TypeDef(self.clone()).encode(),
            )
            .map(GenericParam)
    }

    pub fn interface_impls(&self) -> impl Iterator<Item = InterfaceImpl> {
        self.0
            .file
            .equal_range(TableIndex::InterfaceImpl, 0, self.0.row + 1)
            .map(InterfaceImpl)
    }

    // TODO: this should be an iterator...
    pub fn nested_types(&self) -> Vec<tables::TypeDef> {
        TypeReader::get().nested_types(self)
    }

    pub fn attributes(&self) -> impl Iterator<Item = Attribute> {
        self.0
            .file
            .equal_range(
                TableIndex::CustomAttribute,
                0,
                HasAttribute::TypeDef(self.clone()).encode(),
            )
            .map(Attribute)
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

    pub fn is_scoped(&self) -> bool {
        self.is_winrt() || self.has_attribute("ScopedEnumAttribute")
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

    pub fn is_convertible(&self) -> Option<ElementType> {
        self.attributes().find_map(|attribute| {
            if attribute.name() == "AlsoUsableForAttribute" {
                if let Some((_, ConstantValue::String(name))) = attribute.args().get(0) {
                    // TODO: https://github.com/microsoft/win32metadata/issues/389
                    return Some(TypeReader::get().resolve_type(self.namespace(), name));
                }
            }

            None
        })
    }

    pub fn is_public_composable(&self) -> bool {
        self.attributes().any(|attribute| {
            attribute.name() == "ComposableAttribute"
                && attribute
                    .args()
                    .iter()
                    .any(|arg| matches!(arg, (_, ConstantValue::I32(2))))
        })
    }

    pub fn kind(&self) -> TypeKind {
        if self.flags().interface() {
            TypeKind::Interface
        } else {
            match self.extends() {
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
        Guid::from_attributes(self.attributes()).expect("TypeDef::guid")
    }

    pub fn enclosing_type(&self) -> Option<Self> {
        self.0
            .file
            .equal_range(TableIndex::NestedClass, 0, self.0.row + 1)
            .map(NestedClass)
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

    pub fn class_layout(&self) -> Option<ClassLayout> {
        self.0
            .file
            .equal_range(TableIndex::ClassLayout, 2, self.0.row + 1)
            .map(ClassLayout)
            .next()
    }

    pub fn overridable_interfaces(&self) -> Vec<TypeDef> {
        self.interface_impls()
            .filter(|interface| interface.is_overridable())
            .map(|interface| interface.interface().resolve())
            .chain(
                self.bases()
                    .next()
                    .iter()
                    .flat_map(|base| base.overridable_interfaces()),
            )
            .collect()
    }

    pub fn overridable_methods(&self) -> BTreeSet<&'static str> {
        self.overridable_interfaces()
            .iter()
            .flat_map(|interface| interface.methods().map(|method| method.name()))
            .collect()
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
        let (namespace, name) = self.0.extends();

        if (namespace, name) == ("System", "Object") {
            None
        } else {
            self.0 = TypeReader::get().resolve_type_def(namespace, name);
            Some(self.0.clone())
        }
    }
}
