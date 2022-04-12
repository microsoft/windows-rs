use super::*;

#[derive(Clone)]
pub enum HasConstant {
    Field(Field),
}

impl HasConstant {
    pub fn encode(&self) -> usize {
        (match self {
            Self::Field(value) => ((value.0.row + 1) << 2),
        }) as _
    }
}
