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
        Self { shared: AtomicPtr::new(::core::ptr::null_mut()), _c: PhantomData, _i: PhantomData }
    }
}

impl<C: RuntimeName, I: Interface> FactoryCache<C, I> {
    pub fn call<R, F: FnOnce(&I) -> Result<R>>(&mut self, callback: F) -> Result<R> {
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
                if self.shared.compare_exchange_weak(core::ptr::null_mut(), unsafe { core::mem::transmute_copy(&factory) }, Ordering::Relaxed, Ordering::Relaxed).is_ok() {
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

/// Attempts to load the factory interface for the given WinRT class.
pub fn factory<C: RuntimeName, I: Interface>() -> Result<I> {
    let mut factory: Option<I> = None;
    let name = HSTRING::from(C::NAME);

    unsafe {
        let function = delay_load(b"combase.dll\0", b"RoGetActivationFactory\0");

        let code = if !function.is_null() {
            let function: RoGetActivationFactory = core::mem::transmute(function);
            let mut code = function(core::mem::transmute_copy(&name), &I::IID, &mut factory as *mut _ as *mut _);

            // If RoGetActivationFactory fails because combase hasn't been loaded yet then load combase
            // automatically so that it "just works" for apartment-agnostic code.
            if code == CO_E_NOTINITIALIZED {
                let mta = delay_load(b"ole32.dll\0", b"CoIncrementMTAUsage\0");

                if !mta.is_null() {
                    let mta: CoIncrementMTAUsage = core::mem::transmute(mta);
                    let mut _cookie = core::ptr::null_mut();
                    let _ = mta(&mut _cookie);
                }

                // Now try a second time to get the activation factory via the OS.
                code = function(core::mem::transmute_copy(&name), &I::IID, &mut factory as *mut _ as *mut _);
            }

            code
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
        let mut path = C::NAME;

        // Remove the suffix until a match is found. For example, if the class name is
        // "A.B.TypeName" then this will first attempt to load "A.B.dll" and if that
        // fails it will attempt "A.dll" before giving up.
        while let Some(pos) = path.rfind('.') {
            path = &path[..pos];

            let library = core::slice::from_raw_parts_mut(heap_alloc(path.len() + 5)? as *mut u8, path.len() + 5);
            library[..path.len()].copy_from_slice(path.as_bytes());
            library[path.len()..].copy_from_slice(b".dll\0");

            let function = delay_load(library, b"DllGetActivationFactory\0");

            if !function.is_null() {
                let function: DllGetActivationFactory = core::mem::transmute(function);
                let mut abi = core::ptr::null_mut();
                let _ = function(core::mem::transmute_copy(&name), &mut abi);

                if abi.is_null() {
                    continue;
                }

                let factory: IActivationFactory = core::mem::transmute(abi);
                return factory.cast();
            }
        }

        Err(original)
    }
}

type CoIncrementMTAUsage = extern "system" fn(cookie: *mut RawPtr) -> HRESULT;
type RoGetActivationFactory = extern "system" fn(hstring: core::mem::ManuallyDrop<HSTRING>, interface: &GUID, result: *mut RawPtr) -> HRESULT;
type DllGetActivationFactory = extern "system" fn(name: core::mem::ManuallyDrop<HSTRING>, factory: *mut RawPtr) -> HRESULT;
