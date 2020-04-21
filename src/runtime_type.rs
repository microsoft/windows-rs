/// RuntimeType is used to constrain WinRT generic types to WinRT types
pub trait RuntimeType {
    type Abi;

    fn abi(&self) -> Self::Abi;
    fn set_abi(&mut self) -> *mut Self::Abi;
}

impl RuntimeType for bool {
    type Abi = Self;
    fn abi(&self) -> Self::Abi {
        *self
    }
    fn set_abi(&mut self) -> *mut Self::Abi {
        self as *mut Self::Abi
    }
}

impl RuntimeType for i8 {
    type Abi = Self;
    fn abi(&self) -> Self::Abi {
        *self
    }
    fn set_abi(&mut self) -> *mut Self::Abi {
        self as *mut Self::Abi
    }
}

impl RuntimeType for u8 {
    type Abi = Self;
    fn abi(&self) -> Self::Abi {
        *self
    }
    fn set_abi(&mut self) -> *mut Self::Abi {
        self as *mut Self::Abi
    }
}

impl RuntimeType for i16 {
    type Abi = Self;
    fn abi(&self) -> Self::Abi {
        *self
    }
    fn set_abi(&mut self) -> *mut Self::Abi {
        self as *mut Self::Abi
    }
}

impl RuntimeType for u16 {
    type Abi = Self;
    fn abi(&self) -> Self::Abi {
        *self
    }
    fn set_abi(&mut self) -> *mut Self::Abi {
        self as *mut Self::Abi
    }
}

impl RuntimeType for i32 {
    type Abi = Self;
    fn abi(&self) -> Self::Abi {
        *self
    }
    fn set_abi(&mut self) -> *mut Self::Abi {
        self as *mut Self::Abi
    }
}

impl RuntimeType for u32 {
    type Abi = Self;
    fn abi(&self) -> Self::Abi {
        *self
    }
    fn set_abi(&mut self) -> *mut Self::Abi {
        self as *mut Self::Abi
    }
}

impl RuntimeType for i64 {
    type Abi = Self;
    fn abi(&self) -> Self::Abi {
        *self
    }
    fn set_abi(&mut self) -> *mut Self::Abi {
        self as *mut Self::Abi
    }
}

impl RuntimeType for u64 {
    type Abi = Self;
    fn abi(&self) -> Self::Abi {
        *self
    }
    fn set_abi(&mut self) -> *mut Self::Abi {
        self as *mut Self::Abi
    }
}

impl RuntimeType for f32 {
    type Abi = Self;
    fn abi(&self) -> Self::Abi {
        *self
    }
    fn set_abi(&mut self) -> *mut Self::Abi {
        self as *mut Self::Abi
    }
}

impl RuntimeType for f64 {
    type Abi = Self;
    fn abi(&self) -> Self::Abi {
        *self
    }
    fn set_abi(&mut self) -> *mut Self::Abi {
        self as *mut Self::Abi
    }
}
