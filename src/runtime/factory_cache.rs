use crate::*;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicPtr, Ordering};

type DllGetActivationFactory = extern "system" fn(name: RawPtr, factory: *mut RawPtr) -> ErrorCode;

/// Attempts to load and cache the factory interface for the given WinRT class. This is automatically
// used by the generated bindings and should not generally be used directly.
pub struct FactoryCache<C, I> {
    shared: AtomicPtr<std::ffi::c_void>,
    _c: PhantomData<C>,
    _i: PhantomData<I>,
}

impl<C, I> FactoryCache<C, I> {
    pub const fn new() -> Self {
        Self {
            shared: AtomicPtr::new(::std::ptr::null_mut()),
            _c: PhantomData,
            _i: PhantomData,
        }
    }
}

impl<C: RuntimeName, I: Interface> FactoryCache<C, I> {
    pub fn call<R, F: FnOnce(&I) -> Result<R>>(&mut self, callback: F) -> Result<R> {
        loop {
            // Attempt to load a previously cached factory pointer.
            let ptr = self.shared.load(Ordering::Relaxed);

            // If a pointer is found, the cache is primed and we're good to go.
            if !ptr.is_null() {
                return callback(unsafe { std::mem::transmute(&ptr) });
            }

            // Otherwise, we load the factory the usual way.
            let factory = factory::<C, I>()?;

            // If the factory is agile, we can safely cache it.
            if factory.cast::<IAgileObject>().is_ok() {
                if self
                    .shared
                    .compare_and_swap(
                        std::ptr::null_mut(),
                        unsafe { std::mem::transmute_copy(&factory) },
                        Ordering::Relaxed,
                    )
                    .is_null()
                {
                    std::mem::forget(factory);
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
    let name = HString::from(C::NAME);

    unsafe {
        // First attempt to get the activation factory via the OS.
        let code = RoGetActivationFactory(name.abi(), &I::IID, factory.set_abi());

        // Treat any delay-load errors like standard errors, so that the heuristics
        // below can still load registration-free libraries on Windows versions below 10.
        let mut code = code.unwrap_or_else(|code| code);

        // If this fails because combase hasn't been loaded yet then load combase
        // automatically so that it "just works" for apartment-agnostic code.
        if code == ErrorCode::CO_E_NOTINITIALIZED {
            let mut _cookie = std::ptr::null_mut();

            // Won't get any delay-load errors here if we got CO_E_NOTINITIALIZED, so quiet the
            // warning from the #[must_use] on the returned Result<>.
            let _ = CoIncrementMTAUsage(&mut _cookie);

            // Now try a second time to get the activation factory via the OS.
            code = RoGetActivationFactory(name.abi(), &I::IID, factory.set_abi())
                .unwrap_or_else(|code| code);
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
            let mut library = String::with_capacity(path.len() + 4);
            library.push_str(path);
            library.push_str(".dll");

            if let Ok(function) = delay_load(&library, "DllGetActivationFactory", 0) {
                let function: DllGetActivationFactory = std::mem::transmute(function);
                let mut abi = std::ptr::null_mut();
                function(name.abi(), &mut abi);

                if abi.is_null() {
                    continue;
                }

                let factory: IActivationFactory = std::mem::transmute(abi);
                return factory.cast();
            }
        }

        Err(original)
    }
}

demand_load! {
    "ole32.dll" {
        fn CoIncrementMTAUsage(cookie: *mut RawPtr) -> ErrorCode;
    }
    "combase.dll" {
        fn RoGetActivationFactory(hstring: RawPtr, interface: &Guid, result: *mut RawPtr) -> ErrorCode;
    }
}
