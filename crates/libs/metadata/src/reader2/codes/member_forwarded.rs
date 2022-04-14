use super::*;

#[derive(Clone)]
pub enum MemberForwarded {
    MethodDef(MethodDef),
}

impl MemberForwarded {
    pub fn encode(&self) -> usize {
        (match self {
            Self::MethodDef(value) => ((value.0.row + 1) << 1) | 1,
        }) as _
    }
}
