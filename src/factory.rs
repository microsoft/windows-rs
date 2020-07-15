use crate::*;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicPtr, Ordering};

pub struct FactoryCache<C: RuntimeName, I: ComInterface> {
    pub shared: AtomicPtr<std::ffi::c_void>,
    pub _c: PhantomData<C>,
    pub _i: PhantomData<I>,
}

impl<C: RuntimeName, I: ComInterface> FactoryCache<C, I> {
    pub fn new() -> Self {
        Self {
            shared: AtomicPtr::new(std::ptr::null_mut()),
            _c: PhantomData,
            _i: PhantomData,
        }
    }
    
    pub fn call<R, F: FnOnce(&I) -> Result<R>>(&mut self, callback: F) -> Result<R> {
        loop {
            unsafe {
                let mut ptr = self.shared.load(Ordering::Relaxed);

                if !ptr.is_null() {
                    return callback(std::mem::transmute(&ptr));
                }

                let name = HString::from(C::NAME);
                
                let mut code = RoGetActivationFactory(name.get_abi(), &I::iid(), &mut ptr)
                    .unwrap_or_else(|code| code);

                if code == NOT_INITIALIZED {
                    let mut cookie = std::ptr::null_mut();
                    let _ = CoIncrementMTAUsage(&mut cookie);

                    code = RoGetActivationFactory(name.get_abi(), &I::iid(), &mut ptr)
                        .unwrap_or_else(|code| code);
                }

                if !code.is_ok() {
                    let original: Error = code.into();
                    let mut path = C::NAME;

                    while let Some(pos) = path.rfind('.') {
                        path = &path[..pos];

                        let path: Vec<u16> =
                            path.encode_utf16().chain(".dll\0".encode_utf16()).collect();

                        let library = Library::from_handle(LoadLibraryExW(
                            path.as_ptr(),
                            std::ptr::null_mut(),
                            0,
                        ));

                        if library.handle.is_null() {
                            continue;
                        }

                        let library_call =
                            GetProcAddress(library.handle, b"DllGetActivationFactory\0".as_ptr());

                        if library_call.is_null() {
                            continue;
                        }

                        let library_call: DllGetActivationFactory =
                            std::mem::transmute(library_call);

                        let mut factory: IActivationFactory = std::mem::zeroed();

                        if library_call(name.get_abi() as _, factory.set_abi()).is_err() {
                            continue;
                        }

                        std::mem::forget(library);
                        let factory = factory.as_iunknown().unwrap();
                        (factory.vtable().unknown_query_interface)(factory, &I::iid(), &mut ptr);
                    }

                    if ptr.is_null() {
                        return Err(original);
                    }
                }

                let factory: I = std::mem::transmute_copy(&ptr);

                if factory.is_agile() {
                    if self
                        .shared
                        .compare_and_swap(std::ptr::null_mut(), ptr, Ordering::Relaxed)
                        .is_null()
                    {
                        std::mem::forget(factory);
                    } else {
                        return callback(&factory);
                    }
                }
            }
        }
    }
}

// Indicates that COM has not been initialized.
const NOT_INITIALIZED: ErrorCode = ErrorCode(0x8004_01F0);

/// Attempts to load the factory interface for the given WinRT class.
///
/// Note that factory caching still needs to be implemented.
pub fn factory<C: RuntimeName, I: ComInterface + Default>() -> Result<I> {
    let mut factory = I::default();
    let name = HString::from(C::NAME);

    unsafe {
        // First attempt to get the activation factory via the OS.
        let code = RoGetActivationFactory(name.get_abi(), &I::iid(), factory.set_abi() as _);

        // Treat any delay-load errors like standard errors, so that the heuristics
        // below can still load registration-free libraries on Windows versions below 10.
        let mut code = code.unwrap_or_else(|code| code);

        // If this fails because combase hasn't been loaded yet then load combase
        // automatically so that it "just works" for apartment-agnostic code.
        if code == NOT_INITIALIZED {
            let mut _cookie = std::ptr::null_mut();

            // Won't get any delay-load errors here if we got NOT_INITIALIZED, so quiet the
            // warning from the #[must_use] on the returned Result<>.
            let _ = CoIncrementMTAUsage(&mut _cookie);

            // Now try a second time to get the activation factory via the OS.
            code = RoGetActivationFactory(name.get_abi(), &I::iid(), factory.set_abi() as _)
                .unwrap_or_else(|code| code);
        }

        // If this succeeded then return the resulting factory interface.
        if code.is_ok() {
            return Ok(factory);
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

            // Turn the resulting namespace portion into a DLL name.
            let path: Vec<u16> = path
                .encode_utf16()
                .chain(".dll\0".encode_utf16())
                .collect();

            // Attempt to load the DLL.
            let library =
                Library::from_handle(LoadLibraryExW(path.as_ptr(), std::ptr::null_mut(), 0));

            if library.handle.is_null() {
                continue;
            }

            // If the DLL was found then get the export used to retrieve the factory.
            let library_call =
                GetProcAddress(library.handle, b"DllGetActivationFactory\0".as_ptr());

            if library_call.is_null() {
                continue;
            }

            let library_call: DllGetActivationFactory = std::mem::transmute(library_call);
            let mut factory: IActivationFactory = std::mem::zeroed();

            // Now call DllGetActivationFactory to request the given class.
            if library_call(name.get_abi(), factory.set_abi()).is_err() {
                continue;
            }

            debug_assert!(!factory.is_null());

            // If we get this far it means the factory has been loaded and will be returned
            // to the caller. At this point we need to pin the library to avoid it unloading
            // while there are outstanding references. Unloading is only supported for
            // components loaded via RoGetActivationFactory.
            std::mem::forget(library);
            return Ok(factory.query());
        }

        Err(original)
    }
}

type DllGetActivationFactory = extern "system" fn(
    name: *mut hstring::Header,
    factory: *mut RawComPtr<IActivationFactory>,
) -> ErrorCode;

struct Library {
    handle: RawPtr,
}

impl Drop for Library {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe {
                FreeLibrary(self.handle);
            }
        }
    }
}

impl Library {
    unsafe fn from_handle(handle: RawPtr) -> Self {
        Library { handle }
    }
}
