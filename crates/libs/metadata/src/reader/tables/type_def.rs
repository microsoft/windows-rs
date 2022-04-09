use super::*;
pub use std::collections::BTreeSet;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct TypeDef {
    pub row: Row,
    pub generics: Vec<Type>,
}

impl From<Row> for TypeDef {
    fn from(row: Row) -> Self {
        Self { row, generics: Vec::new() }
    }
}

impl TypeDef {
    #[must_use]
    pub fn with_generics(mut self) -> Self {
        self.generics = self.generic_params().map(|generic| Type::GenericParam(generic.name())).collect();
        self
    }

    pub fn is_callback(&self) -> bool {
        !self.is_winrt() && self.kind() == TypeKind::Delegate
    }

    pub fn has_default_constructor(&self) -> bool {
        for attribute in self.attributes() {
            if attribute.name() == "ActivatableAttribute" {
                if attribute.args().iter().any(|arg| matches!(arg.1, ConstantValue::TypeDef(_))) {
                    continue;
                } else {
                    return true;
                }
            }
        }

        false
    }

    pub fn invoke_method(&self) -> MethodDef {
        self.methods().find(|m| m.name() == "Invoke").expect("`Invoke` method not found")
    }

    pub fn default_interface(&self) -> Option<Self> {
        for interface in self.interface_impls() {
            if interface.is_default() {
                if let Type::TypeDef(def) = interface.generic_interface(&self.generics) {
                    return Some(def);
                }
            }
        }

        None
    }

