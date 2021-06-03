use super::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct TypeDef(Row, Vec<ElementType>);

impl std::hash::Hash for TypeDef {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl From<Row> for TypeDef {
    fn from(row: Row) -> Self {
        Self(row, Vec::new())
    }
}

impl TypeDef {
    pub fn generics(&self) -> &Vec<ElementType> {
        &self.1
    }

    pub fn from_blob(blob: &mut Blob, generics: &[ElementType]) -> Self {
        blob.read_unsigned();

        let mut def = TypeDefOrRef::decode(blob.file, blob.read_unsigned())
            .resolve()
            .clone();
        let args = blob.read_unsigned();

        for _ in 0..args {
            def.1.push(ElementType::from_blob(blob, generics));
        }

        def
    }

    // TODO: with_generics?
    pub fn from_type_def(def: &tables::TypeDef, generics: Vec<ElementType>) -> Self {
        let mut def = def.clone();

        if generics.is_empty() {
            def.1 = def
                .generic_params()
                .map(ElementType::GenericParam)
                .collect();
        } else {
            def.1 = generics;
        }

        def
    }

    pub fn has_default_constructor(&self) -> bool {
        for attribute in self.attributes() {
            if attribute.name() == "ActivatableAttribute" {
                if attribute
                    .args()
                    .iter()
                    .any(|arg| matches!(arg.1, parser::ConstantValue::TypeDef(_)))
                {
                    continue;
                } else {
                    return true;
                }
            }
        }

        false
    }

    pub fn invoke_method(&self) -> tables::MethodDef {
        self
            .methods()
            .find(|m| m.name() == "Invoke")
            .expect("`Invoke` method not found")
    }

    // TODO: get rid of the definition functions
    pub fn definition(&self) -> Vec<ElementType> {
        let mut definition = vec![ElementType::from_type_def(self, Vec::new())];

        for generic in &self.1 {
            definition.append(&mut generic.definition());
        }

        definition
    }

    pub fn default_interface(&self) -> Self {
        for interface in self.interface_impls() {
            if interface.is_default() {
                if let Some(result) = interface.generic_interface(&self.1) {
                    return result;
                }
            }
        }

        panic!(
            "`{}.{}` does not have a default interface.",
            self.namespace(),
            self.name()
        );
    }

    pub fn interfaces(&self) -> impl Iterator<Item = Self> + '_ {
        self.interface_impls()
            .filter_map(move |i| i.generic_interface(&self.1))
    }

    pub fn gen_name(&self, gen: &Gen) -> TokenStream {
        self.format_name(gen, to_ident, false)
    }

    pub fn gen_abi_name(&self, gen: &Gen) -> TokenStream {
        self.format_name(gen, to_abi_ident, false)
    }

    pub fn gen_turbo_abi_name(&self, gen: &Gen) -> TokenStream {
        self.format_name(gen, to_abi_ident, true)
    }

