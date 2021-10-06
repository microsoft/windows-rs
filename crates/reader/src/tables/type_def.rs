use super::*;
pub use std::collections::BTreeSet;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct TypeDef {
    pub row: Row,
    pub generics: Vec<ElementType>,
}

impl From<Row> for TypeDef {
    fn from(row: Row) -> Self {
        Self {
            row,
            generics: Vec::new(),
        }
    }
}

impl TypeDef {
    pub fn with_generics(mut self) -> Self {
        self.generics = self
            .generic_params()
            .map(|generic| ElementType::GenericParam(generic.name().to_string()))
            .collect();
        self
    }

    pub fn is_callback(&self) -> bool {
        !self.is_winrt() && self.kind() == TypeKind::Delegate
    }

    pub fn has_default_constructor(&self) -> bool {
        for attribute in self.attributes() {
            if attribute.name() == "ActivatableAttribute" {
                if attribute
                    .args()
                    .iter()
                    .any(|arg| matches!(arg.1, ConstantValue::TypeDef(_)))
                {
                    continue;
                } else {
                    return true;
                }
            }
        }

        false
    }

    pub fn invoke_method(&self) -> MethodDef {
        self.methods()
            .find(|m| m.name() == "Invoke")
            .expect("`Invoke` method not found")
    }

    pub fn definition(&self, include: TypeInclude) -> Vec<TypeEntry> {
        let mut definition = vec![TypeEntry {
            include,
            def: ElementType::TypeDef(self.clone()),
        }];

        for generic in &self.generics {
            definition.append(&mut generic.definition(include));
        }

        definition
    }

    pub fn include_definition(&self, reader: &mut TypeReader, include: TypeInclude) {
        reader.include_type_name(self.type_name(), include);

        for generic in &self.generics {
            generic.include_definition(reader, include);
        }
    }

    pub fn default_interface(&self) -> Option<Self> {
        for interface in self.interface_impls() {
            if interface.is_default() {
                if let ElementType::TypeDef(def) = interface.generic_interface(&self.generics) {
                    return Some(def);
                }
            }
        }

        None
    }

