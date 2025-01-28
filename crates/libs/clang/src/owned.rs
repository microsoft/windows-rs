pub trait Free {
    fn free(&mut self);
}

#[repr(transparent)]
#[derive(PartialEq, Eq, Default)]
pub struct Owned<T: Free>(T);

impl<T: Free> Owned<T> {
    pub(crate) fn new(x: T) -> Self {
        Self(x)
    }
}

impl<T: Free> Drop for Owned<T> {
    fn drop(&mut self) {
        self.0.free();
    }
}

impl<T: Free> std::ops::Deref for Owned<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Free> std::ops::DerefMut for Owned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Free + std::fmt::Display> std::fmt::Display for Owned<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

impl<T: Free + std::fmt::Debug> std::fmt::Debug for Owned<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{:?}", self.0)
    }
}
