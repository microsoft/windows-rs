use super::*;
use windows_core::*;

#[derive(Clone)]
pub struct AgileDelegate<T: Interface>(Delegate<T>);

#[derive(Clone)]
enum Delegate<T: Interface> {
    Direct(T),
    Indirect(AgileReference<T>),
}

// Direct references have declared agility; indirect references are marshaled on each call.
unsafe impl<T: Interface> Send for AgileDelegate<T> {}
unsafe impl<T: Interface> Sync for AgileDelegate<T> {}

impl<T: Interface> AgileDelegate<T> {
    pub fn new(delegate: &T) -> Result<Self> {
        if delegate.cast::<IAgileObject>().is_ok() {
            Ok(Self(Delegate::Direct(delegate.clone())))
        } else {
            AgileReference::new(delegate).map(|reference| Self(Delegate::Indirect(reference)))
        }
    }

    pub fn to_token(&self) -> i64 {
        match &self.0 {
            Delegate::Direct(delegate) => encode_pointer(delegate.as_raw()),
            Delegate::Indirect(delegate) => encode_pointer(delegate.as_raw()),
        }
    }

    pub fn call<F: FnMut(&T) -> Result<()>>(&self, mut callback: F) -> Result<()> {
        match &self.0 {
            Delegate::Direct(delegate) => callback(delegate),
            Delegate::Indirect(delegate) => callback(&delegate.resolve()?),
        }
    }
}
