// TODO: consider renaming these XxxxFlags
pub struct MethodAttributes(pub(crate) u32);
pub struct TypeAttributes(pub(crate) u32);

#[derive(Default)]
pub struct ParamAttributes(pub(crate) u32);

impl MethodAttributes {
    pub fn special(&self) -> bool {
        self.0 & 0b1000_0000_0000 != 0
    }
}

impl TypeAttributes {
    pub fn windows_runtime(&self) -> bool {
        self.0 & 0b100_0000_0000_0000 != 0
    }
    pub fn interface(&self) -> bool {
        self.0 & 0b10_0000 != 0
    }
}

impl ParamAttributes {
    pub fn input(&self) -> bool {
        self.0 & 0b1 != 0
    }
}
