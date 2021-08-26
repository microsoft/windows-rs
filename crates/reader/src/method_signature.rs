// TODO: split the parsing code from teh gen code

use super::*;

pub struct MethodSignature {
    pub params: Vec<MethodParam>,
    pub return_type: Option<Signature>,
}

#[derive(Clone)]
pub struct MethodParam {
    pub param: Param,
    pub signature: Signature,
}

impl MethodSignature {
    pub fn dependencies(&self, include: TypeInclude) -> Vec<TypeEntry> {
        self.return_type
            .iter()
            .map(|s| s.definition(include))
            .chain(self.params.iter().map(|p| p.signature.definition(include)))
            .flatten()
            .collect()
    }

    pub fn has_query_interface(&self) -> bool {
        self.return_type.as_ref().map_or(false, |signature| {
            if signature.kind == ElementType::HRESULT && self.params.len() >= 2 {
                let guid = &self.params[self.params.len() - 2];
                let object = &self.params[self.params.len() - 1];

                if guid.signature.kind == ElementType::Guid
                    && !guid.param.flags().output()
                    && object.signature.kind == ElementType::Void
                    && object.param.is_com_out_ptr()
                {
                    return true;
                }
            }

            false
        })
    }

    pub fn has_retval(&self) -> bool {
        self.return_type.as_ref().map_or(false, |signature| {
            if signature.kind == ElementType::HRESULT
                && self.params.last().map_or(false, |param| param.is_retval())
            {
                return self.params[..self.params.len() - 1].iter().all(|param| {
                    let flags = param.param.flags();
                    flags.input() && !flags.output()
                });
            }

            false
        })
    }

    pub fn has_udt_return(&self) -> bool {
        self.return_type
            .as_ref()
            .map_or(false, |signature| signature.is_udt())
    }
}

impl MethodParam {
    fn is_retval(&self) -> bool {
        if self.signature.pointers == 0 {
            return false;
        }

        let flags = self.param.flags();

        // TODO: NativeArrayInfo indicates and array parameter #479
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
        self.param.is_input()
            && !self.signature.is_array
            && self.signature.pointers == 0
            && self.signature.kind.is_convertible()
    }
}
