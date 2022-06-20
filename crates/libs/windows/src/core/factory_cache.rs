use super::*;
use bindings::*;
use core::marker::PhantomData;
use core::sync::atomic::{AtomicPtr, Ordering};

#[doc(hidden)]
pub struct FactoryCache<C, I> {
    shared: AtomicPtr<core::ffi::c_void>,
    _c: PhantomData<C>,
    _i: PhantomData<I>,
}

impl<C, I> FactoryCache<C, I> {
    pub const fn new() -> Self {
        Self { shared: AtomicPtr::new(core::ptr::null_mut()), _c: PhantomData, _i: PhantomData }
    }
}

impl<C: RuntimeName, I: Interface> FactoryCache<C, I> {
    pub fn call<R, F: FnOnce(&I) -> Result<R>>(&self, callback: F) -> Result<R> {
        loop {
            // Attempt to load a previously cached factory pointer.
            let ptr = self.shared.load(Ordering::Relaxed);

            // If a pointer is found, the cache is primed and we're good to go.
            if !ptr.is_null() {
                return callback(unsafe { core::mem::transmute(&ptr) });
            }

            // Otherwise, we load the factory the usual way.
            let factory = factory::<C, I>()?;

            // If the factory is agile, we can safely cache it.
            if factory.cast::<IAgileObject>().is_ok() {
                if self.shared.compare_exchange_weak(core::ptr::null_mut(), factory.as_raw(), Ordering::Relaxed, Ordering::Relaxed).is_ok() {
                    core::mem::forget(factory);
                }
            } else {
                // Otherwise, for non-agile factories we simply use the factory
                // and discard after use as it is not safe to cache.
                return callback(&factory);
            }
        }
    }
}

// This is safe because `FactoryCache` only holds agile factory pointers, which are safe to cache and share between threads.
unsafe impl<C, I> ::core::marker::Sync for FactoryCache<C, I> {}

/// Attempts to load the factory interface for the given WinRT class
pub fn factory<C: RuntimeName, I: Interface>() -> Result<I> {
    let mut factory: Option<I> = None;
    let name = HSTRING::from(C::NAME);

    let code = if let Ok(function) = unsafe { delay_load(b"combase.dll\0", b"RoGetActivationFactory\0") } {
        unsafe {
            let function: RoGetActivationFactory = core::mem::transmute(function);
            let mut code = function(core::mem::transmute_copy(&name), &I::IID, &mut factory as *mut _ as *mut _);

            // If RoGetActivationFactory fails because combase hasn't been loaded yet then load combase
            // automatically so that it "just works" for apartment-agnostic code.
            if code == CO_E_NOTINITIALIZED {
                if let Ok(mta) = delay_load(b"ole32.dll\0", b"CoIncrementMTAUsage\0") {
                    let mta: CoIncrementMTAUsage = core::mem::transmute(mta);
                    let mut cookie = core::ptr::null_mut();
                    let _ = mta(&mut cookie);
                }

                // Now try a second time to get the activation factory via the OS.
                code = function(core::mem::transmute_copy(&name), &I::IID, &mut factory as *mut _ as *mut _);
            }

            code
        }
    } else {
        CLASS_E_CLASSNOTAVAILABLE
    };

    // If this succeeded then return the resulting factory interface.
    if code.is_ok() {
        return code.and_some(factory);
    }

    // If not, first capture the error information from the failure above so that we
    // can ultimately return this error information if all else fails.
    let original: Error = code.into();

    // Now attempt to find the factory's implementation heuristically.
    if let Some(i) = search_path(C::NAME, |library| unsafe { get_activation_factory(library, &name) }) {
        i.cast()
    } else {
        Err(original)
    }
}

// Remove the suffix until a match is found appending `.dll\0` at the end
///
/// For example, if the class name is
/// "A.B.TypeName" then the attempted load order will be:
///   1. A.B.dll
///   2. A.dll
fn search_path<F, R>(mut path: &str, mut callback: F) -> Option<R>
where
    F: FnMut(&[u8]) -> Result<R>,
{
    let suffix = b".dll\0";
    let mut library = vec![0; path.len() + suffix.len()];
    while let Some(pos) = path.rfind('.') {
        path = &path[..pos];
        library.truncate(path.len() + suffix.len());
        library[..path.len()].copy_from_slice(path.as_bytes());
        library[path.len()..].copy_from_slice(suffix);

        if let Ok(r) = callback(&library) {
            return Some(r);
        }
    }

    None
}

unsafe fn get_activation_factory(library: &[u8], name: &HSTRING) -> Result<IGenericFactory> {
    let function = delay_load(library, b"DllGetActivationFactory\0")?;
    let function: DllGetActivationFactory = core::mem::transmute(function);
    let mut abi = core::mem::MaybeUninit::zeroed();
    function(core::mem::transmute_copy(name), abi.as_mut_ptr()).from_abi(abi)
}

type CoIncrementMTAUsage = extern "system" fn(cookie: *mut *mut core::ffi::c_void) -> HRESULT;
type RoGetActivationFactory = extern "system" fn(hstring: core::mem::ManuallyDrop<HSTRING>, interface: &GUID, result: *mut *mut core::ffi::c_void) -> HRESULT;
type DllGetActivationFactory = extern "system" fn(name: core::mem::ManuallyDrop<HSTRING>, factory: *mut *mut core::ffi::c_void) -> HRESULT;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dll_search() {
        let path = "A.B.TypeName";

        // Test library successfully found
        let mut results = Vec::new();
        let end_result = search_path(path, |library| {
            results.push(library.to_vec());
            if library == &b"A.dll\0"[..] {
                Ok(42)
            } else {
                Err(Error::OK)
            }
        });
        assert!(matches!(end_result, Some(42)));
        assert_eq!(results, vec![b"A.B.dll\0".to_vec(), b"A.dll\0".to_vec()]);

        // Test library never successfully found
        let mut results = Vec::new();
        let end_result = search_path(path, |library| {
            results.push(library.to_vec());
            Result::<()>::Err(Error::OK)
        });
        assert!(matches!(end_result, None));
        assert_eq!(results, vec![b"A.B.dll\0".to_vec(), b"A.dll\0".to_vec()]);
    }
}
