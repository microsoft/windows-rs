use crate::*;

/// Attempts to load the factory interface for the given WinRT class.
///
/// Note that factory caching still needs to be implemented.
pub fn factory<C: RuntimeName, I: ComInterface>() -> Result<I> {
    let mut factory = std::ptr::null_mut();
    let name = HString::from(C::NAME);

    unsafe {
        // First attempt to get the activation factory via the OS.
        let code = RoGetActivationFactory(name.get_abi(), &I::iid(), &mut factory);

        // Treat any delay-load errors like standard errors, so that the heuristics
        // below can still load registration-free libraries on Windows versions below 10.
        let mut code = code.unwrap_or_else(|code| code);

        // If this fails because combase hasn't been loaded yet then load combase
        // automatically so that it "just works" for apartment-agnostic code.
        if code == ErrorCode::NOT_INITIALIZED {
            let mut _cookie = std::ptr::null_mut();

            // Won't get any delay-load errors here if we got NOT_INITIALIZED, so quiet the
            // warning from the #[must_use] on the returned Result<>.
            let _ = CoIncrementMTAUsage(&mut _cookie);

            // Now try a second time to get the activation factory via the OS.
            code = RoGetActivationFactory(name.get_abi(), &I::iid(), &mut factory)
                .unwrap_or_else(|code| code);
        }

        // If this succeeded then return the resulting factory interface.
        if code.is_ok() {
            return Ok(std::mem::transmute_copy(&factory));
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
                .chain(".dll".encode_utf16())
                .chain(std::iter::once(0))
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
