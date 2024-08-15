use super::*;
use core::mem::transmute_copy;
use std::sync::{Arc, RwLock};

/// A type that you can use to declare and implement an event of a specified delegate type.
///
/// The implementation is thread-safe and designed to avoid contention between events being
/// raised and delegates being added or removed.
pub struct Event<T: Interface> {
    delegates: RwLock<Option<Arc<[Delegate<T>]>>>,
}

impl<T: Interface> Event<T> {
    /// Creates a new, empty `Event<T>`.
    pub fn new() -> Self {
        Self {
            delegates: RwLock::new(None),
        }
    }

    /// Registers a delegate with the event object.
    pub fn add(&self, delegate: &T) -> Result<i64> {
        let mut guard = self.delegates.write().unwrap();
        let mut new_list = if let Some(old_delegates) = guard.as_ref() {
            let mut new_list = Vec::with_capacity(old_delegates.len() + 1);
            new_list.extend_from_slice(&old_delegates);
            new_list
        } else {
            Vec::with_capacity(1)
        };

        let new_delegate = Delegate::new(delegate)?;
        let token = new_delegate.to_token();
        new_list.push(new_delegate);

        *guard = Some(new_list.into());
        Ok(token)
    }

    /// Revokes a delegate's registration from the event object.
    pub fn remove(&self, token: i64) {
        let mut guard = self.delegates.write().unwrap();
        if let Some(old_delegates) = guard.as_ref() {
            // `self.delegates` is only modified if the token is found.
            if let Some(i) = old_delegates
                .iter()
                .position(|old_delegate| old_delegate.to_token() == token)
            {
                let mut new_list = Vec::with_capacity(old_delegates.len() - 1);
                new_list.extend_from_slice(&old_delegates[..i]);
                new_list.extend_from_slice(&old_delegates[i + 1..]);
                *guard = Some(new_list.into());
            }
        }
    }

    /// Clears the event, removing all delegates.
    pub fn clear(&self) {
        let mut guard = self.delegates.write().unwrap();
        *guard = None;
    }

    /// Invokes all of the event object's registered delegates with the provided callback.
    pub fn call<F: FnMut(&T) -> Result<()>>(&self, mut callback: F) {
        let delegates = {
            let guard = self.delegates.read().unwrap();
            if let Some(delegates) = guard.as_ref() {
                delegates.clone()
            } else {
                // No delegates to call.
                return;
            }
            // <-- lock is released here
        };

        for delegate in delegates.iter() {
            if let Err(error) = delegate.call(&mut callback) {
                const RPC_E_SERVER_UNAVAILABLE: HRESULT = HRESULT(-2147023174); // HRESULT_FROM_WIN32(RPC_S_SERVER_UNAVAILABLE)
                if matches!(
                    error.code(),
                    imp::RPC_E_DISCONNECTED | imp::JSCRIPT_E_CANTEXECUTE | RPC_E_SERVER_UNAVAILABLE
                ) {
                    self.remove(delegate.to_token());
                }
            }
        }
    }
}

/// Holds either a direct or indirect reference to a delegate. A direct reference is typically
/// agile while an indirect reference is an agile wrapper.
#[derive(Clone)]
enum Delegate<T> {
    Direct(T),
    Indirect(AgileReference<T>),
}

impl<T: Interface> Delegate<T> {
    /// Creates a new `Delegate<T>`, containing a suitable reference to the specified delegate.
    fn new(delegate: &T) -> Result<Self> {
        if delegate.cast::<imp::IAgileObject>().is_ok() {
            Ok(Self::Direct(delegate.clone()))
        } else {
            Ok(Self::Indirect(AgileReference::new(delegate)?))
        }
    }

    /// Returns an encoded token to identify the delegate.
    fn to_token(&self) -> i64 {
        unsafe {
            match self {
                Self::Direct(delegate) => imp::EncodePointer(transmute_copy(delegate)) as i64,
                Self::Indirect(delegate) => imp::EncodePointer(transmute_copy(delegate)) as i64,
            }
        }
    }

    /// Invokes the delegates with the provided callback.
    fn call<F: FnMut(&T) -> Result<()>>(&self, mut callback: F) -> Result<()> {
        match self {
            Self::Direct(delegate) => callback(delegate),
            Self::Indirect(delegate) => callback(&delegate.resolve()?),
        }
    }
}
