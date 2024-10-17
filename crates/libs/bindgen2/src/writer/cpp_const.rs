use super::*;

impl CppConst {
    pub fn write(&self, writer: &Writer) -> TokenStream {
        let name = to_ident(self.field.name());

        // TODO: is this even needed?
        if let Some(guid) = self.field.guid_attribute() {
            return writer.write_cpp_const_guid(name, &guid);
        }

        let field_ty = self.field.ty(None).to_const_type();

        let mut dependencies = Dependencies::new();

        if writer.config.package {
            self.dependencies(&mut dependencies, &writer.config);
        }

        let cfg = writer.write_cfg(self.field, self.def.namespace(), dependencies, false);

        if let Some(constant) = self.field.constant() {
            let constant_ty = constant.ty();

            if field_ty == constant_ty {
                if field_ty == Type::String {
                    let crate_name = writer.write_core();
                    let value = constant.value().write();

                    // TODO: if writer.no_deps then write these literals out as byte strings?
                    if is_ansi_encoding(self.field) {
                        quote! {
                            #cfg
                            pub const #name: #crate_name PCSTR = #crate_name s!(#value);
                        }
                    } else {
                        quote! {
                            #cfg
                            pub const #name: #crate_name PCWSTR = #crate_name w!(#value);
                        }
                    }
                } else {
                    // TODO: typed value
                    let ty = field_ty.write(writer);
                    let value = constant.value().write();

                    quote! {
                        #cfg
                        pub const #name: #ty = #value;
                    }
                }
            } else {
                let underlying_ty = underlying_type(&field_ty);
                let ty = field_ty.write(writer);
                let mut value = constant.value().write();

                if underlying_ty == constant_ty {
                    if is_signed_error(&field_ty) {
                        if let Value::I32(signed) = constant.value() {
                            value = format!("0x{:X}_u32 as _", signed).into();
                        }
                    }
                } else {
                    value = quote! { #value as _ };
                }

                quote! {
                    #cfg
                    pub const #name: #ty = #value;
                }
            }
        } else if let Some(attribute) = self.field.find_attribute("ConstantAttribute") {
            let args = attribute.args();
            let Some((_, Value::String(input))) = args.first() else {
                panic!()
            };

            let Type::Item(Item::CppStruct(item)) = &field_ty else {
                panic!()
            };

            let mut input = input.as_str();
            let mut tokens = quote! {};

            for field in item.def.fields() {
                let (value, rest) = writer.field_initializer(field, input);
                input = rest;
                tokens.combine(value);
            }

            let ty = field_ty.write(writer);

            quote! {
                #cfg
                pub const #name: #ty = #ty { #tokens };
            }
        } else {
            panic!()
        }
    }
}

fn is_ansi_encoding(row: Field) -> bool {
    row.find_attribute("NativeEncodingAttribute").is_some_and(|attribute| matches!(attribute.args().first(), Some((_, Value::String(encoding))) if encoding == "ansi"))
}

fn is_signed_error(ty: &Type) -> bool {
    match ty {
        Type::HRESULT => true,
        Type::Item(item) => TypeName(item.namespace(), item.name()) == TypeName::NTSTATUS,
        _ => false,
    }
}

fn underlying_type(ty: &Type) -> Type {
    match ty {
        Type::Item(item) => item.underlying_type(),
        Type::HRESULT => Type::I32,
        _ => ty.clone(),
    }
}
