#[derive(Default)]
pub struct FieldAttributes(pub usize);

#[derive(Default)]
pub struct MethodAttributes(pub usize);

#[derive(Default)]
pub struct MethodImplAttributes(pub usize);

#[derive(Default)]
pub struct ParamAttributes(pub usize);

#[derive(Default)]
pub struct PInvokeAttributes(pub usize);

#[derive(Default)]
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
    pub fn public(&self) -> bool {
        self.0 & 0x1 != 0
    }
    pub fn set_public(&mut self) {
        self.0 |= 0x1;
    }

    pub fn explicit_layout(&self) -> bool {
        self.0 & 0x10 != 0
    }
    pub fn set_explicit_layout(&mut self) {
        self.0 |= 0x10;
    }

    pub fn get_abstract(&self) -> bool {
        self.0 & 0x80 != 0
    }
    pub fn set_abstract(&mut self) {
        self.0 |= 0x80;
    }

    pub fn winrt(&self) -> bool {
        self.0 & 0x4000 != 0
    }
    pub fn set_winrt(&mut self) {
        self.0 |= 0x4000;
    }

    pub fn interface(&self) -> bool {
        self.0 & 0x20 != 0
    }
    pub fn set_interface(&mut self) {
        self.0 |= 0x20
    }
}