    pub fn interfaces(&self) -> impl Iterator<Item = Self> + '_ {
        self.interface_impls().filter_map(move |i| {
            if let ElementType::TypeDef(def) = i.generic_interface(&self.generics) {
                Some(def)
            } else {
                None
            }
        })
    }

    pub fn is_packed(&self) -> bool {
        if self.kind() != TypeKind::Struct {
            return false;
        }

        if self.class_layout().is_some() {
            return true;
        }

        self.fields().any(|field| field.signature().is_packed())
    }

    pub fn is_handle(&self) -> bool {
        self.has_attribute("NativeTypedefAttribute")
    }

    pub fn dependencies(&self, include: TypeInclude) -> Vec<TypeEntry> {
        match self.kind() {
            TypeKind::Interface => {
                if include == TypeInclude::Minimal {
                    return Vec::new();
                }

                let interfaces = self.interfaces().map(|i| i.definition(include));
                let methods = self.methods().map(|m| m.dependencies());
                interfaces.chain(methods).flatten().collect()
            }
            TypeKind::Class => {
                if include == TypeInclude::Minimal {
                    if let Some(default_interface) = self.default_interface() {
                        return default_interface.definition(TypeInclude::Minimal);
                    } else {
                        return Vec::new();
                    }
                }

                let generics = self
                    .generics
                    .iter()
                    .map(|g| g.definition(TypeInclude::Minimal));
                let interfaces = self.interfaces().map(|i| i.definition(TypeInclude::Full));
                let bases = self.bases().map(|b| b.definition(TypeInclude::Full));

                let factories = self.attributes().filter_map(|attribute| {
                    match attribute.name() {
                        "StaticAttribute" | "ActivatableAttribute" | "ComposableAttribute" => {
                            for (_, arg) in attribute.args() {
                                if let ConstantValue::TypeDef(def) = arg {
                                    return Some(TypeEntry {
                                        include: TypeInclude::Full,
                                        def: ElementType::TypeDef(def),
                                    });
                                }
                            }
                        }
                        _ => {}
                    }

                    None
                });

                generics
                    .chain(interfaces)
                    .chain(bases)
                    .flatten()
                    .chain(factories)
                    .collect()
            }
            TypeKind::Enum => Vec::new(),
            TypeKind::Struct => {
                let mut dependencies: Vec<TypeEntry> = self
                    .fields()
                    .map(|f| f.definition(TypeInclude::Minimal))
                    .flatten()
                    .collect();

                if let Some(dependency) = self.is_convertible_to() {
                    dependencies.push(TypeEntry {
                        include: TypeInclude::Minimal,
                        def: ElementType::TypeDef(dependency),
                    });
                }

                dependencies
            }
            TypeKind::Delegate => self.invoke_method().dependencies(),
        }
    }

    pub fn include_dependencies(&self, reader: &mut TypeReader, include: TypeInclude) {

    }

    pub fn is_udt(&self) -> bool {
        // TODO: should this just check whether the struct has > 1 fields rather than is_handle?
        self.kind() == TypeKind::Struct && !self.is_handle()
    }

    pub fn is_convertible(&self) -> bool {
        match self.kind() {
            TypeKind::Interface | TypeKind::Class | TypeKind::Struct => true,
            TypeKind::Delegate => self.is_winrt(),
            _ => false,
        }
    }

    pub fn is_primitive(&self) -> bool {
        self.kind() == TypeKind::Enum
    }

    pub fn is_explicit(&self) -> bool {
        self.row.u32(0) & 0b1_0000 != 0
    }

    pub fn has_explicit(&self) -> bool {
        if self.kind() != TypeKind::Struct {
            return false;
        }

        if self.is_explicit() {
            true
        } else {
            self.fields().any(|f| f.signature().has_explicit())
        }
    }

    pub fn type_signature(&self) -> String {
        match self.kind() {
            TypeKind::Interface => self.interface_signature(),
            TypeKind::Class => format!(
                "rc({};{})",
                self.type_name(),
                self.default_interface()
                    .unwrap_or_else(|| panic!(
                        "`{}` does not have a default interface.",
                        self.type_name()
                    ))
                    .interface_signature()
            ),
            TypeKind::Enum => format!(
                "enum({};{})",
                self.type_name(),
                self.underlying_type().type_signature()
            ),
            TypeKind::Struct => {
                let mut result = format!("struct({}", self.type_name());

                for field in self.fields() {
                    result.push(';');
                    result.push_str(&field.signature().kind.type_signature());
                }

                result.push(')');
                result
            }
            TypeKind::Delegate => {
                if self.generics.is_empty() {
                    format!("delegate({})", self.interface_signature())
                } else {
                    self.interface_signature()
                }
            }
        }
    }

    pub fn underlying_type(&self) -> ElementType {
        if let Some(field) = self.fields().next() {
            if let Some(constant) = field.constant() {
                return constant.value_type();
            } else {
                return field.signature().kind;
            }
        }

        unimplemented!();
    }

    pub fn interface_signature(&self) -> String {
        let guid = self.guid();

        if self.generics.is_empty() {
            format!("{{{:#?}}}", guid)
        } else {
            let mut result = format!("pinterface({{{:#?}}}", guid);

            for generic in &self.generics {
                result.push(';');
                result.push_str(&generic.type_signature());
            }

            result.push(')');
            result
        }
    }

    pub fn is_winrt(&self) -> bool {
        self.row.u32(0) & 0b100_0000_0000_0000 != 0
    }

    pub fn is_interface(&self) -> bool {
        self.row.u32(0) & 0b10_0000 != 0
    }

    pub fn name(&self) -> &'static str {
        self.row.str(1)
    }

    pub fn namespace(&self) -> &'static str {
        self.row.str(2)
    }

    pub fn type_name(&self) -> TypeName {
        TypeName::new(self.namespace(), self.name())
    }

    pub fn extends(&self) -> TypeName {
        let extends = self.row.u32(3);

        if extends == 0 {
            TypeName::None
        } else {
            TypeDefOrRef::decode(self.row.file, extends).type_name()
        }
    }

    // TODO: rename base_classes
    pub fn bases(&self) -> impl Iterator<Item = TypeDef> {
        Bases(self.clone())
    }

    pub fn base_interfaces(&self) -> (Vec<Self>, bool) {
        let mut result = Vec::new();
        let mut next = self.clone();
        let mut inspectable = false;

        while let Some(base) = next
            .interface_impls()
            .filter_map(|i| match i.generic_interface(&[]) {
                ElementType::TypeDef(def) => Some(def),
                ElementType::IUnknown => None,
                ElementType::IInspectable => {
                    inspectable = true;
                    None
                }
                _ => unimplemented!(),
            })
            .next()
        {
            next = base.clone();
            result.push(base);
        }

        (result, inspectable)
    }

    pub fn fields(&self) -> impl Iterator<Item = Field> {
        self.row.list(4, TableIndex::Field).map(Field)
    }

    pub fn methods(&self) -> impl Iterator<Item = MethodDef> {
        self.row.list(5, TableIndex::MethodDef).map(MethodDef)
    }

    pub fn generic_params(&self) -> impl Iterator<Item = GenericParam> {
        self.row
            .file
            .equal_range(
                TableIndex::GenericParam,
                2,
                TypeOrMethodDef::TypeDef(self.clone()).encode(),
            )
            .map(GenericParam)
    }

    pub fn interface_impls(&self) -> impl Iterator<Item = InterfaceImpl> {
        self.row
            .file
            .equal_range(TableIndex::InterfaceImpl, 0, self.row.row + 1)
            .map(InterfaceImpl)
    }

    pub fn nested_types(&self) -> Option<&BTreeMap<&'static str, TypeDef>> {
        TypeReader::get().nested_types(self)
    }

    pub fn attributes(&self) -> impl Iterator<Item = Attribute> {
        self.row
            .file
            .attributes(HasAttribute::TypeDef(self.clone()))
    }

    fn has_attribute(&self, name: &str) -> bool {
        self.attributes().any(|attribute| attribute.name() == name)
    }

    pub fn has_flags(&self) -> bool {
        self.has_attribute("FlagsAttribute")
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

    pub fn is_convertible_to(&self) -> Option<TypeDef> {
        self.attributes().find_map(|attribute| {
            if attribute.name() == "AlsoUsableForAttribute" {
                if let Some((_, ConstantValue::String(name))) = attribute.args().get(0) {
                    return TypeReader::get()
                        .get_type((self.namespace(), name.as_str()))
                        .and_then(|row| {
                            if let ElementType::TypeDef(def) = row {
                                Some(def)
                            } else {
                                None
                            }
                        });
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

    pub fn is_blittable(&self) -> bool {
        match self.kind() {
            TypeKind::Struct => {
                // TODO: should be "if self.can_drop().is_some() {" once win32metadata bugs are fixed (423, 422, 421, 389)
                if self.type_name() == TypeName::BSTR {
                    false
                } else {
                    self.fields().all(|f| f.is_blittable())
                }
            }
            TypeKind::Enum => true,
            _ => false,
        }
    }

    pub fn kind(&self) -> TypeKind {
        if self.is_interface() {
            TypeKind::Interface
        } else {
            match self.extends() {
                TypeName::Enum => TypeKind::Enum,
                TypeName::Delegate => TypeKind::Delegate,
                TypeName::Struct => TypeKind::Struct,
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

    pub fn is_nullable(&self) -> bool {
        matches!(
            self.kind(),
            TypeKind::Interface | TypeKind::Class | TypeKind::Delegate
        )
    }

    pub fn enclosing_type(&self) -> Option<Self> {
        self.row
            .file
            .equal_range(TableIndex::NestedClass, 0, self.row.row + 1)
            .map(NestedClass)
            .next()
            .map(|nested| nested.enclosing_type())
    }

    pub fn class_layout(&self) -> Option<ClassLayout> {
        self.row
            .file
            .equal_range(TableIndex::ClassLayout, 2, self.row.row + 1)
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

    // May need to "re-resolve" the TypeDef as it may point to an arch-specific
    // definition. This lets the TypeTree be built for a specific architecture
    // without accidentally pulling in the wrong definition.
    pub fn resolve(&self) -> Self {
        TypeReader::get().expect_type_def(self.type_name())
    }
}

struct Bases(TypeDef);

impl Iterator for Bases {
    type Item = TypeDef;

    fn next(&mut self) -> Option<Self::Item> {
        let extends = self.0.extends();

        if extends == TypeName::Object {
            None
        } else {
            self.0 = TypeReader::get().expect_type_def(extends);
            Some(self.0.clone())
        }
    }
}
