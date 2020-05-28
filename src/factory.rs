use crate::runtime::*;
use crate::*;

// extern "system" fn(
//     RawComPtr<IActivationFactory>,
//     *mut <Object as RuntimeType>::Abi,
// ) -> ErrorCode;

// TODO: this should return `Result<&I>` e.g. a reference pointing to the factory cache.
// So this function needs to be implemented as some sort of atomic/singleton where RoGetActivationFactory
// is only called once and the result is then cached. Here's how I do it in C++ - it's critical
// that this is super fast. Also, load RoGetActivationFactory dynamically and fall back to LoadLibrary
// and implement DLL garbage collection for those. Version 0.1 can probably just pin everything.
// https://github.com/microsoft/cppwinrt/blob/master/strings/base_activation.h
pub fn factory<C: RuntimeName, I: ComInterface>() -> Result<I> {
    let mut factory = std::ptr::null_mut();
    unsafe {
        let mut code =
            RoGetActivationFactory(HString::from(C::NAME).abi(), &I::iid(), &mut factory);

        if code == ErrorCode::NOT_INITIALIZED {
            let mut _cookie = std::ptr::null_mut();
            CoIncrementMTAUsage(&mut _cookie);
            code = RoGetActivationFactory(HString::from(C::NAME).abi(), &I::iid(), &mut factory);
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
                .chain("dll".encode_utf16())
                .chain(std::iter::once(0))
                .collect();

            let library: Library = LoadLibraryW(path.as_ptr()).into();

            if library.handle.is_null() {
                continue;
            }
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
