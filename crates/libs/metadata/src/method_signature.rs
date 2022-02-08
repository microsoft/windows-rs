use super::*;

pub struct MethodSignature {
    pub params: Vec<MethodParam>,
    pub return_sig: Option<Signature>,
    pub return_param: Option<Param>,
    pub preserve_sig: bool,
}

#[derive(Clone)]
pub struct MethodParam {
    pub param: Param,
    pub signature: Signature,
}

impl MethodSignature {
    pub fn kind(&self) -> SignatureKind {
        if self.preserve_sig {
            return SignatureKind::PreserveSig;
        }

        if let Some(return_sig) = &self.return_sig {
            match &return_sig.kind {
                ElementType::HRESULT => {
                    if self.params.len() >= 2 {
                        let guid = &self.params[self.params.len() - 2];
                        let object = &self.params[self.params.len() - 1];

                        if guid.signature.kind == ElementType::GUID && !guid.param.flags().output() && object.signature.kind == ElementType::Void && object.param.is_com_out_ptr() {
                            if object.param.is_optional() {
                                return SignatureKind::QueryOptional;
                            } else {
                                return SignatureKind::Query;
                            }
                        }
                    }

                    if self.params.last().map_or(false, |param| param.is_retval())
                        && self.params[..self.params.len() - 1].iter().all(|param| {
                            let flags = param.param.flags();
                            flags.input() && !flags.output()
                        })
                    {
                        return SignatureKind::ResultValue;
                    }

                    return SignatureKind::ResultVoid;
                }
                ElementType::TypeDef(def) if def.type_name() == TypeName::NTSTATUS => {
                    return SignatureKind::ResultVoid;
                }
                _ if return_sig.is_udt() => {
                    return SignatureKind::ReturnStruct;
                }
                _ => return SignatureKind::PreserveSig,
            }
        }

        SignatureKind::ReturnVoid
    }

    pub fn size(&self) -> usize {
        self.params.iter().fold(0, |sum, param| sum + param.signature.size())
    }
}

impl MethodParam {
    fn is_retval(&self) -> bool {
        if self.signature.pointers == 0 {
            return false;
        }

        let flags = self.param.flags();

        // TODO: NativeArrayInfo indicates an array parameter #479
        if flags.input() || !flags.output() || self.param.array_info() {
            return false;
        }

        match &self.signature.kind {
            ElementType::Void => false,
            ElementType::TypeDef(def) => def.kind() != TypeKind::Delegate,
            _ => true,
        }
    }

    pub fn is_convertible(&self) -> bool {
        self.param.is_input() && !self.signature.is_array && self.signature.pointers == 0 && self.signature.kind.is_convertible()
    }
}
