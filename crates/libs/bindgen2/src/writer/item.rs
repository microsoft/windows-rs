use super::*;

impl Item {
    pub fn write(&self, writer: &Writer) -> TokenStream {
        match self {
            Item::Struct(item) => item.write(writer),
            Item::Enum(item) => item.write(writer),
            Item::Interface(item) => item.write(writer),
            Item::CppStruct(item) => item.write(writer),
            Item::CppEnum(item) => item.write(writer),
            Item::CppFn(item) => item.write(writer),
            Item::CppConst(item) => item.write(writer),
            Item::CppDelegate(item) => item.write(writer),
            _ => quote! {},
        }
    }

    pub fn write_name(&self, writer: &Writer) -> TokenStream {
        match self {
            Item::CppInterface(_) if writer.config.sys => quote! { *mut core::ffi::c_void },
            Item::Interface(item) => item.write_name(writer),
            _ => {
                let name = to_ident(self.name());
                let namespace = writer.write_namespace(self.namespace());
                quote! { #namespace #name }
            }
        }
    }
}
