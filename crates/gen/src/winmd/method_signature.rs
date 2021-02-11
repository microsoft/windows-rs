use super::*;

#[derive(Debug)]
pub struct MethodSignature {
    pub params: Vec<(Param, Signature)>,
    pub return_type: Option<Signature>,
}
