use crate::*;

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
            ((*(*(self.ptr.as_raw()))).activate_instance)(self.ptr.as_raw(), object.set_abi())
                .and_then(|| object.query())
        }
    }
}

unsafe impl ComInterface for IActivationFactory {
    type VTable = abi_IActivationFactory;

    fn iid() -> Guid {
        Guid::from_values(
            0x0000_0035,
            0x0000,
            0x0000,
            [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
        )
    }
}

#[repr(C)]
pub struct abi_IActivationFactory {
    __base: [usize; 6],
    activate_instance: extern "system" fn(
        RawComPtr<IActivationFactory>,
        *mut <Object as RuntimeType>::Abi,
    ) -> ErrorCode,
}
