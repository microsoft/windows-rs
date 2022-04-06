pub struct FieldAttributes(pub usize);
pub struct MethodAttributes(pub usize);
pub struct MethodImplAttributes(pub usize);
pub struct ParamAttributes(pub usize);
pub struct PInvokeAttributes(pub usize);
pub struct TypeAttributes(pub usize);

impl FieldAttributes {}

impl MethodAttributes {}

impl MethodImplAttributes {}

impl ParamAttributes {}

impl PInvokeAttributes {
    pub fn last_error(&self) -> bool {
        self.0 & 0x0040 != 0
    }
}

impl TypeAttributes {
    pub fn input(&self) -> bool {
        self.0 & 0x0001 != 0
    }
    pub fn output(&self) -> bool {
        self.0 & 0x0002 != 0
    }
    pub fn optional(&self) -> bool {
        self.0 & 0x0010 != 0
    }
}
