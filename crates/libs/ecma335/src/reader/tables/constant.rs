use super::*;

impl Constant<'_> {
    pub fn ty(&self) -> u8 {
        self.usize(0).try_into().unwrap()
    }

    pub fn parent(&self) -> HasConstant {
        self.decode(1)
    }

    pub fn value(&self) -> Blob {
        self.blob(2)
    }
}
