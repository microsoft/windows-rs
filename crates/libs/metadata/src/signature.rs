use super::*;

pub struct Signature {
    pub params: Vec<MethodParam>,
    pub return_type: Option<Type>,
    pub return_param: Option<Param>,
    pub preserve_sig: bool,
}

#[derive(Clone)]
pub struct MethodParam {
    pub def: Param,
    pub ty: Type,
}

impl Signature {
    pub fn kind(&self) -> SignatureKind {
        if self.preserve_sig {
            return SignatureKind::PreserveSig;
        }

        if let Some(return_type) = &self.return_type {
            match return_type {
                Type::HRESULT => {
                    if self.params.len() >= 2 {
                        let guid = &self.params[self.params.len() - 2];
                        let object = &self.params[self.params.len() - 1];

                        if guid.ty == Type::ConstPtr((Box::new(Type::GUID), 1)) && !guid.def.flags().output() && object.ty == Type::MutPtr((Box::new(Type::Void), 2)) && object.def.is_com_out_ptr() {
                            if object.def.flags().optional() {
                                return SignatureKind::QueryOptional;
                            } else {
                                return SignatureKind::Query;
                            }
                        }
                    }

                    if self.params.last().map_or(false, |param| param.is_retval())
                        && self.params[..self.params.len() - 1].iter().all(|param| {
                            let flags = param.def.flags();
                            flags.input() && !flags.output()
                        })
                    {
                        return SignatureKind::ResultValue;
                    }

                    return SignatureKind::ResultVoid;
                }
                Type::TypeDef(def) if def.type_name() == TypeName::NTSTATUS => {
                    return SignatureKind::ResultVoid;
                }
                _ if return_type.is_udt() => {
                    return SignatureKind::ReturnStruct;
                }
                _ => return SignatureKind::PreserveSig,
            }
        }

        SignatureKind::ReturnVoid
    }

    pub fn size(&self) -> usize {
        self.params.iter().fold(0, |sum, param| sum + param.ty.size())
    }
}

impl MethodParam {
    fn is_retval(&self) -> bool {
        // The Win32 metadata uses `RetValAttribute` to call out retval methods but it is employed
        // very sparingly, so this heuristic is used to apply the transformation more uniformly.

        if self.def.is_retval() {
            return true;
        }

        if !self.ty.is_pointer() {
            return false;
        }

        if self.ty.is_void() {
            return false;
        }

        let flags = self.def.flags();

        // TODO: NativeArrayInfo indicates an array parameter #479
        if flags.input() || !flags.output() || self.def.array_info() {
            return false;
        }

        // TODO: find a way to treat this like COM interface result values.
        !self.ty.is_callback()
    }

    pub fn is_convertible(&self) -> bool {
        self.def.flags().input() && !self.ty.is_winrt_array() && !self.ty.is_pointer() && self.ty.is_convertible()
    }
}
