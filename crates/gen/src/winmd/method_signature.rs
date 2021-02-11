use super::*;

#[derive(Debug)]
pub struct MethodSignature {
    pub params: Vec<MethodParam>,
    pub return_type: Option<Signature>,
}

#[derive(Debug)]
pub struct MethodParam {
    pub param: Param,
    pub signature: Signature,
}

impl MethodSignature {
    pub fn dependencies(&self) -> Vec<TypeDef> {
        self.return_type
            .iter()
            .flat_map(|s| s.dependencies())
            .chain(self.params.iter().flat_map(|p| p.signature.dependencies()))
            .collect()
    }
}
