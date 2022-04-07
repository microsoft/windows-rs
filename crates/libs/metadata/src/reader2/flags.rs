pub struct FieldAttributes(pub usize);
pub struct MethodAttributes(pub usize);
pub struct MethodImplAttributes(pub usize);
pub struct ParamAttributes(pub usize);
pub struct PInvokeAttributes(pub usize);
pub struct TypeAttributes(pub usize);

impl FieldAttributes {
    pub fn literal(&self) -> bool {
        self.0 & 0x40 != 0
    }
}

impl MethodAttributes {
    pub fn special(&self) -> bool {
        self.0 & 0x800 != 0
    }
}

impl MethodImplAttributes {
    pub fn preserve_sig(&self) -> bool {
        self.0 & 0x80 != 0
    }
}

impl ParamAttributes {
    pub fn input(&self) -> bool {
        self.0 & 0x1 != 0
    }
    pub fn output(&self) -> bool {
        self.0 & 0x2 != 0
    }
    pub fn optional(&self) -> bool {
        self.0 & 0x10 != 0
    }
}

impl PInvokeAttributes {
    pub fn last_error(&self) -> bool {
        self.0 & 0x40 != 0
    }
}

impl TypeAttributes {
    pub fn union(&self) -> bool {
        self.0 & 0x10 != 0
    }
    pub fn winrt(&self) -> bool {
        self.0 & 0x800 != 0
    }
    pub fn interface(&self) -> bool {
        self.0 & 0x20 != 0
    }
}
