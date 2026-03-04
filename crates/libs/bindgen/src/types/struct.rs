use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Struct {
    pub def: TypeDef,
}

impl Struct {
    pub fn type_name(&self) -> TypeName {
        self.def.type_name()
    }

    pub fn write_name(&self, config: &Config) -> TokenStream {
        self.type_name().write(config, &[])
    }

    pub fn write(&self, config: &Config) -> TokenStream {
        let name = to_ident(self.def.name());

        let fields: Vec<_> = self
            .def
            .fields()
            .map(|field| (field.name(), field.field_type(None, config.reader)))
            .collect();

        let is_copyable = fields.iter().all(|(_, ty)| ty.is_copyable(config.reader));

        let mut derive = DeriveWriter::new(config, self.type_name());
        derive.extend(["Clone"]);

        if is_copyable {
            derive.extend(["Copy"]);
        }

        if !config.sys {
            derive.extend(["Default", "Debug", "PartialEq"]);
        }

        let fields = fields.iter().map(|(name, ty)| {
            let name = to_ident(name);
            let ty = ty.write_default(config);
            quote! { pub #name: #ty, }
        });

        let win_traits = if config.sys {
            quote! {}
        } else {
            let type_kind = if is_copyable {
                quote! { CopyType }
            } else {
                quote! { CloneType }
            };

            let signature = Literal::byte_string(&self.runtime_signature(config.reader));

            quote! {
                impl windows_core::TypeKind for #name {
                    type TypeKind = windows_core::#type_kind;
                }
                impl windows_core::RuntimeType for #name {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(#signature);
                }
            }
        };

        quote! {
            #[repr(C)]
            #derive
            pub struct #name {
                #(#fields)*
            }
            #win_traits
        }
    }

    pub fn runtime_signature(&self, reader: &Reader) -> String {
        let mut signature = format!("struct({}", self.def.type_name());
        for field in self.def.fields() {
            signature.push(';');
            signature.push_str(&field.field_type(None, reader).runtime_signature(reader));
        }
        signature.push(')');
        signature
    }

    pub fn is_copyable(&self, reader: &Reader) -> bool {
        self.def
            .fields()
            .all(|field| field.field_type(None, reader).is_copyable(reader))
    }

    pub fn size(&self, reader: &Reader) -> usize {
        let mut sum = 0;
        for field in self.def.fields() {
            let ty = field.field_type(None, reader);
            let size = ty.size(reader);
            let align = ty.align(reader);
            sum = (sum + (align - 1)) & !(align - 1);
            sum += size;
        }
        sum
    }

    pub fn align(&self, reader: &Reader) -> usize {
        self.def
            .fields()
            .map(|field| field.field_type(None, reader).align(reader))
            .max()
            .unwrap_or(1)
    }
}

impl Dependencies for Struct {
    fn combine(&self, dependencies: &mut TypeMap, reader: &Reader) {
        for field in self.def.fields() {
            field.field_type(None, reader).combine(dependencies, reader);
        }
    }
}