    fn format_name<F>(&self, gen: &Gen, format_name: F, turbo: bool) -> TokenStream
    where
        F: FnOnce(&str) -> Ident,
    {
        let namespace = self.namespace();

        if namespace.is_empty() {
            let name = format_name(&self.scoped_name());
            quote! { #name }
        } else {
            let name = self.name();
            let namespace = gen.namespace(self.namespace());

            if self.1.is_empty() {
                let name = format_name(name);
                quote! { #namespace#name }
            } else {
                let colon_separated = if turbo || !namespace.as_str().is_empty() {
                    quote! { :: }
                } else {
                    quote! {}
                };

                let name = format_name(&name[..name.len() - 2]);
                let generics = self.1.iter().map(|g| g.gen_name(gen));
                quote! { #namespace#name#colon_separated<#(#generics),*> }
            }
        }
    }

    pub fn gen_guid(&self, gen: &Gen) -> TokenStream {
        if self.1.is_empty() {
            match Guid::from_attributes(self.attributes()) {
                Some(guid) => {
                    let guid = guid.gen();

                    quote! {
                        ::windows::Guid::from_values(#guid)
                    }
                }
                None => {
                    quote! {
                        ::windows::Guid::zeroed()
                    }
                }
            }
        } else {
            let tokens = self.gen_name(gen);

            quote! {
                ::windows::Guid::from_signature(<#tokens as ::windows::RuntimeType>::SIGNATURE)
            }
        }
    }

    pub fn type_signature(&self) -> String {
        match self.kind() {
            TypeKind::Interface => {
                let guid = Guid::from_attributes(self.attributes()).expect("Interface guid not found");

                if self.1.is_empty() {
                    format!("{{{:#?}}}", guid)
                } else {
                    let mut result = format!("pinterface({{{:#?}}}", guid);
        
                    for generic in &self.1 {
                        result.push(';');
                        result.push_str(&generic.type_signature());
                    }
        
                    result.push(')');
                    result
                }
            }
            TypeKind::Class => {
                let default = self.default_interface();

                format!(
                    "rc({}.{};{})",
                    self.namespace(),
                    self.name(),
                    default.interface_signature()
                )
            }
            TypeKind::Enum => {
                let underlying_type = match self.underlying_type() {
                    ElementType::I32 => "i4",
                    ElementType::U32 => "u4",
                    _ => unexpected!(),
                };
        
                format!(
                    "enum({}.{};{})",
                    self.namespace(),
                    self.name(),
                    underlying_type
                )
            }
            TypeKind::Struct => {
                let mut result = format!("struct({}.{}", self.namespace(), self.name());

                for field in self.fields() {
                    result.push(';');
                    result.push_str(&field.signature().kind.type_signature());
                }
        
                result.push(')');
                result
            }
            TypeKind::Delegate => {
                if self.1.is_empty() {
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
                let blob = &mut field.blob();
                blob.read_unsigned();
                blob.read_modifiers();

                blob.read_expected(0x1D);
                blob.read_modifiers();

                return ElementType::from_blob(blob, &[]);
            }
        }

        unexpected!();
    }

    pub fn gen_signature(&self, signature: &str, gen: &Gen) -> TokenStream {
        let signature = Literal::byte_string(signature.as_bytes());

        if self.1.is_empty() {
            return quote! { ::windows::ConstBuffer::from_slice(#signature) };
        }

        let generics = self.1.iter().enumerate().map(|(index, g)| {
            let g = g.gen(gen);
            let semi = if index != self.1.len() - 1 {
                Some(quote! {
                    .push_slice(b";")
                })
            } else {
                None
            };

            quote! {
                .push_other(<#g as ::windows::RuntimeType>::SIGNATURE)
                #semi
            }
        });

        quote! {
            {
                ::windows::ConstBuffer::new()
                .push_slice(b"pinterface(")
                .push_slice(#signature)
                .push_slice(b";")
                #(#generics)*
                .push_slice(b")")
            }
        }
    }

    pub fn gen_phantoms<'a>(&'a self, gen: &'a Gen) -> impl Iterator<Item = TokenStream> + 'a {
        self.1.iter().map(move |g| {
            let g = g.gen(gen);
            quote! { ::std::marker::PhantomData::<#g> }
        })
    }

    pub fn gen_constraints(&self, gen: &Gen) -> TokenStream {
        self.1
            .iter()
            .map(|g| {
                let g = g.gen(gen);
                quote! { #g: ::windows::RuntimeType + 'static, }
            })
            .collect()
    }

    pub fn interface_signature(&self) -> String {
        let guid = self.guid();

        if self.1.is_empty() {
            format!("{{{:#?}}}", guid)
        } else {
            let mut result = format!("pinterface({{{:#?}}}", guid);

            for generic in &self.1 {
                result.push(';');
                result.push_str(&generic.type_signature());
            }

            result.push(')');
            result
        }
    }

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

    pub fn generic_params(&self) -> impl Iterator<Item = GenericParam> {
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

    pub fn nested_types(&self) -> Option<&BTreeMap<&'static str, tables::TypeDef>> {
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

    pub fn is_blittable(&self) -> bool {
        // TODO: should be "if self.can_drop().is_some() {" once win32metadata bugs are fixed (423, 422, 421, 389)
        if self.full_name() == ("Windows.Win32.System.OleAutomation", "BSTR") {
            false
        } else {
            self.fields().all(|f| f.is_blittable())
        }
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
            if let Some(nested_types) = enclosing_type.nested_types() {
                for (index, (nested_type, _)) in nested_types.iter().enumerate() {
                    if *nested_type == self.name() {
                        return format!("{}_{}", enclosing_type.scoped_name(), index);
                    }
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
            .map(|interface| interface.interface().resolve().clone())
            .chain(
                self.bases()
                    .next()
                    .iter()
                    .flat_map(|base| base.overridable_interfaces()),
            )
            .collect()
    }

    // TODO: hash?
    pub fn overridable_methods(&self) -> BTreeSet<&'static str> {
        self.overridable_interfaces()
            .iter()
            .flat_map(|interface| interface.methods().map(|method| method.name()))
            .collect()
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
            self.0 = TypeReader::get().resolve_type_def(namespace, name).clone();
            Some(self.0.clone())
        }
    }
}
