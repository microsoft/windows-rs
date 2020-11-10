// https://github.com/microsoft/winrt-rs/issues/292

// This is needed for structs with IReference fields, to avoid having to use Option<IReference<T>>
// partly because that's lame and partly because structs must be Default but IReference<T> is not.

#[repr(transparent)]
#[derive(Clone, PartialEq, Default)]
pub struct Option<T: RuntimeType + 'static>(std::option::Option<IReference<T>>);

impl<T: RuntimeType + 'static> Option<T> {
    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }

    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }

    pub fn expect(self, msg: &str) -> T {
        self.0.expect(msg).value().expect(msg)
    }

    pub fn unwrap(self) -> T {
        self.0.unwrap().value().unwrap()
    }

    // pub fn unwrap_or(self, default: T) -> T {
    //     self.0.map_or(default, |r|r.value().unwrap_or(default))
    // }

    // pub fn unwrap_or_else<F: FnOnce() -> T>(self, f: F) -> T {
    //     self.0.map_or_else(f, |r|r.value().unwrap_or_else(|_|f()))
    // }
}

impl<T: RuntimeType> From<std::option::Option<T>> for Option<T> {
    fn from(from: std::option::Option<T>) -> Self {
        // TODO: need implementation support to complete this
    }
}
