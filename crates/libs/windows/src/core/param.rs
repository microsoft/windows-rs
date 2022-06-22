use super::{Abi, Borrowed};

pub enum Param<'a, T: Abi> {
    Owned(T),
    Borrowed(Borrowed<'a, T>),
}

impl<'a, T: Abi> Param<'a, T> {
    pub fn abi(&self) -> T::Abi {
        let borrowed = match self {
            Self::Owned(item) => item.into(),
            Self::Borrowed(borrowed) => borrowed.clone(),
        };
        borrowed.abi()
    }
}

impl<'a, T, U: Abi> From<&'a T> for Param<'a, U>
where
    &'a T: Into<Borrowed<'a, U>>,
{
    fn from(borrowed: &'a T) -> Self {
        Param::Borrowed(borrowed.into())
    }
}
