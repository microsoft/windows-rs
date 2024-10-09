use super::*;

impl Writer {
    pub fn write_cpp_const(&self, item: &CppConst) -> TokenStream {
        let name = to_ident(item.field.name());
        let field_ty = item.field.ty(None).to_const_type();

        let mut dependencies = Dependencies::new();

        if self.package {
            item.dependencies(&mut dependencies, self.minimal);
        }

        let cfg = self.write_cfg(item.field, item.def.namespace(), dependencies, false);

        if let Some(constant) = item.field.constant() {
            //let constant_ty = constant.ty();

            let ty = self.write_default_name(&field_ty);
            let value = self.write_value(&constant.value());

            quote! {
                #cfg
                pub const #name: #ty = #value;
            }
        } else {
            quote! {}
        }
    }
}
