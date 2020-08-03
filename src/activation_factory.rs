use crate::*;

/// The [IActivationFactory](https://docs.microsoft.com/en-us/windows/win32/api/activation/nn-activation-iactivationfactory)
/// interface is the interface that WinRT activation factories implement at a minimum.
#[repr(transparent)]
#[derive(Default, Clone)]
pub struct IActivationFactory {
    ptr: ComPtr<IActivationFactory>,
}

impl IActivationFactory {
    /// Creates a new instance of the WinRT class that the activation factory represents.
    pub fn activate_instance<I: ComInterface>(&self) -> Result<I> {
        let ptr = self
            .get_abi()
            .expect("The `this` pointer was null when calling method");

        let mut object = Object::default();

        unsafe { (ptr.vtable().activate_instance)(ptr, object.set_abi()) }
            .and_then(|| object.query())
    }
}

unsafe impl ComInterface for IActivationFactory {
    type VTable = abi_IActivationFactory;

    const IID: Guid = {
        Guid::from_values(
            0x0000_0035,
            0x0000,
            0x0000,
            [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
        )
    };
}

unsafe impl AbiTransferable for IActivationFactory {
    type Abi = RawComPtr<Self>;

    fn get_abi(&self) -> Self::Abi {
        self.ptr.get_abi()
    }

    fn set_abi(&mut self) -> *mut Self::Abi {
        self.ptr.set_abi()
    }
}

#[repr(C)]
pub struct abi_IActivationFactory {
    base__: [usize; 6],
    activate_instance: unsafe extern "system" fn(
        NonNullRawComPtr<IActivationFactory>,
        *mut <Object as AbiTransferable>::Abi,
    ) -> ErrorCode,
}
