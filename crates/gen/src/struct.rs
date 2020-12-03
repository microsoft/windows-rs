use crate::*;
use squote::{format_ident, quote, Literal, TokenStream};

// TODO: have Struct handle both WinRT and Win32 structs - it's almost all the same code.
#[derive(Debug)]
pub struct Struct {
    pub name: TypeName,
    pub fields: Vec<(String, TypeKind)>,
    pub signature: String,
}

impl Struct {
    pub fn from_type_name(name: TypeName) -> Self {
        let signature = name.struct_signature();
        let mut fields = Vec::new();

        for field in name.def.fields() {
            let field_name = to_snake(field.name(), MethodKind::Normal);
            let kind = TypeKind::from_field( field, &name.namespace);

            fields.push((field_name, kind));
        }

        Self {
            name,
            fields,
            signature,
        }
    }

    pub fn dependencies(&self) -> Vec<winmd::TypeDef> {
        self.fields
            .iter()
            .flat_map(|i| i.1.dependencies())
            .collect()
    }

    pub fn gen(&self) -> TokenStream {
        let name = self.name.gen();
        let abi_ident = format_ident!("{}_abi", self.name.name);
        let signature = Literal::byte_string(&self.signature.as_bytes());

        let fields = self.fields.iter().map(|(name, kind)| {
            let name = format_ident(&name);
            let kind = kind.gen_field();
            quote! {
                pub #name: #kind
            }
        });

        let abi = self.fields.iter().map(|field| field.1.gen_abi());

        // TODO: unroll these traits - it's too expensive to call derive macro.
        // https://github.com/microsoft/winrt-rs/issues/353

        quote! {
            #[repr(C)]
            #[derive(::std::fmt::Debug, ::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq)]
            pub struct #name {
                #(#fields),*
            }
            impl ::std::cmp::Eq for #name {}
            #[repr(C)]
            pub struct #abi_ident(#(#abi),*);
            unsafe impl ::winrt::RuntimeType for #name {
                type DefaultType = Self;
                const SIGNATURE: ::winrt::ConstBuffer = ::winrt::ConstBuffer::from_slice(#signature);
            }
            unsafe impl ::winrt::Abi for #name {
                type Abi = #abi_ident;
            }
        }
    }
}
