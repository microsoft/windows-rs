use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Struct {
    pub def: TypeDef,
}

impl Struct {
    pub fn write(&self, writer: &Writer) -> TokenStream {
        let name = to_ident(self.def.name());

        let fields: Vec<_> = self
            .def
            .fields()
            .map(|field| (field.name(), field.ty(None)))
            .collect();

        let is_copyable = fields.iter().all(|(_, ty)| ty.is_copyable());

        let derive = quote! { Clone, Debug, Default, PartialEq };
        // TODO: add any user-defined derive names

        let fields = fields.iter().map(|(name, ty)| {
            let name = to_ident(name);
            let ty = ty.write_default(writer);
            quote! { pub #name: #ty, }
        });

        let type_kind = if is_copyable {
            quote! { CopyType }
        } else {
            quote! { CloneType }
        };

        let signature = Literal::byte_string(&self.runtime_signature());

        quote! {
            #[repr(C)]
            #[derive(#derive)]
            pub struct #name {
                #(#fields)*
            }
            impl windows_core::TypeKind for #name {
                type TypeKind = windows_core::#type_kind;
            }
            impl windows_core::RuntimeType for #name {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(#signature);
            }
        }
    }

    pub fn runtime_signature(&self) -> String {
        let mut signature = format!("struct({}.{}", self.def.namespace(), self.def.name());
        for field in self.def.fields() {
            signature.push(';');
            signature.push_str(&field.ty(None).runtime_signature());
        }
        signature.push(')');
        signature
    }

    pub fn dependencies(&self, dependencies: &mut Dependencies) {
        if dependencies.insert(self.def.namespace(), self.def.name()) {
            for field in self.def.fields() {
                field.ty(None).dependencies(dependencies);
            }
        }
    }
}
