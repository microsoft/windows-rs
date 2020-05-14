use crate::runtime;
use crate::*;

// TODO: this should return `Result<&I>` e.g. a reference pointing to the factory cache.
// So this function needs to be implemented as some sort of atomic/singleton where RoGetActivationFactory
// is only called once and the result is then cached. Here's how I do it in C++ - it's critical
// that this is super fast. Also, load RoGetActivationFactory dynamically and fall back to LoadLibrary
// and implement DLL garbage collection for those. Version 0.1 can probably just pin everything.
// https://github.com/microsoft/cppwinrt/blob/master/strings/base_activation.h
pub fn factory<C: RuntimeName, I: ComInterface>() -> Result<I> {
    let mut ptr = std::ptr::null_mut();
    unsafe {
        let mut code =
            runtime::RoGetActivationFactory(HString::from(C::NAME).abi(), &I::iid(), &mut ptr);

        if code == ErrorCode::NOT_INITIALIZED {
            let mut _cookie = std::ptr::null_mut();
            runtime::CoIncrementMTAUsage(&mut _cookie);

            code =
                runtime::RoGetActivationFactory(HString::from(C::NAME).abi(), &I::iid(), &mut ptr);
        }

        code.and_then(|| std::mem::transmute_copy(&ptr))
    }
}
