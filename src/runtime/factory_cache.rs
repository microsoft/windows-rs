use super::*;
use bindings::{Windows::Win32::Foundation::CLASS_E_CLASSNOTAVAILABLE, Windows::Win32::Graphics::DirectDraw::CO_E_NOTINITIALIZED};
use core::marker::PhantomData;
use core::sync::atomic::{AtomicPtr, Ordering};

type DllGetActivationFactory = extern "system" fn(name: core::mem::ManuallyDrop<HSTRING>, factory: *mut RawPtr) -> HRESULT;

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
        // First attempt to get the activation factory via the OS.
        let code = RoGetActivationFactory(core::mem::transmute_copy(&name), &I::IID, &mut factory as *mut _ as *mut _);

        // Treat any delay-load errors like standard errors, so that the heuristics
        // below can still load registration-free libraries on Windows versions below 10.
        let mut code = code.unwrap_or_else(|| CLASS_E_CLASSNOTAVAILABLE);

        // If this fails because combase hasn't been loaded yet then load combase
        // automatically so that it "just works" for apartment-agnostic code.
        if code == HRESULT(CO_E_NOTINITIALIZED as _) {
            let mut _cookie = core::ptr::null_mut();

            // Won't get any delay-load errors here if we got CO_E_NOTINITIALIZED, so quiet the
            // warning from the #[must_use] on the returned Result<>.
            let _ = CoIncrementMTAUsage(&mut _cookie);

            // Now try a second time to get the activation factory via the OS.
            code = RoGetActivationFactory(core::mem::transmute_copy(&name), &I::IID, &mut factory as *mut _ as *mut _).unwrap();
        }

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

            let library = core::slice::from_raw_parts_mut(heap_alloc(path.len() + 4)? as *mut u8, path.len() + 4);
            library.copy_from_slice(path.as_bytes());
            library[path.len()..].copy_from_slice(b".dll");

            let function = delay_load(core::str::from_utf8_unchecked(library), "DllGetActivationFactory");

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

demand_load! {
    "ole32.dll" {
        fn CoIncrementMTAUsage(cookie: *mut RawPtr) -> HRESULT;
    }
    "combase.dll" {
        fn RoGetActivationFactory(hstring: core::mem::ManuallyDrop<HSTRING>, interface: &GUID, result: *mut RawPtr) -> HRESULT;
    }
}
