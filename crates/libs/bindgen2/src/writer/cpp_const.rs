use super::*;

impl Writer {
    pub fn write_cpp_const(&self, item: &CppConst) -> TokenStream {
        let name = to_ident(item.field.name());

        // TODO: is this even needed?
        if let Some(guid) = item.field.guid_attribute() {
            return self.write_cpp_const_guid(name, &guid);
        }

        let field_ty = item.field.ty(None).to_const_type();

        let mut dependencies = Dependencies::new();

        if self.package {
            item.dependencies(&mut dependencies, self.minimal);
        }

        let cfg = self.write_cfg(item.field, item.def.namespace(), dependencies, false);

        if let Some(constant) = item.field.constant() {
            let constant_ty = constant.ty();

            if field_ty == constant_ty {
                if field_ty == Type::String {
                    let crate_name = self.write_crate();
                    let value = self.write_value(&constant.value());

                    // TODO: if self.no_deps then write these literals out as byte strings?
                    if is_ansi_encoding(item.field) {
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
                    let ty = self.write_name(&field_ty);
                    let value = self.write_value(&constant.value());

                    quote! {
                        #cfg
                        pub const #name: #ty = #value;
                    }
                }
            } else {
                let underlying_ty = underlying_type(&field_ty);
                let ty = self.write_name(&field_ty);
                let mut value = self.write_value(&constant.value());

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
        } else {
            quote! {
                #cfg
                pub const #name: i32 = 0;
            }
        }
    }

    pub fn write_cpp_const_guid(&self, name: TokenStream, value: &GUID) -> TokenStream {
        let crate_name = self.write_crate();
        let value: TokenStream = format!("0x{:08x?}_{:04x?}_{:04x?}_{:02x?}{:02x?}_{:02x?}{:02x?}{:02x?}{:02x?}{:02x?}{:02x?}", value.0, value.1, value.2, value.3, value.4, value.5, value.6, value.7, value.8, value.9, value.10).into();

        quote! {
            pub const #name: #crate_name GUID = #crate_name GUID::from_u128(#value);
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
