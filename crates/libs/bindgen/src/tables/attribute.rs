use super::*;

pub trait AttributeExt {
    fn args(&self) -> Vec<(String, Value)>;
}

impl AttributeExt for Attribute {
    fn args(&self) -> Vec<(String, Value)> {
        self.value()
    }
}
