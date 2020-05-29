use crate::runtime::*;
use crate::*;

type DllGetActivationFactory = extern "system" fn(
        name: *mut hstring::Header,
        factory: *mut RawComPtr<IActivationFactory>,
    ) -> ErrorCode;

pub fn factory<C: RuntimeName, I: ComInterface>() -> Result<I> {
    let mut factory = std::ptr::null_mut();
    let name = HString::from(C::NAME);
    unsafe {
        let mut code =
            RoGetActivationFactory(name.abi(), &I::iid(), &mut factory);

        if code == ErrorCode::NOT_INITIALIZED {
            let mut _cookie = std::ptr::null_mut();
            CoIncrementMTAUsage(&mut _cookie);
            code = RoGetActivationFactory(name.abi(), &I::iid(), &mut factory);
        }

        if code.is_ok() {
            return Ok(std::mem::transmute_copy(&factory));
        }

        let original: Error = code.into();
        let mut path = C::NAME;

        while let Some(pos) = path.rfind('.') {
            path = &path[..pos];

            let path: Vec<u16> = path
                .encode_utf16()
                .chain(".dll".encode_utf16())
                .chain(std::iter::once(0))
                .collect();

            let library: Library = LoadLibraryW(path.as_ptr()).into();

            if library.handle.is_null() {
                continue;
            }

            let library_call = GetProcAddress(library.handle, b"DllGetActivationFactory\0".as_ptr());

            if library_call.is_null() {
                continue;
            }

            let library_call: DllGetActivationFactory = std::mem::transmute(library_call);
            let mut default_factory: IActivationFactory = std::mem::zeroed();

            if library_call(name.abi(), default_factory.set_abi()).is_err() {
                continue;
            }

            std::mem::forget(library);
            return Ok(default_factory.query());
        }

        Err(original)
    }
}

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

impl From<RawPtr> for Library {
    fn from(handle: RawPtr) -> Library {
        Library { handle }
    }
}
