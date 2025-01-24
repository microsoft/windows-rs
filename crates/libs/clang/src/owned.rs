use super::*;

pub trait Free {
    unsafe fn free(&mut self);
}

#[repr(transparent)]
#[derive(PartialEq, Eq, Default, Debug)]
pub struct Owned<T: Free>(T);

impl<T: Free> Owned<T> {
    pub unsafe fn new(x: T) -> Self {
        Self(x)
    }
}

impl<T: Free> Drop for Owned<T> {
    fn drop(&mut self) {
        unsafe { self.0.free() };
    }
}

impl<T: Free> Deref for Owned<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Free> DerefMut for Owned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
