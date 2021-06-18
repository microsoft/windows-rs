use super::*;

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
            .map(ElementType::GenericParam)
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
        self.methods()
            .find(|m| m.name() == "Invoke")
            .expect("`Invoke` method not found")
    }

    // TODO: get rid of the definition functions
    pub fn definition(&self, include: TypeInclude) -> Vec<TypeEntry> {
        let mut definition = vec![TypeEntry{ include, def: TypeRow::TypeDef(self.clone())}];

        for generic in &self.generics {
            definition.append(&mut generic.definition(include));
        }

        definition
    }

    pub fn default_interface(&self) -> Self {
        for interface in self.interface_impls() {
            if interface.is_default() {
                if let ElementType::TypeDef(def) = interface.generic_interface(&self.generics) {
                    return def;
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
        self.interface_impls().filter_map(move |i| {
            if let ElementType::TypeDef(def) = i.generic_interface(&self.generics) {
                Some(def)
            } else {
                None
            }
        })
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

            if self.generics.is_empty() {
                let name = format_name(name);
                quote! { #namespace#name }
            } else {
                let colon_separated = if turbo || !namespace.as_str().is_empty() {
                    quote! { :: }
                } else {
                    quote! {}
                };

                let name = format_name(&name[..name.len() - 2]);
                let generics = self.generics.iter().map(|g| g.gen_name(gen));
                quote! { #namespace#name#colon_separated<#(#generics),*> }
            }
        }
    }

    pub fn gen_guid(&self, gen: &Gen) -> TokenStream {
        if self.generics.is_empty() {
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

    // TODO: use `include` to determin whether to gen full or minimal definition
    pub fn gen(&self, gen: &Gen, include: TypeInclude) -> TokenStream {
        // TODO: all the cloning here is ridiculous
        match self.kind() {
            TypeKind::Interface => {
                if self.is_winrt() {
                    types::Interface(self.clone().with_generics()).gen(gen, include)
                } else {
                    types::ComInterface(self.clone()).gen(gen, include)
                }
            }
            TypeKind::Class => types::Class(self.clone().with_generics()).gen(gen, include),
            TypeKind::Enum => types::Enum(self.clone()).gen(gen, include),
            TypeKind::Struct => types::Struct(self.clone()).gen(gen),
            TypeKind::Delegate => {
                if self.is_winrt() {
                    types::Delegate(self.clone().with_generics()).gen(gen)
                } else {
                    types::Callback(self.clone()).gen(gen)
                }
            }
        }
    }

    pub fn gen_abi_type(&self, gen: &Gen) -> TokenStream {
        match self.kind() {
            TypeKind::Enum => self.gen_name(gen),
            TypeKind::Struct => {
                if self.is_blittable() {
                    self.gen_name(gen)
                } else {
                    self.gen_abi_name(gen)
                }
            }
            _ => quote! { ::windows::RawPtr },
        }
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
        // TODO: interface and class definitions should be Full and only their method dependnecies should be minimal
        // so that inheritance works correctly
        match self.kind() {
            TypeKind::Interface => {
                if include == TypeInclude::Minimal {
                    return Vec::new();
                }

                let interfaces = self.interfaces().map(|i| TypeEntry{include: TypeInclude::Full, def: TypeRow::TypeDef(i.clone())});

                let methods = self.methods().map(|m| m.dependencies(TypeInclude::Minimal)).flatten();

                interfaces.chain(methods).collect()
            }
            TypeKind::Class => {
                if include == TypeInclude::Minimal {
                    return Vec::new();
                }

                let generics = self.generics.iter().map(|g| g.definition(TypeInclude::Minimal));
                let interfaces = self.interfaces().map(|i| i.definition(TypeInclude::Full));
                let bases = self.bases().map(|b| b.definition(TypeInclude::Full));

                let factories = self.attributes().filter_map(|attribute| {
                    match attribute.name() {
                        "StaticAttribute" | "ActivatableAttribute" | "ComposableAttribute" => {
                            for (_, arg) in attribute.args() {
                                if let parser::ConstantValue::TypeDef(def) = arg {
                                    return Some(TypeEntry{include: TypeInclude::Full, def: TypeRow::TypeDef(def.clone())});
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
                let reader = TypeReader::get();
                let mut dependencies = vec![];

                match self.full_name() {
                    ("Windows.Win32.Foundation", "BSTR") => {
                        dependencies.push(TypeEntry{include:TypeInclude::Minimal, def: reader.result_type_row(
                            "Windows.Win32.System.OleAutomation",
                            "SysFreeString",
                        )});
                        dependencies.push(TypeEntry{include:TypeInclude::Minimal, def: reader.result_type_row(
                            "Windows.Win32.System.OleAutomation",
                            "SysAllocStringLen",
                        )});
                        dependencies.push(
                            TypeEntry{include:TypeInclude::Minimal, def: reader.result_type_row(
                                "Windows.Win32.System.OleAutomation",
                                "SysStringLen",
                            ),
                            });
                    }
                    ("Windows.Foundation.Numerics", "Matrix3x2") => {
                        dependencies.push(TypeEntry{include:TypeInclude::Minimal, def: reader.result_type_row(
                            "Windows.Win32.Graphics.Direct2D",
                            "D2D1MakeRotateMatrix",
                        )});
                    }
                    _ => {
                        // TODO: doesn't seem to pick up nested type dependencies e.g. D3D11_DEPTH_STENCIL_VIEW_DESC
                        dependencies.extend(self.fields().map(|f| f.definition(TypeInclude::Minimal)).flatten());

                        if let Some(dependency) = self.is_convertible_to() {
                            dependencies.push(TypeEntry{include:TypeInclude::Minimal, def: TypeRow::TypeDef(dependency)});
                        }
                    }
                }

                dependencies
            }
            TypeKind::Delegate => self.invoke_method().dependencies(TypeInclude::Minimal),
        }
    }

    pub fn is_udt(&self) -> bool {
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
        if self.kind() != TypeKind::Struct {
            return false;
        }

        if self.flags().explicit() {
            true
        } else {
            self.fields().any(|f| f.signature().is_explicit())
        }
    }

    pub fn type_signature(&self) -> String {
        match self.kind() {
            TypeKind::Interface => self.interface_signature(),
            TypeKind::Class => format!(
                "rc({}.{};{})",
                self.namespace(),
                self.name(),
                self.default_interface().interface_signature()
            ),
            TypeKind::Enum => format!(
                "enum({}.{};{})",
                self.namespace(),
                self.name(),
                self.underlying_type().type_signature()
            ),
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
                let blob = &mut field.blob();
                blob.read_unsigned();
                blob.read_modifiers();

                blob.read_expected(0x1D);
                blob.read_modifiers();

                return TypeReader::get().type_from_blob(blob, &[]);
            }
        }

        unexpected!();
    }

    pub fn gen_signature(&self, signature: &str) -> TokenStream {
        let signature = Literal::byte_string(signature.as_bytes());

        if self.generics.is_empty() {
            return quote! { ::windows::ConstBuffer::from_slice(#signature) };
        }

        let generics = self.generics.iter().enumerate().map(|(index, g)| {
            let g = g.gen();
            let semi = if index != self.generics.len() - 1 {
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

    pub fn gen_phantoms<'a>(&'a self) -> impl Iterator<Item = TokenStream> + 'a {
        // TODO: this gen really only needs the GenericParam name
        self.generics.iter().map(move |g| {
            let g = g.gen();
            quote! { ::std::marker::PhantomData::<#g> }
        })
    }

    pub fn gen_constraints(&self) -> TokenStream {
        // TODO: this gen really only needs the GenericParam name
        self.generics
            .iter()
            .map(|g| {
                let g = g.gen();
                quote! { #g: ::windows::RuntimeType + 'static, }
            })
            .collect()
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

    pub fn flags(&self) -> TypeFlags {
        TypeFlags(self.row.u32(0))
    }

    pub fn name(&self) -> &'static str {
        self.row.str(1)
    }

    pub fn namespace(&self) -> &'static str {
        self.row.str(2)
    }

    pub fn full_name(&self) -> (&'static str, &'static str) {
        (self.namespace(), self.name())
    }

    pub fn extends(&self) -> (&'static str, &'static str) {
        let extends = self.row.u32(3);

        if extends == 0 {
            ("", "")
        } else {
            TypeDefOrRef::decode(self.row.file, extends).full_name()
        }
    }

    pub fn bases(&self) -> impl Iterator<Item = TypeDef> {
        Bases(self.clone())
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

    pub fn nested_types(&self) -> Option<&BTreeMap<&'static str, tables::TypeDef>> {
        TypeReader::get().nested_types(self)
    }

    pub fn attributes(&self) -> impl Iterator<Item = Attribute> {
        self.row
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

    pub fn is_convertible_to(&self) -> Option<TypeDef> {
        self.attributes().find_map(|attribute| {
            if attribute.name() == "AlsoUsableForAttribute" {
                if let Some((_, ConstantValue::String(name))) = attribute.args().get(0) {
                    // TODO: https://github.com/microsoft/win32metadata/issues/389
                    return Some(TypeReader::get().resolve_type_def(self.namespace(), name));
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
                if self.full_name() == ("Windows.Win32.Foundation", "BSTR") {
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

    // TODO: hash?
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
        TypeReader::get().resolve_type_def(self.namespace(), self.name())
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
