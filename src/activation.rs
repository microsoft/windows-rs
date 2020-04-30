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
            runtime::RoGetActivationFactory(HString::from(C::NAME).abi(), &I::GUID, &mut ptr);

        if code == ErrorCode::NOT_INITIALIZED {
            let mut _cookie = std::ptr::null_mut();
            runtime::CoIncrementMTAUsage(&mut _cookie);

            code =
                runtime::RoGetActivationFactory(HString::from(C::NAME).abi(), &I::GUID, &mut ptr);
        }

        code.and_then(|| std::mem::transmute_copy(&ptr))
    }
}

/// An [activation factory](https://docs.microsoft.com/en-us/windows/win32/api/activation/nn-activation-iactivationfactory) for activating WinRT types.
#[repr(transparent)]
#[derive(Default, Clone)]
pub struct IActivationFactory {
    ptr: ComPtr<IActivationFactory>,
}

impl IActivationFactory {
    pub fn activate_instance<I: ComInterface>(&self) -> Result<I> {
        if self.ptr.is_null() {
            panic!("The `this` pointer was null when calling method");
        }

        let mut object = Object::default();
        unsafe {
            ((*(*(self.ptr.abi()))).activate_instance)(self.ptr.abi(), object.set_abi())
                .and_then(|| object.query())
        }
    }
}

unsafe impl ComInterface for IActivationFactory {
    type VTable = abi_IActivationFactory;
    const GUID: Guid = Guid::from_values(
        0x0000_0035,
        0x0000,
        0x0000,
        [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
    );
}

#[repr(C)]
pub struct abi_IActivationFactory {
    __base: [usize; 6],
    activate_instance: extern "system" fn(
        *const *const activation::abi_IActivationFactory,
        *mut <Object as RuntimeType>::Abi,
    ) -> ErrorCode,
}