    pub fn interfaces(&self) -> impl Iterator<Item = Self> + '_ {
        self.interface_impls().filter_map(move |i| if let Type::TypeDef(def) = i.generic_interface(&self.generics) { Some(def) } else { None })
    }

    pub fn required_interfaces(&self) -> Vec<Self> {
        fn walk(result: &mut Vec<TypeDef>, parent: &TypeDef) {
            for child in parent.interface_impls() {
                if let Type::TypeDef(def) = child.generic_interface(&parent.generics) {
                    if !result.iter().any(|element| element == &def) {
                        walk(result, &def);
                        result.push(def);
                    }
                }
            }
        }

        let mut result = vec![];
        walk(&mut result, self);
        result.sort_by(|a, b| a.name().cmp(b.name()));
        result
    }

    pub fn class_interfaces(&self) -> Vec<(Self, InterfaceKind)> {
        fn walk(result: &mut Vec<(TypeDef, InterfaceKind)>, parent: &TypeDef, is_base: bool) {
            for child in parent.interface_impls() {
                if let Type::TypeDef(def) = child.generic_interface(&parent.generics) {
                    let kind = if !is_base && child.is_default() {
                        InterfaceKind::Default
                    } else if child.is_overridable() {
                        continue;
                    } else if is_base {
                        InterfaceKind::Base
                    } else {
                        InterfaceKind::NonDefault
                    };

                    let mut found = false;

                    for existing in result.iter_mut() {
                        if existing.0 == def {
                            found = true;

                            if kind == InterfaceKind::Default {
                                existing.1 = kind;
                            }
                        }
                    }

                    if !found {
                        walk(result, &def, is_base);
                        result.push((def, kind));
                    }
                }
            }
        }

        let mut result = vec![];
        walk(&mut result, self, false);

        for base in self.bases() {
            walk(&mut result, &base, true);
        }

        for attribute in self.attributes() {
            match attribute.name() {
                "StaticAttribute" | "ActivatableAttribute" => {
                    for (_, arg) in attribute.args() {
                        if let ConstantValue::TypeDef(def) = arg {
                            result.push((def, InterfaceKind::Static));
                            break;
                        }
                    }
                }
                "ComposableAttribute" => {
                    if let Some(def) = attribute.composable_type() {
                        result.push((def, InterfaceKind::Composable));
                    }
                }
                _ => {}
            }
        }

        // TODO: need to sort by hierarchy as well?
        result.sort_by(|a, b| a.0.name().cmp(b.0.name()));
        result
    }

    pub fn size(&self) -> usize {
        if self.kind() == TypeKind::Struct {
            if self.is_union() {
                self.fields().map(|field| field.get_type(Some(self)).size()).max().unwrap_or(1)
            } else {
                self.fields().fold(0, |sum, field| sum + field.get_type(Some(self)).size())
            }
        } else {
            4
        }
    }

    pub fn is_deprecated(&self) -> bool {
        self.has_attribute("DeprecatedAttribute")
    }

    pub fn is_handle(&self) -> bool {
        self.has_attribute("NativeTypedefAttribute")
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
        match self.kind() {
            TypeKind::Enum => true,
            TypeKind::Struct => self.is_handle() && self.type_name() != TypeName::BSTR,
            _ => false,
        }
    }

    pub fn is_union(&self) -> bool {
        self.row.u32(0) & 0b1_0000 != 0
    }

    pub fn has_union(&self) -> bool {
        fn has_union(def: &TypeDef) -> bool {
            if def.kind() != TypeKind::Struct {
                return false;
            }

            if def.is_union() {
                true
            } else {
                def.fields().any(|f| f.get_type(Some(def)).has_union())
            }
        }

        if has_union(self) {
            return true;
        }

        if let Some(entry) = TypeReader::get().get_type_entry(self.type_name()) {
            for def in entry {
                if let Type::TypeDef(def) = def {
                    if has_union(def) {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn has_pack(&self) -> bool {
        fn has_pack(def: &TypeDef) -> bool {
            if def.kind() != TypeKind::Struct {
                return false;
            }

            if def.class_layout().is_some() {
                true
            } else {
                def.fields().any(|f| f.get_type(Some(def)).has_pack())
            }
        }

        if has_pack(self) {
            return true;
        }

        if let Some(entry) = TypeReader::get().get_type_entry(self.type_name()) {
            for def in entry {
                if let Type::TypeDef(def) = def {
                    if has_pack(def) {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn type_signature(&self) -> String {
        match self.kind() {
            TypeKind::Interface => self.interface_signature(),
            TypeKind::Class => format!("rc({};{})", self.type_name(), self.default_interface().unwrap_or_else(|| panic!("`{}` does not have a default interface.", self.type_name())).interface_signature()),
            TypeKind::Enum => format!("enum({};{})", self.type_name(), self.underlying_type().type_signature()),
            TypeKind::Struct => {
                let mut result = format!("struct({}", self.type_name());

                for field in self.fields() {
                    result.push(';');
                    result.push_str(&field.get_type(Some(self)).type_signature());
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

    pub fn underlying_type(&self) -> Type {
        if let Some(field) = self.fields().next() {
            if let Some(constant) = field.constant() {
                return constant.value_type();
            } else {
                return field.get_type(Some(self));
            }
        }

        unimplemented!();
    }

    fn interface_signature(&self) -> String {
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
                Type::TypeDef(def) => Some(def),
                Type::IUnknown => None,
                Type::IInspectable => {
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

    pub fn vtable_types(&self) -> Vec<Type> {
        let mut result = Vec::new();

        if self.is_winrt() {
            result.push(Type::IUnknown);
            if self.kind() != TypeKind::Delegate {
                result.push(Type::IInspectable);
            }
        } else {
            let mut next = self.clone();

            while let Some(base) = next.interface_impls().map(|i| i.generic_interface(&[])).next() {
                match base {
                    Type::TypeDef(ref def) => {
                        next = def.clone();
                        result.insert(0, base);
                    }
                    Type::IInspectable => {
                        result.insert(0, Type::IUnknown);
                        result.insert(1, Type::IInspectable);
                        break;
                    }
                    Type::IUnknown => {
                        result.insert(0, Type::IUnknown);
                        break;
                    }
                    _ => unimplemented!(),
                }
            }
        }

        result
    }

    pub fn fields(&self) -> impl Iterator<Item = Field> {
        self.row.list(4, TableIndex::Field).map(Field)
    }

    pub fn methods(&self) -> impl Iterator<Item = MethodDef> {
        self.row.list(5, TableIndex::MethodDef).map(MethodDef)
    }

    pub fn generic_params(&self) -> impl Iterator<Item = GenericParam> {
        self.row.file.equal_range(TableIndex::GenericParam, 2, TypeOrMethodDef::TypeDef(self.clone()).encode()).map(GenericParam)
    }

    pub fn interface_impls(&self) -> impl Iterator<Item = InterfaceImpl> {
        self.row.file.equal_range(TableIndex::InterfaceImpl, 0, self.row.row + 1).map(InterfaceImpl)
    }

    pub fn nested_types(&self) -> Option<&BTreeMap<&'static str, TypeDef>> {
        TypeReader::get().nested_types(self)
    }

    pub fn attributes(&self) -> impl Iterator<Item = Attribute> {
        self.row.file.attributes(HasAttribute::TypeDef(self.clone()))
    }

    fn has_attribute(&self, name: &str) -> bool {
        self.attributes().any(|attribute| attribute.name() == name)
    }

    pub fn has_flags(&self) -> bool {
        // Win32 enums use the Flags attribute. WinRT enums don't have the Flags attribute but are paritioned merely based
        // on whether they are signed.
        self.has_attribute("FlagsAttribute") || (self.is_winrt() && self.underlying_type() == Type::U32)
    }

    pub fn is_exclusive(&self) -> bool {
        self.has_attribute("ExclusiveToAttribute")
    }

    pub fn is_scoped(&self) -> bool {
        self.is_winrt() || self.has_attribute("ScopedEnumAttribute")
    }

    pub fn is_api_contract(&self) -> bool {
        self.has_attribute("ApiContractAttribute")
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

    pub fn invalid_values(&self) -> Vec<i64> {
        self.attributes()
            .filter_map(|attribute| {
                if attribute.name() == "InvalidHandleValueAttribute" {
                    if let Some((_, ConstantValue::I64(value))) = attribute.args().get(0) {
                        return Some(*value);
                    }
                }
                None
            })
            .collect()
    }

    pub fn is_convertible_to(&self) -> Option<&Type> {
        self.attributes().find_map(|attribute| {
            if attribute.name() == "AlsoUsableForAttribute" {
                if let Some((_, ConstantValue::String(name))) = attribute.args().get(0) {
                    return TypeReader::get().get_type((self.namespace(), name.as_str()));
                }
            }

            None
        })
    }

    pub fn is_public_composable(&self) -> bool {
        self.attributes().any(|attribute| attribute.name() == "ComposableAttribute" && attribute.args().iter().any(|arg| matches!(arg, (_, ConstantValue::I32(2)))))
    }

    pub fn is_blittable(&self) -> bool {
        match self.kind() {
            TypeKind::Struct => {
                // TODO: should be "if self.can_drop().is_some() {" once win32metadata bugs are fixed (423, 422, 421, 389)
                if self.type_name() == TypeName::BSTR {
                    false
                } else {
                    self.fields().all(|f| f.is_blittable(Some(self)))
                }
            }
            TypeKind::Enum => true,
            TypeKind::Delegate => !self.is_winrt(),
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

    pub fn guid(&self) -> GUID {
        GUID::from_attributes(self.attributes()).expect("TypeDef::guid")
    }

    pub fn is_nullable(&self) -> bool {
        match self.kind() {
            TypeKind::Interface | TypeKind::Class => true,
            TypeKind::Delegate => self.is_winrt(),
            _ => false,
        }
    }

    pub fn can_implement(&self) -> bool {
        for attribute in self.attributes() {
            if attribute.name() == "ExclusiveToAttribute" {
                for (_, arg) in attribute.args() {
                    if let ConstantValue::TypeDef(def) = arg {
                        for child in def.interface_impls() {
                            if child.is_overridable() {
                                if let Type::TypeDef(def) = child.generic_interface(&def.generics) {
                                    if def.name() == self.name() {
                                        return true;
                                    }
                                }
                            }
                        }
                    }
                }

                return false;
            }
        }

        true
    }

    pub fn enclosing_type(&self) -> Option<Self> {
        self.row.file.equal_range(TableIndex::NestedClass, 0, self.row.row + 1).map(NestedClass).next().map(|nested| nested.enclosing_type())
    }

    pub fn class_layout(&self) -> Option<ClassLayout> {
        self.row.file.equal_range(TableIndex::ClassLayout, 2, self.row.row + 1).map(ClassLayout).next()
    }

    pub fn overridable_interfaces(&self) -> Vec<TypeDef> {
        self.interface_impls().filter(|interface| interface.is_overridable()).map(|interface| interface.interface().resolve(None)).chain(self.bases().next().iter().flat_map(|base| base.overridable_interfaces())).collect()
    }

    pub fn overridable_methods(&self) -> BTreeSet<&'static str> {
        self.overridable_interfaces().iter().flat_map(|interface| interface.methods().map(|method| method.name())).collect()
    }

    pub fn async_kind(&self) -> AsyncKind {
        match self.type_name() {
            TypeName::IAsyncAction => AsyncKind::Action,
            TypeName::IAsyncActionWithProgress => AsyncKind::ActionWithProgress,
            TypeName::IAsyncOperation => AsyncKind::Operation,
            TypeName::IAsyncOperationWithProgress => AsyncKind::OperationWithProgress,
            _ => AsyncKind::None,
        }
    }

    pub fn cfg(&self) -> Cfg {
        let mut cfg = Cfg::new();
        self.combine_cfg(&mut cfg);
        cfg.add_attributes(self.attributes());
        cfg
    }

    pub fn impl_cfg(&self) -> Cfg {
        let mut cfg = Cfg::new();

        fn combine(def: &TypeDef, cfg: &mut Cfg) {
            def.combine_cfg(cfg);

            for method in def.methods() {
                method.combine_cfg(cfg);
            }
        }

        combine(self, &mut cfg);

        for def in self.vtable_types() {
            if let Type::TypeDef(def) = def {
                combine(&def, &mut cfg);
            }
        }

        if self.is_winrt() {
            for def in self.required_interfaces() {
                combine(&def, &mut cfg);
            }
        }

        cfg.add_attributes(self.attributes());
        cfg
    }

    pub(crate) fn combine_cfg(&self, cfg: &mut Cfg) {
        for generic in &self.generics {
            generic.combine_cfg(cfg);
        }

        if cfg.add_type(self) {
            match self.kind() {
                TypeKind::Class => {
                    if let Some(def) = self.default_interface() {
                        cfg.add_feature(def.namespace());
                    }
                }
                TypeKind::Interface => {
                    if !self.is_winrt() {
                        for def in self.vtable_types() {
                            if let Type::TypeDef(def) = def {
                                cfg.add_feature(def.namespace());
                            }
                        }
                    }
                }
                TypeKind::Struct => {
                    self.fields().for_each(|field| field.combine_cfg(Some(self), cfg));

                    if let Some(entry) = TypeReader::get().get_type_entry(self.type_name()) {
                        for def in entry {
                            if let Type::TypeDef(def) = def {
                                def.combine_cfg(cfg);
                            }
                        }
                    }
                }
                TypeKind::Delegate => self.invoke_method().combine_cfg(cfg),
                _ => {}
            }
        }
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
